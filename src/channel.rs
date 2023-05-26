use chrono::Weekday;
use chrono::{Datelike, Utc};
use serenity::http::client::Http;
use serenity::model::prelude::ChannelId;

pub async fn update_channel_name(http: Http, channel_id: ChannelId) {
    match channel_id
        .edit(http, |channel| channel.name(get_new_channel_name()))
        .await
    {
        Ok(_) => println!("update channel name successful!"),
        Err(error) => panic!("update channel name failed: {error}"),
    };
}

/// get current utc weekday
fn get_current_week_day() -> String {
    let utc_week_day = Utc::now().naive_utc().weekday();

    match utc_week_day {
        Weekday::Mon => "monday".to_owned(),
        Weekday::Tue => "tuesday".to_owned(),
        Weekday::Wed => "wednesday".to_owned(),
        Weekday::Thu => "thursday".to_owned(),
        Weekday::Fri => "friday".to_owned(),
        Weekday::Sat => "saturday".to_owned(),
        Weekday::Sun => "sunday".to_owned(),
    }
}

fn get_new_channel_name() -> String {
    let week_day = get_current_week_day();

    format!("its-{week_day}-bitches")
}
