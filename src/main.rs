// import stuff from eframe extern crate

use eframe::egui;

// main entry point of the program

fn main() -> Result<(), eframe::Error> {
    // default window settings
    let options = eframe::NativeOptions::default();
    // starting the egui app
    eframe::run_native(
        "TshaderZ",
        options,
        //create app state
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// the struct here holds the application's data state

#[derive(Default)] // makes it easier to create a starting instance i dont really know what that means but i remember it from class somewhere
struct MyApp {
    label: String, // for text inputs
    value: f32,    // value inputs
                   // add more fields as we go
}

// implement the efraim::app trait for MyApp telling eframe how to run it
impl eframe::App for MyApp {
    // ui method is called every frame to draw the ui (in newer versions of eframe, this replaced `update`)
    // '&mut self' allows the method to modify MyApp's fields, mutable self reference thanks
    // Wheelock!!!
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // central for ui
        //ui our tool to add widgets
        egui::Panel::left("left").show(ui, |ui| {
            ui.heading("My Application");
            ui.horizontal(|ui| {
                ui.label("Testing testing");
                ui.text_edit_singleline(&mut self.label);
            });

            // adding a slider linked to self.value
            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // adding a button
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            //display the current state in a label
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });

        egui::Panel::bottom("bottom").show(ui, |ui| {
            ui.heading("testing bottom pannel and long line of text i guess idk")
        });
    }
}
