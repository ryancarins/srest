use srest::async_main;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    async_main().await;
}

