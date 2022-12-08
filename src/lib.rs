use anyhow::{anyhow, Result};

pub fn read_input() -> Result<String> {
    let filename = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input filename"))?;

    Ok(std::fs::read_to_string(&filename)?)
}
