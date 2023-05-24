use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::prelude::{Context, GatewayIntents};
use serenity::Client;
use serenity::{async_trait, prelude::EventHandler};
use shuttle_secrets::SecretStore;
use tracing::info;

mod channel;
mod secrets;

use channel::update_channel_name;
use secrets::{get_secrets, Secrets};

struct Bot {
    channel_id: ChannelId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            println!("hi")
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let Secrets { token, channel_id } = get_secrets(secret_store);

    let client = Client::builder(&token, intents)
        .event_handler(Bot { channel_id })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
