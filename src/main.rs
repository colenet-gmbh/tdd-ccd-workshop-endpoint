use actix_web::{get, web, App, HttpServer, Result};
use actix_web::error::ErrorNotFound;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Follower {
    pub login: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MockUser {
    pub name: String,
    pub login: String,
    pub followers: Vec<Follower>,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct GithubUser {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    site_admin: bool,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: Option<String>,
    hireable: Option<String>,
    bio: Option<String>,
    twitter_username: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    created_at: String,
    updated_at: String
}


fn find_user(user_name: String) -> std::result::Result<Option<MockUser>, serde_json::error::Error>
{
    let users = include_str!("users.json");
    let users: Vec<MockUser> = serde_json::from_str(users)?;
    let user: Option<MockUser> = users.into_iter()
                .find(|user| user.login.eq(&user_name.to_string()));

    Ok(user)
}

fn find_github_user(login: String) -> std::result::Result<Option<GithubUser>, serde_json::error::Error>
{
    let users = include_str!("github.json");
    let users: Vec<GithubUser> = serde_json::from_str(users)?;
    let user: Option<GithubUser> = users.into_iter()
                .find(|user| user.login.eq(&login.to_string()));

    Ok(user)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_user() {
        let actual = find_user(String::from("rwaldvogel"));
        if let Ok(Some(user)) = actual {
            assert_eq!(user.name, "Ralf Waldvogel")
        } else {
            assert_eq!("Fehler", "");
        }
    }

    #[test]
    fn test_find_github_user() {
        let actual = find_github_user(String::from("maschonber"));
        if let Ok(Some(user)) = actual {
            assert_eq!(user.name, "Martin Schönberger")
        } else {
            assert_eq!("Fehler", "");
        }
    }
}

#[get("/users/{user_name}")]
async fn get_mock_user(user_name: web::Path<String>) -> Result<web::Json<MockUser>> {
    let user = find_user(user_name.to_string())?;

    match user {
        Some(user) => Ok(web::Json(user.clone())),
        None => Err(ErrorNotFound("user does not exist"))
    }
}

#[get("/github/users/{user_name}")]
async fn get_mock_github_user(user_name: web::Path<String>) -> Result<web::Json<GithubUser>> {
    let user = find_github_user(user_name.to_string())?;

    match user {
        Some(user) => Ok(web::Json(user.clone())),
        None => Err(ErrorNotFound("user does not exist"))
    }
}


#[get("/version")]
async fn get_version() -> Result<String> {
    Ok(env!( "CARGO_PKG_VERSION" ).to_string())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware::Logger;
    use log;

    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(get_mock_user)
            .service(get_mock_github_user)
            .service(get_version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
