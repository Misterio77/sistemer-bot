pub use anyhow::Result;

use anyhow::anyhow;
use indoc::formatdoc;
use serde::Deserialize;
const URL: &str = "http://127.0.0.1:4000";

#[derive(Deserialize)]
pub struct Disciplina {
    codigo: String,
    nome: String,
    #[serde(default)]
    optativa: bool,
    professor: Option<Professor>,
    plataforma: Option<Informacao>,
    presenca: Option<Informacao>,
    sala: Option<Informacao>,
    avaliacoes: Option<Avaliacoes>,
}

impl Disciplina {
    pub async fn listar_disciplinas(turma: &str) -> Result<Vec<Self>> {
        let url = format!("{}/disciplinas/{}.json", URL, turma);
        let response = reqwest::get(&url).await?;

        let disciplinas = response.json().await?;
        Ok(disciplinas)
    }
    pub async fn buscar_disciplina(turma: &str, busca: &str) -> Result<Self> {
        let disciplinas = Self::listar_disciplinas(turma).await?;
        let encontrado = disciplinas
            .into_iter()
            .find(|x| x.codigo == busca)
            .ok_or(anyhow!("Disciplina não encontrada"))?;
        Ok(encontrado)
    }

    fn titulo(&self) -> String {
        let codigo = &self.codigo;
        let nome = &self.nome;
        format!("*{codigo} - {nome}*")
    }

    fn sobre(&self) -> String {
        let tipo = if self.optativa {
            "optativa"
        } else {
            "obrigatória"
        };
        let prof = self
            .professor
            .as_ref()
            .map(|p| p.info())
            .unwrap_or("TBD".into());
        let plataforma = self
            .plataforma
            .as_ref()
            .map(|p| p.info())
            .unwrap_or("TBD".into());
        let sala = self.sala.as_ref().map(|p| p.info()).unwrap_or("TBD".into());
        let presenca = self
            .presenca
            .as_ref()
            .map(|p| p.info())
            .unwrap_or("TBD".into());

        formatdoc!(
            "
            *Sobre*
            _Tipo_: {tipo}
            _Prof_: {prof}
            _Plataforma_: {plataforma}
            _Sala_: {sala}
            _Presença_: {presenca}"
        )
    }

    pub fn info(&self) -> String {
        let titulo = self.titulo();
        let sobre = self.sobre();
        let avaliacoes = self
            .avaliacoes
            .as_ref()
            .map(|a| a.info())
            .unwrap_or_default();

        formatdoc!(
            "
            {titulo}

            {sobre}
            {}{avaliacoes}",
            if avaliacoes.is_empty() { "" } else { "\n" }
        )
    }
}

#[derive(Deserialize)]
struct Professor {
    nome: String,
    email: String,
}

impl Professor {
    pub fn info(&self) -> String {
        format!("{} ({})", self.nome, self.email)
    }
}

#[derive(Deserialize)]
struct Informacao {
    info: String,
    url: Option<String>,
}

impl Informacao {
    pub fn info(&self) -> String {
        match &self.url {
            Some(url) => format!("[{}]({})", self.info, url),
            None => format!("{}", self.info),
        }
    }
}

#[derive(Deserialize)]
struct Avaliacoes {
    criterio: Option<String>,
    provas: Vec<Avaliacao>,
    atividades: Vec<Avaliacao>,
}
impl Avaliacoes {
    pub fn info(&self) -> String {
        let provas = self
            .provas
            .iter()
            .map(|x| format!("  {}", x.info()))
            .collect::<Vec<String>>()
            .join("\n");

        let atividades = self
            .atividades
            .iter()
            .map(|x| format!("  {}", x.info()))
            .collect::<Vec<String>>()
            .join("\n");

        let criterio = self.criterio.as_deref().unwrap_or("TBD");

        formatdoc!(
            "
            _Criterio_: {criterio}
            _Provas_:
            {provas}
            _Atividades_:
            {atividades}"
        )
    }
}

#[derive(Deserialize)]
struct Avaliacao {
    nome: String,
    data: Option<String>,
    assunto: Option<String>,
}

impl Avaliacao {
    pub fn info(&self) -> String {
        let nome = &self.nome;
        let data = match &self.data {
            Some(d) => format!(": {d}"),
            None => "".into(),
        };
        let assunto = match &self.assunto {
            Some(a) => format!("- {a}"),
            None => "".into(),
        };
        format!("{nome}{data}{assunto}")
    }
}

#[derive(Deserialize)]
struct Aulas {
    segunda: Option<Aula>,
    terca: Option<Aula>,
    quarta: Option<Aula>,
    quinta: Option<Aula>,
    sexta: Option<Aula>,
}

#[derive(Deserialize)]
struct Aula {
    inicio: String,
    fim: String,
}
