use std::io::Read;
use reqwest::blocking::Response;
use reqwest::blocking::Client;
use serde_json::Value;
use slint::SharedString;
slint::include_modules!();

fn handle_request(method: &str, query: &str, body: String) -> String {
    let result: Result<Response, reqwest::Error>;
    let client = Client::new();
    match method {
        "GET" => {
            result = client.get(query).send();
        },
        "POST" => {
            result = client.post(query).body(body).header("Content-Type", "application/json").send();
        },
        "PUT" => {
            result = client.put(query).body(body).send();
        },
        _ => {panic!()}
    }

    if let Err(err) = result {
        return err.to_string();
    }
    let mut res = result.unwrap();

    let mut body = String::new();
    println!("{}", method);
    if res.read_to_string(&mut body).is_err() {
        return "Failed to load result to string".to_string();
    }
    let json: Result<Value, serde_json::Error> = serde_json::from_str(body.as_str());
    if json.is_err() {
        return body;
    }
    let pretty = serde_json::to_string_pretty(&json.unwrap());

    if pretty.is_err() {
        return body;
    }

    pretty.unwrap()
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_get_result({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let method = ui.get_method();
            let query = ui.get_request();
            let body = ui.get_body();
            let body = handle_request(&method, &query, body.to_string());
            ui.set_result(SharedString::from(body));
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

    ui.run()
}
