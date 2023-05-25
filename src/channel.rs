use serenity::client::Context;
use serenity::model::prelude::ChannelId;

pub async fn update_channel_name(ctx: Context, channel_id: ChannelId, new_channel_name: &str) {
    match channel_id
        .edit(&ctx, |channel| channel.name(new_channel_name))
        .await
    {
        Ok(_) => println!("update channel name successful!"),
        Err(error) => panic!("update channel name failed: {error}"),
    };
}
