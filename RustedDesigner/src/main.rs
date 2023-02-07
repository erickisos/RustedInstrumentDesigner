mod lib;
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::WidgetExt,
    Application, ApplicationWindow,
};
use lib::note::cents;
use lib::physics::Parameters;

fn main() {
    let app = Application::builder()
        .application_id("com.tliwaka.RustedDesigner")
        .build();

    let f1 = 10.0;
    let f2 = 32.0;

    let parameters = Parameters {
        temperature: 29.0,
        pressure: 100.0,
        molar_co2: 0.0,
        molar_water_vapour: 10.0,
        humidity_saturation: 0.8,
    };
    let calculated_cents: f64 = cents(&f1, &f2);
    println!("{}", calculated_cents);

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
