use std::env;

pub fn get_token() -> String {
    dotenv::from_filename(".env.dev").ok();
    env::var("BOT_TOKEN").expect("BOT_TOKEN must be set in the environment")
}
