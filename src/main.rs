use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use serde::Serialize;
use url::Url;

#[derive(Serialize)]
struct AuthorizationQueryParams {
    pub client_id: String,
    pub redirect_uri: String,
}

#[derive(Serialize)]
struct AccessTokenQueryParams {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

const AUTHORIZATION_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

fn main() {
    let github_client_id =
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable.");
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET")
        .expect("Missing the GITHUB_CLIENT_SECRET environment variable.");

    let auth_query_params = AuthorizationQueryParams {
        client_id: github_client_id.clone(),
        redirect_uri: "http://localhost:3000/oauth2callback".to_string(),
    };

    let client = reqwest::blocking::Client::new();

    let authorization_result = client.get(AUTHORIZATION_URL).query(&auth_query_params).build().unwrap();

    println!("OPEN THIS LINK: {:?}\n", authorization_result.url().to_string());

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = value.into_owned();
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Github returned the following code:\n{}\n", code);

            let token_url_with_params = Url::parse_with_params(
                TOKEN_URL,
                &[
                    ("client_id", github_client_id.clone()),
                    ("client_secret", github_client_secret.clone()),
                    ("code", code.clone()),
                ],
            )
                .unwrap()
                .to_string();

            let token_result = client
                .post(token_url_with_params)
                .send()
                .unwrap()
                .text()
                .unwrap();

            println!("This is your access token:\n{}\n", token_result);
            break;
        }
    }
}
