use moss_applib::{TauriAppRuntime, Wry, context::MutableContext};
use moss_git_hosting_provider::{GitAuthAdapter, gitlab::RealGitLabAuthAdapter};
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use reqwest::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = MutableContext::background().freeze();
    let callback_port = 8081;
    let auth_api_client: Arc<AccountAuthGatewayApiClient> = AccountAuthGatewayApiClient::new(
        Client::new(),
        "https://account-auth-gateway-dev.20g10z3r.workers.dev".to_string(),
    )
    .into();
    let worker_url = auth_api_client.base_url();

    let adapter = RealGitLabAuthAdapter::<TauriAppRuntime<Wry>>::new(
        auth_api_client,
        worker_url,
        callback_port,
    );

    println!("🚀 Run GitLab OAuth through Cloudflare Worker...");
    println!("📡 Worker URL: https://account-auth-gateway-dev.20g10z3r.workers.dev");
    println!("🔗 Callback port: {}", callback_port);
    println!();

    let token = adapter.auth_with_pkce(&ctx).await?;

    println!();
    println!("✅ Authorization successful!");
    println!("🔑 Access Token: {}", token.access_token);
    println!("🔑 Refresh Token: {}", token.refresh_token);

    Ok(())
}
