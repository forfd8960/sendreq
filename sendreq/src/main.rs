slint::include_modules!();

mod request;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_send_request(move |method, request_url, header, body| {
        println!("{method}");
        println!("{request_url}");
        println!("{header}");
        println!("{body}");

        let inner_ui = ui_handle.unwrap();
        inner_ui.set_response(format!("{}: {}: {}", method, request_url, header).into());
    });

    // ui.set_response(resp.into());

    ui.run()
}
