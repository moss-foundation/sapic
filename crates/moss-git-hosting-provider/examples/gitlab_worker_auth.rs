#![allow(deprecated)] // TODO: remove once we get rid of old context types

use moss_applib::{TauriAppRuntime, Wry};
use moss_git_hosting_provider::{GitAuthAdapter, gitlab::AppGitLabAuthAdapter};
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use reqwest::Client;
use sapic_core::context::MutableContext;
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

    let adapter = AppGitLabAuthAdapter::<TauriAppRuntime<Wry>>::new(
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
