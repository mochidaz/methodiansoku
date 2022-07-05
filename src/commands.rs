use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::MessageBuilder;

use crate::danbooru::Danbooru;

pub async fn get_random_from_tag(context: &Context, msg: &Message) {
    let splitted = msg.content.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
    let db = Danbooru::new(splitted.get(1));
    let response = MessageBuilder::new()
        .push(db.random_picture().await)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
}