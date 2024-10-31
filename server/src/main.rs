use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::process::Command;

#[derive(Deserialize)]
struct UvRequest {
    project: String,
    command: String,
    args: Vec<String>,
}

#[derive(Serialize)]
struct UvResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

async fn handle_uv(Json(req): Json<UvRequest>) -> Json<UvResponse> {
    let output = Command::new("uv")
        .arg(req.command)
        .arg("--project")
        .arg(req.project)
        .args(req.args)
        .output()
        .await
        .unwrap_or_else(|e| panic!("Failed to execute uv: {}", e));

    Json(UvResponse {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/uv", post(handle_uv));

    println!("Starting server on http://localhost:9876");
    let listener = TcpListener::bind("127.0.0.1:9876").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

