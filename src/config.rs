use dotenv::dotenv;
use std::env;

pub fn get_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("Set the github key in .env")
}
