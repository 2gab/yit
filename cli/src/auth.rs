use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::config::GoogleConfig;

const REDIRECT_URI: &str = "http://localhost:9999/callback";
const SCOPES: &str =
    "openid email profile https://www.googleapis.com/auth/youtube.readonly";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub email: String,
    pub name: Option<String>,
}

pub fn auth_path() -> PathBuf {
    dirs::data_dir()
        .expect("could not find data directory")
        .join("yit")
        .join("auth.json")
}

pub fn load_tokens() -> Option<Tokens> {
    let content = fs::read_to_string(auth_path()).ok()?;
    serde_json::from_str(&content).ok()
}

fn save_tokens(tokens: &Tokens) -> Result<()> {
    let path = auth_path();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, serde_json::to_string_pretty(tokens)?)?;
    Ok(())
}

pub async fn login(google: &GoogleConfig) -> Result<Tokens> {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
        ?client_id={}\
        &redirect_uri={REDIRECT_URI}\
        &response_type=code\
        &scope={}\
        &access_type=offline\
        &prompt=consent",
        google.client_id,
        urlencoding::encode(SCOPES)
    );

    println!("Opening browser for Google login...");
    open::that(&auth_url)?;

    let code = wait_for_callback().await?;
    let tokens = exchange_code(&code, google).await?;
    save_tokens(&tokens)?;

    println!("Logged in as {}", tokens.email);
    Ok(tokens)
}

async fn wait_for_callback() -> Result<String> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Waiting for Google callback on localhost:9999...");

    let (mut stream, _) = listener.accept().await?;
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;

    let code = request_line
        .split_whitespace()
        .nth(1)
        .and_then(|path| path.strip_prefix("/callback?"))
        .and_then(|query| {
            query.split('&').find_map(|param| {
                let (k, v) = param.split_once('=')?;
                if k == "code" { Some(v.to_string()) } else { None }
            })
        })
        .context("could not extract code from callback")?;

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h2>yit: login successful. You can close this tab.</h2></body></html>";
    stream.write_all(response.as_bytes()).await?;

    Ok(code)
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
}

#[derive(Deserialize)]
struct UserInfo {
    email: String,
    name: Option<String>,
}

async fn exchange_code(code: &str, google: &GoogleConfig) -> Result<Tokens> {
    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("code", code.to_string());
    params.insert("client_id", google.client_id.clone());
    params.insert("client_secret", google.client_secret.clone());
    params.insert("redirect_uri", REDIRECT_URI.to_string());
    params.insert("grant_type", "authorization_code".to_string());

    let token_res: TokenResponse = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    let user_info: UserInfo = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(&token_res.access_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(Tokens {
        access_token: token_res.access_token,
        refresh_token: token_res.refresh_token,
        email: user_info.email,
        name: user_info.name,
    })
}
