use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepoData {
    pub name: String,
    pub description: Option<String>, // Description can be null
    pub forks_count: u32,
    pub language: Option<String>, // Language can be null
    pub created_at: String,
    pub stargazers_count: u32,
    pub link: Option<String>
}

pub async fn fetch_data(url: &str) -> Result<RepoData, Error> {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header(USER_AGENT, user_agent)
        .send().await?;
    let data = response.json::<RepoData>().await?;
    Ok(data)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    async fn test_fetch_data_github_repo() {
        let url = "https://api.github.com/repos/monikeo/portfolio-site";
        let data = fetch_data(url).await;
        println!("{:#?}", data);
        assert!(true);
    }
}
*/
