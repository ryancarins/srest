use std::io::Read;
use serde_json::Value;
use slint::SharedString;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_get_result({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut res = reqwest::blocking::get(ui.get_request().as_str()).unwrap();
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            let json: Value = serde_json::from_str(body.as_str()).unwrap();
            body = serde_json::to_string_pretty(&json).unwrap();
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

    ui.run()
}
