use moss_git_hosting_provider::{GitAuthAdapter, github::GitHubAuthAdapter};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker_url = "https://account-auth-gateway-dev.20g10z3r.workers.dev".to_string();
    let callback_port = 8080;

    let http_client = Client::new();

    let adapter = GitHubAuthAdapter::new(http_client, worker_url, callback_port);

    println!("🚀 Run GitHub OAuth through Cloudflare Worker...");
    println!("📡 Worker URL: https://account-auth-gateway-dev.20g10z3r.workers.dev");
    println!("🔗 Callback port: {}", callback_port);
    println!();

    let token = adapter.auth_with_pkce().await?;

    println!();
    println!("✅ Authorization successful!");
    println!("🔑 Access Token: {}", token.access_token);

    Ok(())
}
