use std::collections::HashMap;

use axum::{extract::Query, response::{IntoResponse, Redirect}};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

use tracing::{error, info};

use crate::{auth::respond_with_token_headers, config::{GITHUB_CALLBACK_URL, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_ORG}, handlers::{redirec_to_page, redirect_to_root}};



#[derive(Debug, Deserialize)]
pub struct GithubQuery {
    code: String,
}

pub async fn github_login() -> impl IntoResponse {
    let client_id = &*GITHUB_CLIENT_ID;
    let redirect_uri = &*GITHUB_CALLBACK_URL;

    Redirect::to(&format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:org",
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

    let client = reqwest::Client::builder()
    .user_agent("lite-wiki")
    .https_only(true) // 명시적으로 HTTPS만 사용
    .build()
    .unwrap();

    // let client = reqwest::Client::builder()
    // .danger_accept_invalid_certs(true) // 개발 중이라면 허용
    // .user_agent("lite-wiki")
    // .build()
    // .unwrap();

    let mut params = HashMap::new();
    params.insert("client_id", client_id.as_str());
    params.insert("client_secret", client_secret.as_str());
    params.insert("code", query.code.as_str());
    params.insert("redirect_uri", GITHUB_CALLBACK_URL.as_str());

    let res = match client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            error!("Github token 요청 실패: {}", err);
            return Redirect::to("/login?error=token_request_failed").into_response();
        }
    };

    let json: serde_json::Value = match res.json().await {
        Ok(j) => j,
        Err(err) => {
            error!("Github 응답 파싱 실패: {}", err);
            return Redirect::to("/login?error=invalid_token_response").into_response();
        }
    };

    let scope_info = json.get("scope").and_then(|v| v.as_str()).unwrap_or("none");
    info!("GitHub token scope: {}", scope_info);

    let access_token = json.get("access_token").and_then(|v| v.as_str());

    let token =  match access_token {
        Some(t) => t,
        None => {
            error!("Failed to get GitHub access_token: {:?}", json);
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let user_res = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .send()
            .await
            .unwrap();

    let user_info: serde_json::Value = user_res.json().await.unwrap();

    let login = match user_info["login"].as_str() {
        Some(l) => l,
        None => {
            error!("GitHub /user 응답에서 login 필드 없음: {:?}", user_info);
            return Redirect::to("/login?error=missing_login").into_response();
        }
    };
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