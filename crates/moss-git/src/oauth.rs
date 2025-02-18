// https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
// https://datatracker.ietf.org/doc/html/rfc8252#section-7



use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use git2::{Cred, RemoteCallbacks};
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::url::Url;

pub struct OAuth {
    auth_url: String,
    token_url: String,
    client_id: String,
    client_secret: String,
}

impl OAuth {
    pub fn new(auth_url: &str, token_url: &str, client_id: &str, client_secret: &str) -> OAuth {
        OAuth {
            auth_url: auth_url.to_string(),
            token_url: token_url.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        }
    }


    pub fn flow(&self, remote_callbacks: &mut RemoteCallbacks) {
        let auth_url = AuthUrl::new(self.auth_url.clone())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(self.token_url.clone())
            .expect("Invalid token endpoint URL");

        // Setting the port as 0 automatically assigns a free port
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let callback_port = listener.local_addr().unwrap().port();

        let client = BasicClient::new(ClientId::new(self.client_id.to_string()))
            .set_client_secret(ClientSecret::new(self.client_secret.to_string()))
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(
                RedirectUrl::new(format!("http://127.0.0.1:{}", callback_port.to_string()))
                    .expect("Invalid redirect URL"),
            );

        // https://datatracker.ietf.org/doc/html/rfc7636#section-1.1
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            // This example is requesting access to the user's public repos and private repos
            .add_scope(Scope::new("repo".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        // TODO: Open a webview window for this url

        println!("Open this URL in your browser:\n{authorize_url}\n");
        let (code, state) = {
            // A very naive implementation of the redirect server.


            // The server will terminate itself after collecting the first code.
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            // GET /?code=*** HTTP/1.1
            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://127.0.0.1".to_string() + redirect_url)).unwrap();

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

            // TODO: Once the code is received, the focus should switch back to the main application
            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            (code, state)
        };

        let http_client = reqwest::blocking::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // Exchange the code alongside with PKCE verifier with a token .
        let token_res =
            client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request(&http_client).unwrap();

        let access_token = token_res.access_token().secret().to_owned();

        remote_callbacks.credentials(move |url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext("oauth2", access_token.as_str())
        });
    }
}