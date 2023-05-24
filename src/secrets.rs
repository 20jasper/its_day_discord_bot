use serenity::model::prelude::{ChannelId, GuildId};
use shuttle_secrets::SecretStore;

pub struct Secrets {
    pub token: String,
    pub guild_id: GuildId,
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

    let guild_id = if let Some(guild_id) = secret_store.get("DISCORD_GUILD_ID") {
        let guild_id: u64 = match guild_id.parse() {
            Ok(guild_id) => guild_id,
            Err(error) => panic!("failed to parse DISCORD_GUILD_ID: {error}"),
        };
        GuildId(guild_id)
    } else {
        panic!("'DISCORD_GUILD_ID' was not found");
    };

    Secrets {
        token,
        guild_id,
        channel_id,
    }
}
