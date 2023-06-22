pub mod logic;

use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
    Application, ApplicationWindow,
};

fn main() {
    let app = Application::builder()
        .application_id("com.tliwaka.RustedDesigner")
        .build();

    app.connect_activate(|app| {
        ApplicationWindow::builder()
            .application(app)
            .title("Rusted Designer!")
            .default_height(480)
            .default_width(640)
            .build()
            .show();
    });

    app.run();
}
