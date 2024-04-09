use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Asset {
    pub name: String,
    pub content_type: String,
    pub browser_download_url: String,
}

#[async_recursion::async_recursion(?Send)]
pub async fn get_github_release() -> Result<Release, reqwest::Error> {
    let url = "https://api.github.com/repos/revanced-apks/build-apps/releases/latest";
    let response = reqwest::get(url).await?;
    let json = response.json::<Release>().await?;
    Ok(json)
}