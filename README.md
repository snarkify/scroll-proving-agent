# Snarkify Scroll Proving

## Introduction

The Snarkify Scroll Proving is a high-performance binary built on [Snarkify](https://snarkify.io/), specifically
designed for efficient block proving within the [Scroll SDK](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/). With this
integration, Scroll SDK clients gain access to robust GPU-proving capabilities, allowing for the fast and effective 
proving of any data or operations within their SDK chain’s blockspace. This supports their goals of decentralization and 
enables faster transaction settlement.

Snarkify is the multiple winners of the [ZPRIZE](https://www.zprize.io/blog/announcing-the-2023-zprize-winners) and 
specilizes in the zero-knowledge GPU acceleration. It has in-house GPU provers that are 25% more efficient than the 
official Scroll GPUprovers. Leveraging Snarkify’s expertise in zero-knowledge proofs and GPU acceleration, SDK clients 
can enjoy a cost-effective, end-to-end infrastructure for best-in-class GPU proving, aiming to empower innovative 
solutions and enable new use cases.

## How It Works

The Snarkify Scroll Proving binary integrates with the [Scroll Proving SDK](https://github.com/scroll-tech/scroll-proving-sdk) 
to handle block proving in three main steps:

1.	Task Retrieval: The binary connects to the coordinator endpoint to retrieve proving tasks and to the Geth endpoint for block trace data required for chunk tasks.
2.	Proof Generation: Each task is dispatched to the Snarkify platform, where proofs are generated.
3.	Task Completion: The binary periodically checks the task’s status on the Snarkify platform and, once complete, submits the proof back to the coordinator.

## Getting Started

### Install Helm Chart
To deploy the binary, you’ll need Helm. Install Helm with:
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
- serviceId: Your Scroll Proving service ID.


### Deploy Snarkify Scroll Proving

Deploy the prover using Helm with your configuration files.
```bash
export HELM_EXPERIMENTAL_OCI=1
helm install snarkify-scroll-proving-chunk oci://ghcr.io/snarkify/snarkify-scroll-proving/helm/snarkify-scroll-proving --version 0.0.1 -f chunk-config.yaml

helm install snarkify-scroll-proving-batch oci://ghcr.io/snarkify/snarkify-scroll-proving/helm/snarkify-scroll-proving --version 0.0.1 -f batch-config.yaml

helm install snarkify-scroll-proving-bundle oci://ghcr.io/snarkify/snarkify-scroll-proving/helm/snarkify-scroll-proving --version 0.0.1 -f bundle-config.yaml
```

### Verify the Deployment
You can verify the deployment by checking the pods and the logs.
```bash
kubectl get pods
kubectl logs -l app.kubernetes.io/name=snarkify-scroll-proving
```

### Uninstall the Helm Chart
```bash
helm uninstall snarkify-scroll-proving-chunk
helm uninstall snarkify-scroll-proving-batch
helm uninstall snarkify-scroll-proving-bundle
```

### Pricing
Snarkify offers affordable proving services with in-house high-performance GPU provers. Please contact us on 
[Telegram](https://t.me/+WRhRjNL6zixjNWUx) for more details.
