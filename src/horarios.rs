use anyhow::Result;
use regex::Regex;

pub async fn get_horarios() -> Result<String> {
    let fulltext = reqwest::get("https://misterio.me/notes/bsi/disciplinas-2021-2.html")
        .await?
        .text()
        .await?;

    let start_pattern = Regex::new("<h2 id=\"horários\">")?;
    let stop_pattern = Regex::new("<hr />")?;

    let mut output: String = "".into();
    let mut adding = false;
    for line in fulltext.lines() {
        if start_pattern.is_match(&line.to_lowercase()) {
            log::info!("Iniciando escrita");
            output.push_str(&crate::sanitize_line(line)?);
            adding = true;
        } else if adding {
            if stop_pattern.is_match(line) {
                log::info!("Finalizando escrita:\n{}", output);
                return Ok(output);
            } else {
                output.push_str(&crate::sanitize_line(line)?);
            }
        }
    }
    return Ok(output);
}