use moss_git_hosting_provider::{GitAuthAdapter, gitlab::GitLabAuthAdapter};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker_url = "https://account-auth-gateway-dev.20g10z3r.workers.dev".to_string();
    let callback_port = 8081;

    let http_client = Client::new();

    let adapter = GitLabAuthAdapter::new(http_client, worker_url, callback_port);

    println!("🚀 Run GitLab OAuth through Cloudflare Worker...");
    println!("📡 Worker URL: https://account-auth-gateway-dev.20g10z3r.workers.dev");
    println!("🔗 Callback port: {}", callback_port);
    println!();

    let token = adapter.auth_with_pkce().await?;

    println!();
    println!("✅ Authorization successful!");
    println!("🔑 Access Token: {}", token.access_token);
    println!("🔑 Refresh Token: {}", token.refresh_token);

    Ok(())
}
