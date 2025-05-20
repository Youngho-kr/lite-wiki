use std::collections::HashMap;

use axum::{extract::Query, response::{IntoResponse, Redirect}};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use tracing::{error, info};

use crate::{auth::respond_with_token_headers, config::{GITHUB_CALLBACK_URL, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_ORG}, handlers::redirect_to_root};



#[derive(Debug, Deserialize)]
pub struct GithubQuery {
    code: String,
}

pub async fn github_login() -> impl IntoResponse {
    let client_id = &*GITHUB_CLIENT_ID;
    let redirect_uri = &*GITHUB_CALLBACK_URL;

    Redirect::to(&format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user",
        client_id, redirect_uri
    ))
}

#[derive(Deserialize)]
struct GithubOrg {
    login: String,
}

pub async fn github_callback(Query(query): Query<GithubQuery>) -> impl IntoResponse {
    let client_id = GITHUB_CLIENT_ID.clone();
    let client_secret = GITHUB_CLIENT_SECRET.clone();
    let github_org = GITHUB_ORG.clone();

    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("clinet_id", &client_id);
    params.insert("clinet_secret", &client_secret);
    params.insert("code", &query.code);

    let res = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = res.json().await.unwrap();
    let access_token = json.get("access_token").and_then(|v| v.as_str());

    let token =  match access_token {
        Some(t) => t,
        None => {
            error!("Failed to get GitHub access_token: {:?}", json);
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let user_res = client
            .get("http://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "lite-wiki")
            .send()
            .await
            .unwrap();

    let user_info: serde_json::Value = user_res.json().await.unwrap();
    let login = user_info["login"].as_str().unwrap_or("unknown");
    info!("GitHub Login: {}", login);

    let org_res = client
        .get("https://api.github.com/user/orgs")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    if !org_res.status().is_success() {
        error!("Failed to get organzation: {:?}", org_res.status());
        return StatusCode::FORBIDDEN.into_response();
    }

    let orgs: Vec<GithubOrg> = org_res.json().await.unwrap();

    let is_member = orgs.iter().any(|org| org.login.eq_ignore_ascii_case(&github_org));

    if !is_member {
        error!("No permission: {}", login);
        return StatusCode::FORBIDDEN.into_response();
    }

    let headers = respond_with_token_headers(login);

    let redirect = redirect_to_root();

    (headers, redirect).into_response()
}