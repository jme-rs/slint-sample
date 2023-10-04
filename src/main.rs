// use std::error::Error;

// use webapi::locale::Locale;

// use crate::webapi::wether::Forecast;

use slint::ComponentHandle;

mod webapi;

slint::include_modules!();

#[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
async fn main() -> Result<(), slint::PlatformError> {
    // let loc = Locale::new().get("千葉市".to_string()).await.unwrap();
    // println!("{:#?}", loc);
    // let forecast = Forecast::new(loc).fetch_json().await.unwrap();
    // println!("{}", serde_json::to_string_pretty(&forecast).unwrap());

    let ui = AppWindow::new()?;

    {
        let counter = ui.as_weak().unwrap();
        ui.on_increment(move || {
            counter.set_count(counter.get_count() + 1);
        });
    }
    {
        let counter = ui.as_weak().unwrap();
        ui.on_decrement(move || {
            counter.set_count(counter.get_count() - 1);
        });
    }
    ui.run()?;
    Ok(())
}
