use serenity::http::client::Http;

mod channel;
mod secrets;

use channel::update_channel_name;
use secrets::{get_secrets, Secrets};

#[tokio::main]
async fn main() {
    let Secrets { token, channel_id } = get_secrets();

    let http = Http::new(&token);

    channel_id
        .say(&http, "hello everyone")
        .await
        .expect("there is a mistake with say");

    update_channel_name(http, channel_id, "hello-testing").await;
}
