use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};
use url::Url;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let addr = ([127, 0, 0, 1], 8000).into();
    let ngrok_url = "https://788a-186-185-126-75.ngrok.io".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, ngrok_url))
        .await
        .expect("Couldn't setup webhook");


    Command::repl_with_listener(bot, answer, listener).await;

}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Use this command to short a URL")]
    Url(String),
    #[command(description = "handle user's chat ID")]
    ChatId,
    
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::ChatId => {
            bot.send_message(msg.chat.id, format!("Your chat ID is {chat_id}")).await?
        }
        Command::Url(url) => {
            bot.send_message(msg.chat.id, shorten(url).to_string()).await?
        }
    };

    Ok(())
}

fn shorten(url: String) -> String {
    let id = &nanoid::nanoid!(6);
    
    let parsed_url = Url::parse(&url);
    
    format!("https://{id}")
}