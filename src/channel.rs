use serenity::client::Context;
use serenity::model::prelude::{ChannelId, GuildChannel};

pub async fn update_channel_name(
    ctx: Context,
    channel_id: ChannelId,
    new_channel_name: &str,
) -> Result<GuildChannel, serenity::Error> {
    channel_id
        .edit(&ctx, |channel| channel.name(new_channel_name))
        .await
}
