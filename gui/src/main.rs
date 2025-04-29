use gtk::{Application, ApplicationWindow, Button, glib, prelude::*};

const APP_ID: &str = "com.cordor.pbpctrlgui";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello, World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pixel Buds Pro Control")
        .child(&button)
        .build();

    window.present();
}
