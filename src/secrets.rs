use serenity::model::prelude::ChannelId;
use std::env;

pub struct Secrets {
    pub token: String,
    pub channel_id: ChannelId,
}

/// get secrets from environment and parse them appropriately
pub fn get_secrets() -> Secrets {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("'DISCORD_TOKEN' was not found: {error}"),
    };

    let channel_id: ChannelId = match env::var("DISCORD_CHANNEL_ID") {
        Ok(channel_id) => match channel_id.parse() {
            Ok(channel_id) => channel_id,
            Err(error) => panic!("failed to parse DISCORD_CHANNEL_ID: {error}"),
        },
        Err(error) => panic!("'DISCORD_CHANNEL_ID' was not found: {error}"),
    };

    Secrets { token, channel_id }
}
