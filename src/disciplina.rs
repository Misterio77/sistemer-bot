use anyhow::Result;
use regex::Regex;

fn sanitize_line(line: &str) -> Result<String> {
    log::info!("Transformando {:?}", line);

    let line = format!("{}\n", line);
    // Replace headers with <b>
    let line = Regex::new("h\\d")?.replace_all(&line, "b");
    // Remove <ul> entire lines
    let line = Regex::new("^ *</?ul>\n")?.replace_all(&line, "");
    let line = Regex::new("</?ul>")?.replace_all(&line, "");
    // Remove <li>
    let line = Regex::new("^ *</?li>\n")?.replace_all(&line, "");
    let line = Regex::new("</?li>")?.replace_all(&line, "");
    // Remove <p>
    let line = Regex::new("</?p>")?.replace_all(&line, "");
    // Remove email links
    let line = Regex::new("<a href=\"mailto:.*>(.*)</a>")?.replace(&line, "$1");

    log::info!("Em: {:?}", line);

    Ok(line.into())
}

pub async fn get_disciplina(disciplina: &str) -> Result<String> {
    let fulltext = reqwest::get("https://misterio.me/notes/bsi/disciplinas-2021-2.html")
        .await?
        .text()
        .await?;

    let start_pattern = Regex::new(&format!("<h2.*>.*{}.*</h2>", disciplina))?;
    let stop_pattern = Regex::new("<hr />")?;

    let mut output: String = "".into();
    let mut adding = false;
    for line in fulltext.lines() {
        if start_pattern.is_match(line) {
            log::info!("Iniciando escrita");
            output.push_str(&sanitize_line(line)?);
            adding = true;
        } else if adding {
            if stop_pattern.is_match(line) {
                log::info!("Finalizando escrita:\n{}", output);
                return Ok(output);
            } else {
                output.push_str(&sanitize_line(line)?);
            }
        }
    }

    Ok("Disciplina não encontrada!".into())
}
