// use joinerror::Error;
// use oauth2::{AuthorizationCode, CsrfToken};
// use std::{
//     io::{BufRead, BufReader, Write},
//     net::TcpListener,
// };
// use url::Url;

// pub(crate) fn create_auth_tcp_listener() -> joinerror::Result<(TcpListener, u16)> {
//     // Setting the port as 0 automatically assigns a free port
//     let listener = TcpListener::bind("127.0.0.1:0")?;
//     let callback_port = listener.local_addr()?.port();

//     Ok((listener, callback_port))
// }

// pub(crate) fn receive_auth_code(
//     listener: &TcpListener,
// ) -> joinerror::Result<(AuthorizationCode, CsrfToken)> {
//     let mut stream = listener
//     .incoming()
//     .flatten()
//     .next()
//     .ok_or_else(|| Error::new::<()>("failed to perform initial credentials setup: listener terminated without accepting a connection"))?;

//     let mut reader = BufReader::new(&stream);
//     let mut request_line = String::new();
//     reader.read_line(&mut request_line)?;

//     // GET /?code=*** HTTP/1.1
//     let redirect_url = request_line.split_whitespace().nth(1).ok_or_else(|| {
//         Error::new::<()>("failed to perform initial credentials setup: invalid request format")
//     })?;
//     let url = Url::parse(&("http://127.0.0.1".to_string() + redirect_url))?;

//     let code = url
//     .query_pairs()
//     .find(|(key, _)| key == "code")
//     .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
//     .ok_or_else(|| Error::new::<()>("failed to perform initial credentials setup: authorization code not found in request parameters"))?;

//     let state = url
//         .query_pairs()
//         .find(|(key, _)| key == "state")
//         .map(|(_, state)| CsrfToken::new(state.into_owned()))
//         .ok_or_else(|| {
//             Error::new::<()>(
//                 "failed to perform initial credentials setup: state not found in request parameters"
//             )
//         })?;

//     let message = "Go back to your terminal :)";
//     let response = format!(
//         "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
//         message.len(),
//         message
//     );
//     stream.write_all(response.as_bytes())?;

//     Ok((code, state))
// }
