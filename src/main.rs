use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;
use teloxide::utils::command::BotCommand;

use anyhow::Result;
use dotenv::dotenv;

use sistemer_bot::disciplina;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "exibe essa ajuda.")]
    Help,
    #[command(description = "mostra info de uma disciplina.")]
    Disciplina(String),
}

async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Disciplina(disciplina) => {
            cx.answer(disciplina::get_disciplina(&disciplina).await?)
                .parse_mode(Html)
                .await?
        }
    };

    Ok(())
}

async fn run() {
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "Sistemer Bot".into();

    teloxide::commands_repl(bot, bot_name, answer).await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}
