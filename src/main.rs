use std::{env, net::SocketAddr};

use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/ws", get(ws_handler));

    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn ping() -> &'static str {
    "ok"
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(result) = socket.recv().await {
        let message = match result {
            Ok(message) => message,
            Err(_) => break,
        };

        match message {
            echoed @ (Message::Text(_) | Message::Binary(_)) => {
                if socket.send(echoed).await.is_err() {
                    break;
                }
            }
            Message::Ping(payload) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            Message::Pong(_) => {}
            Message::Close(_) => break,
        }
    }
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Runpod Axum WebSocket Echo</title>
  <style>
    body { font-family: sans-serif; max-width: 700px; margin: 40px auto; padding: 0 16px; }
    h1 { margin-bottom: 8px; }
    #log { border: 1px solid #ccc; border-radius: 8px; min-height: 160px; padding: 12px; white-space: pre-wrap; }
    .row { display: flex; gap: 8px; margin-top: 12px; }
    input { flex: 1; padding: 8px; }
    button { padding: 8px 12px; }
  </style>
</head>
<body>
  <h1>WebSocket Echo</h1>
  <p>This page connects to <code>/ws</code> and echoes messages.</p>
  <div id="log"></div>
  <div class="row">
    <input id="input" value="hello from browser" />
    <button id="send">Send</button>
  </div>

  <script>
    const log = document.getElementById('log');
    const input = document.getElementById('input');
    const send = document.getElementById('send');

    const wsProtocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new WebSocket(`${wsProtocol}//${location.host}/ws`);

    function write(line) {
      log.textContent += line + '\n';
      log.scrollTop = log.scrollHeight;
    }

    ws.onopen = () => write('connected');
    ws.onmessage = (event) => write('echo: ' + event.data);
    ws.onclose = () => write('closed');
    ws.onerror = () => write('error');

    send.onclick = () => {
      ws.send(input.value);
      write('sent: ' + input.value);
    };
  </script>
</body>
</html>
"#,
    )
}
