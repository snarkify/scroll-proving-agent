use crate::config::SnarkifyConfig;
use crate::types::{
    SnarkifyCreateTaskInput, SnarkifyCreateTaskRequest, SnarkifyGetTaskResponse,
    SnarkifyGetVkResponse,
};
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use core::time::Duration;
use reqwest::{header::CONTENT_TYPE, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use scroll_proving_sdk::prover::{
    proving_service::{
        GetVkRequest, GetVkResponse, ProveRequest, ProveResponse, QueryTaskRequest,
        QueryTaskResponse, TaskStatus,
    },
    types::ProofType,
    ProvingService,
};
use serde::Serialize;
use tracing::{debug, error, info};

/// API version used by the Snarkify platform.
const API_VERSION: &str = "v1";

pub struct SnarkifyProver {
    base_url: String,
    api_key: String,
    service_id: String,
    connection_timeout_sec: Duration,
    client: ClientWithMiddleware,
}

#[async_trait]
impl ProvingService for SnarkifyProver {
    fn is_local(&self) -> bool {
        false
    }

    async fn get_vks(&self, req: GetVkRequest) -> GetVkResponse {
        if req.proof_types.is_empty() {
            error!("[Snarkify Client][get_vks] proof types are empty");
            return GetVkResponse {
                vks: vec![],
                error: Some("Proof types are empty".to_string()),
            };
        }
        if req.proof_types.len() > 1 {
            error!("[Snarkify Client][get_vks] proof types are more than one");
            return GetVkResponse {
                vks: vec![],
                error: Some("Proof types are more than one".to_string()),
            };
        }

        let method = format!(
            "/{}/scroll/sdk/vks/versions/{}/types/{}",
            API_VERSION,
            &req.circuit_version,
            &req.proof_types[0].to_u8()
        );
        match self.get::<SnarkifyGetVkResponse>(&method).await {
            Ok(resp) => GetVkResponse {
                vks: resp.vks,
                error: None,
            },
            Err(e) => {
                error!("[Snarkify Client][get_vks] Failed to get vks: {:?}", e);
                GetVkResponse {
                    vks: vec![],
                    error: Some(format!("Failed to get vks: {}", e)),
                }
            }
        }
    }

    async fn prove(&mut self, req: ProveRequest) -> ProveResponse {
        // Send input to S3 directly.
        let body = match SnarkifyCreateTaskRequest::from_prove_request(&req) {
            Ok(body) => body,
            Err(e) => {
                error!(
                    "[Snarkify Client][prove] Failed to create task request: {:?}",
                    e
                );
                return self.build_prove_error_response(
                    &req,
                    &format!("Failed to create task request: {}", e),
                );
            }
        };
        let method = format!("/{}/services/{}", API_VERSION, &self.service_id);

        match self
            .post::<SnarkifyCreateTaskRequest, SnarkifyGetTaskResponse>(&method, &body)
            .await
        {
            Ok(resp) => ProveResponse {
                task_id: resp.task_id,
                proof_type: req.proof_type,
                circuit_version: req.circuit_version,
                hard_fork_name: req.hard_fork_name,
                status: resp.state.into(),
                created_at: resp.created.map(|t| t.timestamp() as f64).unwrap_or(0.0),
                started_at: resp.started.map(|t| t.timestamp() as f64),
                finished_at: None,
                compute_time_sec: None,
                input: Some(req.input.clone()),
                proof: None,
                vk: None,
                error: None,
            },
            Err(e) => {
                error!("[Snarkify Client][prove] Failed to request proof: {:?}", e);
                self.build_prove_error_response(&req, &format!("Failed to request proof: {}", e))
            }
        }
    }

    async fn query_task(&mut self, req: QueryTaskRequest) -> QueryTaskResponse {
        // TODO: Query task result from S3 directly.
        let method = format!("/{}/tasks/{}", API_VERSION, &req.task_id);
        match self.get::<SnarkifyGetTaskResponse>(&method).await {
            Ok(resp) => {
                let task_input: SnarkifyCreateTaskInput = match serde_json::from_str(&resp.input) {
                    Ok(input) => input,
                    Err(e) => {
                        return self.build_query_task_error_response(
                            &req,
                            &format!("Failed to parse task input: {}", e),
                        )
                    }
                };
                let started_at = resp.started.map(|t| t.timestamp() as f64);
                let finished_at = resp.finished.map(|t| t.timestamp() as f64);
                let compute_time_sec = match (started_at, finished_at) {
                    (Some(started), Some(finished)) => Some(finished - started),
                    _ => None,
                };
                QueryTaskResponse {
                    task_id: resp.task_id,
                    proof_type: resp.proof_type.into(),
                    circuit_version: task_input.circuit_version,
                    hard_fork_name: task_input.hard_fork_name,
                    status: resp.state.into(),
                    created_at: resp.created.map(|t| t.timestamp() as f64).unwrap_or(0.0),
                    started_at,
                    finished_at,
                    compute_time_sec,
                    input: Some(task_input.task_data),
                    proof: resp.proof,
                    vk: None,
                    error: resp.error,
                }
            }
            Err(e) => {
                error!(
                    "[Snarkify Client][query_task] Failed to query proof: {:?}",
                    e
                );
                self.build_query_task_error_response(&req, &format!("Failed to query proof: {}", e))
            }
        }
    }
}

impl SnarkifyProver {
    pub fn new(config: SnarkifyConfig) -> Self {
        let retry_wait_duration =
            Duration::from_secs(config.sdk_config.coordinator.retry_wait_time_sec);
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(retry_wait_duration / 2, retry_wait_duration)
            .build_with_max_retries(config.sdk_config.coordinator.retry_count);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            base_url: config.base_url,
            api_key: config.api_key,
            service_id: config.service_id,
            connection_timeout_sec: Duration::from_secs(
                config.sdk_config.coordinator.connection_timeout_sec,
            ),
            client,
        }
    }

    pub fn build_prove_error_response(&self, req: &ProveRequest, error_msg: &str) -> ProveResponse {
        ProveResponse {
            task_id: String::new(),
            proof_type: req.proof_type,
            circuit_version: req.circuit_version.clone(),
            hard_fork_name: req.hard_fork_name.clone(),
            status: TaskStatus::Failed,
            created_at: 0.0,
            started_at: None,
            finished_at: None,
            compute_time_sec: None,
            input: Some(req.input.clone()),
            proof: None,
            vk: None,
            error: Some(error_msg.to_string()),
        }
    }

    pub fn build_query_task_error_response(
        &self,
        req: &QueryTaskRequest,
        error_msg: &str,
    ) -> QueryTaskResponse {
        QueryTaskResponse {
            task_id: req.task_id.clone(),
            proof_type: ProofType::Undefined,
            circuit_version: "".to_string(),
            hard_fork_name: "".to_string(),
            status: TaskStatus::Failed,
            created_at: 0.0,
            started_at: None,
            finished_at: None,
            compute_time_sec: None,
            input: None,
            proof: None,
            vk: None,
            error: Some(error_msg.to_string()),
        }
    }

    fn build_url(&self, method: &str) -> Result<Url> {
        let full_url = format!("{}{}", self.base_url, method);
        Url::parse(&full_url).map_err(|e| anyhow!("Failed to parse URL '{}': {}", full_url, e))
    }

    async fn get<Resp>(&self, method: &str) -> Result<Resp>
    where
        Resp: serde::de::DeserializeOwned,
    {
        let url = self.build_url(method)?;
        info!("[Snarkify Client], {method}, sent request");
        let response = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .header("X-Api-Key", &self.api_key)
            .timeout(self.connection_timeout_sec)
            .send()
            .await?;

        let status = response.status();
        if !(status >= http::status::StatusCode::OK && status <= http::status::StatusCode::ACCEPTED)
        {
            bail!("[Snarkify Client], {method}, status not ok: {}", status)
        }

        let response_body = response.text().await?;

        info!("[Snarkify Client], {method}, received response");
        debug!("[Snarkify Client], {method}, response: {response_body}");
        serde_json::from_str(&response_body).map_err(|e| anyhow!(e))
    }

    async fn post<Req, Resp>(&self, method: &str, req: &Req) -> Result<Resp>
    where
        Req: ?Sized + Serialize,
        Resp: serde::de::DeserializeOwned,
    {
        let url = self.build_url(method)?;
        let request_body = serde_json::to_string(req)?;
        info!("[Snarkify Client], {method}, sent request");
        debug!("[Snarkify Client], {method}, request: {request_body}");
        let response = self
            .client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .header("X-Api-Key", &self.api_key)
            .body(request_body)
            .timeout(self.connection_timeout_sec)
            .send()
            .await?;

        let status = response.status();
        if !(status >= http::status::StatusCode::OK && status <= http::status::StatusCode::ACCEPTED)
        {
            bail!("[Snarkify Client], {method}, status not ok: {}", status)
        }

        let response_body = response.text().await?;

        info!("[Snarkify Client], {method}, received response");
        debug!("[Snarkify Client], {method}, response: {response_body}");
        serde_json::from_str(&response_body).map_err(|e| anyhow!(e))
    }
}
