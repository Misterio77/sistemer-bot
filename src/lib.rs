pub mod disciplina;
pub mod horarios;

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

