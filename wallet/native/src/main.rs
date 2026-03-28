#![allow(non_snake_case)]
use Turkium_cli_lib::{TerminalOptions, Turkium_cli};

#[tokio::main]
async fn main() {
    let result = Turkium_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
