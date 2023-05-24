use serenity::model::prelude::ChannelId;
use shuttle_secrets::SecretStore;

pub struct Secrets {
    pub token: String,
    pub channel_id: ChannelId,
}

/// get secrets from secrets.toml and parse them appropriately
pub fn get_secrets(secret_store: SecretStore) -> Secrets {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        panic!("'DISCORD_TOKEN' was not found");
    };

    let channel_id: ChannelId = if let Some(channel_id) = secret_store.get("DISCORD_CHANNEL_ID") {
        let channel_id: u64 = match channel_id.parse() {
            Ok(channel_id) => channel_id,
            Err(error) => panic!("failed to parse DISCORD_CHANNEL_ID: {error}"),
        };
        ChannelId(channel_id)
    } else {
        panic!("'DISCORD_CHANNEL_ID' was not found");
    };

    Secrets { token, channel_id }
}
