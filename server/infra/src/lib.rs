use generated::models;
use once_cell::sync::Lazy;
use redis::Commands;
use std::env;

static NINGENME_REDIS_HOST: Lazy<String> = Lazy::new(|| {
    match env::var("NINGENME_REDIS_HOST") {
        Ok(it) => it,
        Err(_) => "127.0.0.1".to_string(),
    }
});

static REDIS_URL: Lazy<String> = Lazy::new(|| {
    format!(
        "redis://{}:6379/",
        *NINGENME_REDIS_HOST,
    )
});

pub fn set(bookmark_directory_list: Vec<models::BookmarkDirectory>) {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();
    let _:()  = connection.set("bookmark", serde_json::to_string(&bookmark_directory_list).unwrap()).unwrap();
}

pub fn get() -> Vec<models::BookmarkDirectory> {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();
    let res: Result<String, redis::RedisError> = connection.get("bookmark");
    match res {
        Ok(res) => return serde_json::from_str::<Vec<models::BookmarkDirectory>>(&res).unwrap(),
        Err(_) => return vec![],
    }
}