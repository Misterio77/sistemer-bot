pub mod agora;
pub mod disciplina;
pub mod disciplinas;
pub mod hoje;
pub mod horarios;
pub mod proxima;

use anyhow::Result;
use chrono::NaiveTime;
use regex::Regex;

fn get_intervalo(input: &str) -> Result<(NaiveTime, NaiveTime)> {
    let inicio = Regex::new("^(.*) - .*:.*$")?.replace(input, "$1");
    let fim = Regex::new("^.* - (.*):.*$")?.replace(input, "$1");
    let parse = |x| NaiveTime::parse_from_str(x, "%H:%M");
    Ok((parse(&inicio)?, parse(&fim)?))
}

fn sanitize_line(line: &str, remove_bold: bool) -> Result<String> {
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
    // Fix calculadora links
    let line = Regex::new("/notes/bsi/calculadora")?
        .replace(&line, "https://misterio.me/notes/bsi/calculadora");

    let line = if remove_bold {
        Regex::new("<b.*>(.*)</b>")?.replace(&line, "$1")
    } else {
        line
    };

    Ok(line.into())
}
