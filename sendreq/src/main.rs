slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_send_request(|method, request_url, header, body| {
        println!("{method}");
        println!("{request_url}");
        println!("{header}");
        println!("{body}");
        (move || {
            let ui_handle = ui.as_weak();
            let inner_ui = ui_handle.unwrap();
            inner_ui.set_response(format!("{}: {}", method, request_url).into());
        })();
        // resp = format!("{}: {}", method, request_url);
    });

    // ui.set_response(resp.into());

    ui.run()
}
