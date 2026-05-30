use std::process::Stdio;

use rmcp::{
    model::CallToolRequestParams,
    transport::{ConfigureCommandExt, TokioChildProcess},
    ServiceExt,
};
use serde_json::json;
use pretty_assertions::{assert_eq};

#[tokio::test]
async fn hello_tool_over_stdio() -> anyhow::Result<()> {
    let (transport, _stderr) = TokioChildProcess::builder(
        tokio::process::Command::new(env!("CARGO_BIN_EXE_simple-mcp-rust")).configure(|cmd| {
            cmd.env("RUST_LOG", "off");
        }),
    )
    .stderr(Stdio::null())
    .spawn()?;

    let client = ().serve(transport).await?;

    let tools = client.list_all_tools().await?;
    assert!(
        tools.iter().any(|tool| tool.name == "hello"),
        "expected hello tool to be listed, got: {:?}",
        tools.iter().map(|t| &t.name).collect::<Vec<_>>()
    );

    let mut arguments = serde_json::Map::new();
    arguments.insert("name".into(), json!("Integration"));

    let result = client
        .call_tool(CallToolRequestParams::new("hello").with_arguments(arguments))
        .await?;

    let structured = result
        .structured_content
        .expect("hello tool should return structured content");

    assert_eq!(structured["message"], "Hello, Integration!");

    client.cancel().await?;
    Ok(())
}
