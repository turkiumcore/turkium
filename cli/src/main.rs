#![allow(non_snake_case)]
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() {}
    } else {
        use Turkium_cli_lib::{Turkium_cli, TerminalOptions};

        #[tokio::main]
        async fn main() {
            let result = Turkium_cli(TerminalOptions::new().with_prompt("$ "), None).await;
            if let Err(err) = result {
                println!("{err}");
            }
        }
    }
}
