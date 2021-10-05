use anyhow::Result;
use regex::Regex;

pub async fn get_disciplina(disciplina: &str) -> Result<String> {
    let fulltext = reqwest::get("https://misterio.me/notes/bsi/disciplinas-2021-2.html")
        .await?
        .text()
        .await?;

    let start_pattern = Regex::new(&format!("<h2.*{}.*</h2>", unidecode::unidecode(&disciplina.to_lowercase())))?;
    let stop_pattern = Regex::new("<hr />")?;

    let mut output: String = "".into();
    let mut adding = false;
    for line in fulltext.lines() {
        if start_pattern.is_match(&unidecode::unidecode(&line.to_lowercase())) {
            output.push_str(&crate::sanitize_line(line, false)?);
            adding = true;
        } else if adding {
            if stop_pattern.is_match(line) {
                return Ok(output);
            } else {
                output.push_str(&crate::sanitize_line(line, false)?);
            }
        }
    }

    Ok("Disciplina não encontrada!".into())
}
