use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("LPP", "abi/erc721.json")?
        .generate()?
        .write_to_file("src/abi/lpp.rs")?;

    Ok(())
}
