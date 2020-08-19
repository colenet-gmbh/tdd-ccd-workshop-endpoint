use actix_web::{get, web, App, HttpServer, Result, Error};
use actix_web::error::ErrorNotFound;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

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

fn find_user(user_name: String) -> std::result::Result<Option<MockUser>, serde_json::error::Error>
{
    let users = include_str!("users.json");
    let users: Vec<MockUser> = serde_json::from_str(users)?;
    let user: Option<MockUser> = users.into_iter()
                .find(|user| user.login.eq(&user_name.to_string()));

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
}

#[get("/users/{user_name}")]
async fn get_mock_user(user_name: web::Path<String>) -> Result<web::Json<MockUser>> {
    use std::time::Duration;
    
    let mut rng: ThreadRng = rand::thread_rng();
    let delay: u64 = rng.gen_range(300, 1200);    
    futures_timer::Delay::new(Duration::from_millis(delay)).await;

    let user = find_user(user_name.to_string())?;

    match user {
        Some(user) => Ok(web::Json(user.clone())),
        None => Err(ErrorNotFound("user does not exist"))
    }
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
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}