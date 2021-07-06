use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use reqwest::{Client, Error, Request, Response};
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Serialize)]
struct QueryParams {
    pub client_id: String,
    pub redirect_uri: String,
}

#[derive(Serialize)]
struct AccessTokenQueryParams {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

fn main() {
    let github_client_id =
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable.");
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET")
        .expect("Missing the GITHUB_CLIENT_SECRET environment variable.");
    let auth_url = "https://github.com/login/oauth/authorize";
    let token_url = "https://github.com/login/oauth/access_token";

    let query_params = QueryParams {
        client_id: github_client_id.clone(),
        redirect_uri: "http://localhost:3000/oauth2callback".to_string(),
    };

    // let client = reqwest::Client::new();
    let client = reqwest::blocking::Client::new();

    let result = client.get(auth_url).query(&query_params).build().unwrap();

    println!("OPEN THIS LINK: {:?}", result.url().to_string());

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

                println!("url: {:?}", url.to_string());

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
                token_url,
                &[
                    ("client_id", github_client_id.clone()),
                    ("client_secret", github_client_secret.clone()),
                    ("code", code.clone()),
                ],
            )
            .unwrap()
            .to_string();

            println!("token_url_with_params:\n{}\n", token_url_with_params);

            let token_result = client
                .post(token_url_with_params)
                .send()
                .unwrap()
                .text()
                .unwrap();

            println!("token_result:\n{}\n", token_result);
            break;
        }
    }
}
