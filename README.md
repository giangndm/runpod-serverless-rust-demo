# Runpod WebSocket Echo (Rust + Axum)

Minimal `axum` app for a Runpod **Load Balancer** endpoint.

## Endpoints

- `GET /ping` -> `ok` (health check)
- `GET /ws` -> WebSocket echo (text/binary)
- `GET /` -> simple browser demo page

## Run locally

```bash
cargo run
```

The app listens on `PORT` (default `8080` locally).

Open:

- `http://localhost:8080/ping`
- `http://localhost:8080/`

## Docker

```bash
docker build -t your-user/runpod-ws-echo:latest .
docker run --rm -p 8080:80 your-user/runpod-ws-echo:latest
```

## Use `runpod-sdk` in this project

This repo also includes `runpod-sdk` for account/endpoint API operations.

Set your API key:

```bash
export RUNPOD_API_KEY="YOUR_API_KEY"
```

List endpoints:

```bash
cargo run --bin runpod_endpoints
# alias:
cargo run --bin runpod_endpoint
```

Note: `runpod-sdk` is an API client (manage endpoints, queue jobs, status). The WebSocket echo service itself is still served directly by this `axum` server on a Load Balancer endpoint.

## Deploy on Runpod

1. Push image to a registry (`docker push ...`).
2. Create Serverless endpoint type **Load Balancer**.
3. Set visibility to **Public**.
4. Use your image and container port `80`.

## Test after deploy

Replace `ENDPOINT_ID`:

```bash
curl https://ENDPOINT_ID.api.runpod.ai/ping
```

Open in browser:

```text
https://ENDPOINT_ID.api.runpod.ai/
```

The page connects to:

```text
wss://ENDPOINT_ID.api.runpod.ai/ws
```
