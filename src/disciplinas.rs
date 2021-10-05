use anyhow::Result;
use regex::Regex;

pub async fn list_disciplinas() -> Result<Vec<String>> {
    let fulltext = reqwest::get("https://misterio.me/notes/bsi/disciplinas-2021-2.html")
        .await?
        .text()
        .await?;

    let disciplina_pattern = Regex::new("<h2 id=\"S")?;

    let mut output = Vec::new();
    for line in fulltext.lines() {
        if disciplina_pattern.is_match(line) {
            output.push(crate::sanitize_line(line, true)?);
        }
    }

    Ok(output)
}
