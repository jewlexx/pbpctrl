use gtk::{Application, ApplicationWindow, Button, glib, prelude::*};
use maestro::{hdlc::Codec, protocol::utils, pwrpc::client::Client};

const APP_ID: &str = "com.cordor.pbpctrlgui";

mod bt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Application::builder().application_id(APP_ID).build();

    // set up session
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    // set up device
    let dev = {
        tracing::debug!("no device specified, searching for compatible one");
        bt::find_maestro_device(&adapter).await?
    };

    // set up profile
    let stream = bt::connect_maestro_rfcomm(&session, &dev).await?;

    // set up codec
    let codec = Codec::new();
    let stream = codec.wrap(stream);

    // set up RPC client
    let mut client = Client::new(stream);
    let handle = client.handle();

    // resolve channel
    let channel = utils::resolve_channel(&mut client).await?;

    app.connect_activate(build_ui);

    app.run();

    Ok(())
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
