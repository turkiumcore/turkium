#![allow(non_snake_case)]
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() {}
    } else {
        use turkium_cli_lib::{turkium_cli, TerminalOptions};

        #[tokio::main]
        async fn main() {
            let result = turkium_cli(TerminalOptions::new().with_prompt("$ "), None).await;
            if let Err(err) = result {
                println!("{err}");
            }
        }
    }
}
