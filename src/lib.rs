use lotus_script::prelude::*;

#[derive(Default)]
pub struct MyScript {}

script!(MyScript);

impl Script for MyScript {
    fn init(&mut self) {
        // DE: Wird einmal beim Laden des Scripts aufgerufen.
        // EN: Called once when the script is loaded.
        log::info!("Hello, world!");
    }

    fn tick(&mut self) {
        // DE: Wird jeden Simulations-Frame aufgerufen.
        // EN: Called every simulation frame.
    }
}
