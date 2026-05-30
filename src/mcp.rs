use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use rmcp::handler::server::tool::IntoCallToolResult;
    use pretty_assertions::{assert_eq};

    use super::*;

    #[test]
    fn hello_returns_greeting_for_name() {
        let Json(response) = ServerHandlerImpl::hello(Parameters(HelloRequest {
            name: "World".into(),
        }));

        assert_eq!(response.message, "Hello, World!");
    }

    #[test]
    fn hello_produces_structured_tool_result() {
        let result = ServerHandlerImpl::hello(Parameters(HelloRequest {
            name: "MCP".into(),
        }))
        .into_call_tool_result()
        .expect("hello should serialize to a tool result");

        let structured = result
            .structured_content
            .expect("Json wrapper should produce structured content");

        assert_eq!(structured["message"], "Hello, MCP!");
    }
}