use crate::{disciplina, hoje};
use anyhow::Result;
use chrono::offset::Local;
use regex::Regex;

pub async fn get_proxima() -> Result<String> {
    let horarios_texto = hoje::get_horarios_hoje().await?;
    let agora = Local::now().time();

    for linha in horarios_texto.lines() {
        let (inicio, _) = crate::get_intervalo(linha)?;
        if inicio > agora {
            let nome_disciplina = Regex::new("^.*<a href=\"#(.*)\">.*$")?.replace(linha, "$1");
            log::info!("Disciplina: {:?}", nome_disciplina);
            return disciplina::get_disciplina(&nome_disciplina).await;
        }
    }
    Ok("Nenhuma aula hoje mais tarde!".into())
}
