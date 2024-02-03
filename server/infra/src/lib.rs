use generated::models;
use redis::Commands;

static REDIS_ADDRESS: &str = "redis://127.0.0.1:6379/";

pub fn set(bookmark_directory_list: Vec<models::BookmarkDirectory>) {
    let client = redis::Client::open(REDIS_ADDRESS).unwrap();
    let mut connection = client.get_connection().unwrap();
    let _:()  = connection.set("bookmark", serde_json::to_string(&bookmark_directory_list).unwrap()).unwrap();
}

pub fn get() -> Vec<models::BookmarkDirectory> {
    let client = redis::Client::open(REDIS_ADDRESS).unwrap();
    let mut connection = client.get_connection().unwrap();
    let res: Result<String, redis::RedisError> = connection.get("bookmark");
    match res {
        Ok(res) => return serde_json::from_str::<Vec<models::BookmarkDirectory>>(&res).unwrap(),
        Err(_) => return vec![],
    }
}