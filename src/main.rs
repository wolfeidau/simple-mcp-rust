use serde::{Deserialize, Serialize};
use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use rmcp::{
    handler::server::wrapper::{Json, Parameters},
    schemars, tool, tool_router,
};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct HelloRequest {
    #[schemars(description = "the name of the person to say hello to")]
    name: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct HelloResponse {
    #[schemars(description = "the message to say hello to the person")]
    message: String,
}

#[derive(Debug, Clone)]
pub struct ServerHandlerImpl;

#[tool_router(server_handler)]
impl ServerHandlerImpl {
    #[tool(description = "Say hello to a person")]
    fn hello(params: Parameters<HelloRequest>) -> Json<HelloResponse> {
        Json(HelloResponse { message: format!("Hello, {}!", params.0.name) })
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    let service = ServerHandlerImpl.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
