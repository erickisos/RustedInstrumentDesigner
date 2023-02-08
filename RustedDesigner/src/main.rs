mod logic;
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
    Application, ApplicationWindow,
};
use logic::physics;

fn main() {
    let app = Application::builder()
        .application_id("com.tliwaka.RustedDesigner")
        .build();

    println!("{}", physics::MOLAR_MASS_CO2);
    let parameters = physics::Parameters::new();

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
