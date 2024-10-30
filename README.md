# Snarkify Scroll Proving SDK

## Introduction

The Snarkify Scroll Proving SDK is a high-performance binary, built on [Snarkify](https://snarkify.io/), designed for efficient block proving within the [Scroll SDK](https://scroll-sdk-init.docs.scroll.xyz/en/sdk/). Leveraging Snarkify’s expertise in zero-knowledge GPU acceleration, this SDK provides an easy-to-use, cost-effective solution for block proving.

## How It Works

The Snarkify Scroll Proving SDK integrates with the [Scroll Proving SDK](https://github.com/scroll-tech/scroll-proving-sdk) to handle block proving in three main steps:

	1.	Task Retrieval: The SDK connects to the coordinator endpoint to retrieve proving tasks and to the Geth endpoint for block trace data required for chunk tasks.
	2.	Proof Generation: Each task is dispatched to the Snarkify platform, where proofs are generated.
	3.	Task Completion: The SDK periodically checks the task’s status on the Snarkify platform and, once complete, submits the proof back to the coordinator.

## Getting Started

### Install Helm Chart
To deploy the SDK, you’ll need Helm. Install Helm with:
```bash
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

### Create an API Key
You’ll need an API key to authenticate. Follow the steps [here](https://docs.snarkify.io/gpu-prover-network/deploy-a-elastic-proving-service#create-an-api-key) to obtain one.

### Set Up Scroll SDK Service
Set up a Scroll SDK service and obtain its service ID. If you need assistance, contact us on [Telegram](https://t.me/+WRhRjNL6zixjNWUx).

### Deploy the Proving SDK
After obtaining your service ID, deploy the prover using Helm. (Note: Deployment details will be updated once the Helm chart is published.)

### Configuration
Configure the SDK using the template below, replacing placeholders as needed:
```json
{
    "prover_name_prefix": "snarkify_",
    "keys_dir": "keys",
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
        "n_workers": 1,
        "cloud": {
            "base_url": "https://api.snarkify.io",
            "api_key": "<Your-API-Key>",
            "retry_count": 3,
            "retry_wait_time_sec": 5,
            "connection_timeout_sec": 60
        }
    }
}
```