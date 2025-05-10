use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Hello, {}!", name);
    Ok(())
}

