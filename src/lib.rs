use reqwest::Response;
use reqwest::Client;
use serde_json::{to_string_pretty, Value};
use slint::SharedString;

slint::include_modules!();
#[cfg_attr(target_arch = "wasm32",
wasm_bindgen::prelude::wasm_bindgen(start))]
#[cfg(target_arch = "wasm32")]
async fn main() {
    async_main().await;
}

pub async fn async_main() {
    let ui = AppWindow::new().unwrap();

    ui.on_request_get_result({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let method = ui.get_method();
            let query = ui.get_request();
            let body = ui.get_body();
            slint::spawn_local(async move {
                let body = handle_request(&method, &query, body.to_string()).await;
                ui.set_result(SharedString::from(body));
            }).unwrap();
        }
    });

    ui.on_request_updated({
        let ui_handle = ui.as_weak();
        move |text| {
            let ui = ui_handle.unwrap();
            ui.set_request(text);
        }
    });

    ui.on_method_updated({
        let ui_handle = ui.as_weak();
        move |text| {
            let ui = ui_handle.unwrap();
            ui.set_method(text);
        }
    });

    ui.on_body_updated({
        let ui_handle = ui.as_weak();
        move |text| {
            let ui = ui_handle.unwrap();
            ui.set_body(text);
        }
    });

    ui.run().unwrap();
}

async fn handle_request(method: &str, query: &str, body: String) -> String {
    let result: Result<Response, reqwest::Error>;
    let client = Client::new();
    match method {
        "GET" => {
            result = client.get(query).send().await;
        },
        "POST" => {
            result = client.post(query).body(body).header("Content-Type", "application/json").send().await;
        },
        "PUT" => {
            result = client.put(query).body(body).send().await;
        },
        _ => {panic!()}
    }

    let response = match result {
        Err(err) => return err.to_string(),
        Ok(res) => res
    };

    let body = match response.text().await {
        Ok(res) => res,
        Err(_) => return "Could not convert result to text".to_string()
    };

    let json: Value = match serde_json::from_str(body.as_str()) {
        Ok(result) => result,
        Err(_) => return body
    };

    to_string_pretty(&json).unwrap_or(body)
}
