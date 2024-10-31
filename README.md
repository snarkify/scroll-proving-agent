# Scroll Proving Agent

## Introduction

The Scroll proving agent is a high-performance service client for efficient block proving for layer-2 rollups
based on the [Scroll SDK](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/). With this integration, Scroll SDK chains
gain access to robust GPU-proving capabilities, allowing for the fast and effective proving of blockspaces.

Snarkify specializes in ZK GPU acceleration and is a multi-time winner of the [ZPRIZE](https://www.zprize.io/blog/announcing-the-2023-zprize-winners)
ZK acceleration competition. Snarkify is the only provider that has developed an in-house GPU prover that is 1.25x more
efficient than the official GPU provers. With the in-house GPU proving capability, Snarkify offers best-in-class GPU
proving services for both Scroll Mainnet and SDK chains. On a daily basis, Snarkify serves 25% to 50% of the Scroll
Mainnet proving jobs.

## How It Works

The Scroll Proving agent integrates with the [Scroll Proving SDK](https://github.com/scroll-tech/scroll-proving-sdk) 
to handle block proving in three main steps:

1.	Task Retrieval: The agent connects to the coordinator endpoint to retrieve proving tasks and to the Geth endpoint 
for block trace data required for chunk tasks.
2.	Proof Generation: Each task is dispatched to the Snarkify platform, where proofs are generated.
3.	Task Completion: The agent periodically checks the task’s status on the Snarkify platform and, once complete, 
submits the proof back to the coordinator.

## Getting Started

### Install Helm Chart
To deploy the agent, you’ll need Helm. Install Helm with:
```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

### Create an API Key
You’ll need an API key to authenticate. Follow the steps [here](https://docs.snarkify.io/gpu-prover-network/deploy-a-elastic-proving-service#create-an-api-key) to obtain one.

### Set Up Scroll Proving Service
Set up a Scroll Proving service and obtain its service ID. Please contact us on [Telegram](https://t.me/+WRhRjNL6zixjNWUx) 
if you want to set up a service.

### Prepare the Configuration
There are 3 types of block proving in Scroll SDK: Chunk, Batch and Bundle, so you need to create 3 different 
configuration files `chunk-config.yaml`, `batch-config.yaml` and `bundle-config.yaml`.
Here is the example of `chunk-config.yaml`, replacing placeholders as needed:
```yaml
scrollConfig: |
  {
    "prover_name_prefix": "<Your-Prover-Name-Prefix>",
    "keys_dir": "/keys",
    "coordinator": {
      "base_url": "<Your-Coordinator-Endpoint>",
      "retry_count": 3,
      "retry_wait_time_sec": 5,
      "connection_timeout_sec": 60
    },
    "l2geth": {
      "endpoint": "<Your-Geth-Endpoint>"
    },
    "prover": {
      "circuit_type": 1,
      "circuit_version": "v0.13.1",
      "n_workers": <Your-Number-Of-Workers>,
      "cloud": {
        "base_url": "https://api.snarkify.io",
        "api_key": "<Your-API-Key>",
        "retry_count": 3,
        "retry_wait_time_sec": 5,
        "connection_timeout_sec": 60
      }
    }
  }

env:
  serviceId: "<Your-Service-ID>"
```

- prover_name_prefix: A prefix for your prover name, should end with an underscore.
- coordinator.base_url: Your coordinator endpoint.
- l2geth.endpoint: Your Geth endpoint.
- prover.circuit_type: The circuit type, can be 1 (Chunk), 2 (Batch) or 3 (Bundle).
- prover.n_workers: The number of workers to run in parallel.
- cloud.api_key: Your Snarkify API key.
- serviceId: Your Scroll Proving service ID in Snarkify platform.


### Deploy Scroll Proving Agent

Deploy the agent using Helm with your configuration files.
```bash
export HELM_EXPERIMENTAL_OCI=1
helm install scroll-proving-agent-chunk oci://ghcr.io/snarkify/scroll-proving-agent/helm/scroll-proving-agent --version 0.0.1 -f chunk-config.yaml

helm install scroll-proving-agent-batch oci://ghcr.io/snarkify/scroll-proving-agent/helm/scroll-proving-agent --version 0.0.1 -f batch-config.yaml

helm install scroll-proving-agent-bundle oci://ghcr.io/snarkify/scroll-proving-agent/helm/scroll-proving-agent --version 0.0.1 -f bundle-config.yaml
```

### Verify the Deployment
You can verify the deployment by checking the pods and the logs.
```bash
kubectl get pods
kubectl logs -l app.kubernetes.io/name=scroll-proving-agent
```

### Uninstall the Helm Chart
```bash
helm uninstall scroll-proving-agent-chunk
helm uninstall scroll-proving-agent-batch
helm uninstall scroll-proving-agent-bundle
```

### Community
Please connect with the team on [Telegram](https://t.me/+WRhRjNL6zixjNWUx) to learn more.
