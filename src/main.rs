use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let addr = ([127, 0, 0, 1], 8000).into();
    let url = "Your HTTPS ngrok URL here. Get it by `ngrok http 8000`".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
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
            bot.send_message(msg.chat.id, format!("The URL you want to short is: {url}")).await?
        }
    };

    Ok(())
}
