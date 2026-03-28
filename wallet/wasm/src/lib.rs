#![allow(non_snake_case)]
use Turkium_cli_lib::Turkium_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_Turkium_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    Turkium_cli(options, None).await?;
    Ok(())
}
