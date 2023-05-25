use serenity::client::Context;
use serenity::model::prelude::{ChannelId, GuildChannel};

pub async fn update_channel_name(
    ctx: Context,
    channel_id: ChannelId,
) -> Result<GuildChannel, serenity::Error> {
    channel_id
        .edit(&ctx, |channel| channel.name("cool-name"))
        .await
}
