use anyhow::Result;

pub fn run(a: i32, b: i32) -> Result<()> {
    println!("{} + {} = {}", a, b, a + b);
    Ok(())
}

