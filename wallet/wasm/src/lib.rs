#![allow(non_snake_case)]
use turkium_cli_lib::turkium_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_turkium_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    turkium_cli(options, None).await?;
    Ok(())
}
