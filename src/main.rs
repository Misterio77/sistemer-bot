use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, ParseMode::Html, ReplyMarkup};
use teloxide::utils::command::BotCommand;

use anyhow::Result;
use dotenv::dotenv;
use regex::Regex;

use sistemer_bot::{agora, disciplina, disciplinas, hoje, horarios, proxima};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Comandos que tenho no momento:")]
enum Command {
    #[command(description = "exibe essa ajuda.")]
    Help,
    #[command(description = "mostra info de uma disciplina.")]
    Disciplina(String),
    #[command(description = "mostra info de uma disciplina.")]
    D(String),
    #[command(description = "lista os horários.")]
    Horarios,
    #[command(description = "informações da aula atual.")]
    Agora,
    #[command(description = "informações da próxima aula.")]
    Proxima,
    #[command(description = "quais as aulas de hoje.")]
    Hoje,
}

fn disciplina_row(disciplina: String) -> Vec<KeyboardButton> {
    let disciplina = Regex::new(".* - ").unwrap().replace_all(&disciplina, "");
    vec![KeyboardButton {
        text: format!("/d {}", disciplina),
        request: None,
    }]
}

async fn answer_command(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Disciplina(disciplina) | Command::D(disciplina) => {
            if disciplina.is_empty() {
                let username = format!(
                    "@{}, ",
                    cx.update
                        .from()
                        .and_then(|u| u.clone().username)
                        .unwrap_or_default()
                );
                cx.answer(&format!("Certo, {}qual disciplina?", username))
                    .reply_markup(ReplyMarkup::Keyboard(KeyboardMarkup {
                        keyboard: disciplinas::list_disciplinas()
                            .await?
                            .into_iter()
                            .map(disciplina_row)
                            .collect(),
                        one_time_keyboard: Some(true),
                        selective: Some(true),
                        ..Default::default()
                    }))
                    .send()
                    .await?
            } else {
                cx.answer(disciplina::get_disciplina(&disciplina).await?)
                    .parse_mode(Html)
                    .await?
            }
        }
        Command::Horarios => {
            cx.answer(horarios::get_horarios().await?)
                .parse_mode(Html)
                .await?
        }
        Command::Agora => {
            cx.answer(agora::get_agora().await?)
                .parse_mode(Html)
                .await?
        }
        Command::Proxima => {
            cx.answer(proxima::get_proxima().await?)
                .parse_mode(Html)
                .await?
        }
        Command::Hoje => {
            cx.answer(hoje::get_horarios_hoje().await?)
                .parse_mode(Html)
                .await?
        }
    };

    Ok(())
}

async fn run() {
    teloxide::enable_logging!();

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "Sistemer_Bot".into();

    teloxide::commands_repl(bot, bot_name, answer_command).await;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}
