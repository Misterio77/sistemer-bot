use sistemer_bot::{Disciplina, Result};

#[tokio::main]
async fn main() -> Result<()> {
    print!("{}", Disciplina::buscar_disciplina("bsi020", "SCC0541").await?.info());
    Ok(())
}
