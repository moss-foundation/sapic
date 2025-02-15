#[cfg(test)]
mod tests {
    use git2::build::RepoBuilder;
    use git2::RemoteCallbacks;
    use git2::{Config, Cred, Repository};
    use oauth2::basic::BasicClient;
    use oauth2::reqwest;
    use oauth2::url::Url;
    use oauth2::{
        AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
        TokenResponse, TokenUrl,
    };
    use std::fs::remove_dir_all;
    use std::io::{stdin, BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::path::Path;

    fn clone_flow(url: &str, path: &Path, callback: RemoteCallbacks) -> Result<Repository, String> {
        // remove_dir_all(path);

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callback);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        match builder.clone(url, path) {
            Ok(repo) => Ok(repo),
            Err(e) => Err(format!("failed to clone: {}", e)),
        }
    }
    #[test]
    fn cloning_with_https() {
        let repo_url = "https://github.com/***.git";
        let repo_path = Path::new("Local Path");

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            let default_config = Config::open_default().unwrap();
            Cred::credential_helper(&default_config, url, username_from_url)
        });

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();

    }
    
    #[test]
    fn cloning_with_ssh() {
        let repo_url = "git@github.com:***/***";
        let repo_path = Path::new("Local Path");

        let private = Path::new(".ssh/id_***");
        let public = Path::new(".ssh/id_***.pub");
        let password = "PASSWORD";

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            Cred::ssh_key("git", Some(public), private, Some(password))
        });

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }

    // Run cargo test cloning_with_oauth_github -- --nocapture
    #[test]
    fn cloning_with_oauth_github() {
        // From example: https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs
        let repo_url = "https://github.com/***.git";
        let repo_path = Path::new("Local Path");

        let github_client_id = "ID";
        let github_client_secret = "Secret";
        let callback_port = "1357";

        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(ClientId::new(github_client_id.into()))
            .set_client_secret(ClientSecret::new(github_client_secret.into()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            // This example will be running its own server at localhost:8080.
            // See below for the server implementation.
            .set_redirect_uri(
                RedirectUrl::new(format!("http://localhost:{}", callback_port))
                    .expect("Invalid redirect URL"),
            );

        let http_client = reqwest::blocking::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            // This example is requesting access to the user's public repos and private repos
            .add_scope(Scope::new("repo".to_string()))
            .url();

        println!("Open this URL in your browser:\n{authorize_url}\n");
        let (code, state) = {
            // A very naive implementation of the redirect server.
            let listener = TcpListener::bind(format!("localhost:{}", callback_port)).unwrap();

            // The server will terminate itself after collecting the first code.
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()))
                .unwrap();

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            (code, state)
        };

        println!("Github returned the following code:\n{}\n", code.secret());
        println!(
            "Github returned the following state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_state.secret()
        );

        // Exchange the code with a token.
        let token_res = client.exchange_code(code).request(&http_client).unwrap();

        println!("Github returned the following token:\n{token_res:?}\n");

        let access_token = token_res.access_token();

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", access_token.secret())
        });

        let repo = clone_flow(repo_url, repo_path, callbacks).unwrap();
    }
}
