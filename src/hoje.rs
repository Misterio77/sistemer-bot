use crate::horarios;
use anyhow::Result;
use chrono::{offset::Local, Datelike, Duration, Weekday};

fn nome_dia(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Segunda",
        Weekday::Tue => "Terça",
        Weekday::Wed => "Quarta",
        Weekday::Thu => "Quinta",
        Weekday::Fri => "Sexta",
        Weekday::Sat => "Sábado",
        Weekday::Sun => "Domingo",
    }
    .into()
}

pub async fn get_horarios_hoje() -> Result<String> {
    let texto_horarios = horarios::get_horarios().await?;
    let nome_hoje = nome_dia(Local::today().weekday());
    let nome_amanha = nome_dia((Local::today() + Duration::days(1)).weekday());

    let mut output: String = "".into();
    let mut adding = false;
    for line in texto_horarios.lines() {
        if line.contains(&nome_hoje) {
            adding = true;
        } else if line.contains(&nome_amanha) {
            adding = false;
        } else if adding {
            output.push_str(&crate::sanitize_line(line.trim(), false)?);
        }
    }

    let output = if output.is_empty() {
        "Sem aulas hoje!".into()
    } else {
        output
    };
    Ok(output)
}
