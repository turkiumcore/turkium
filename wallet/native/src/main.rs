#![allow(non_snake_case)]
use turkium_cli_lib::{TerminalOptions, turkium_cli};

#[tokio::main]
async fn main() {
    let result = turkium_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
