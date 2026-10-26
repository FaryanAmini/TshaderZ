// import stuff from eframe extern crate 

use eframe::egui;

// main entry point of the program

fn main() -> Result<(),eframe::Error>{
    // default window settings
    let options = eframe::NativeOptions::default();
    // starting the egui app
    eframe::run_native(
        "TshaderZ",
        options,
        //create app state
        Box::new(|_cc| Box::new(MyApp::default())),
        )
}

// the struct here holds the application's data state

#[derive(Default)] // makes it easier to create a starting instance i dont really know what that means but i remember it from class somewhere
struct MyApp{
    label: String, // for text inputs
    value: f32, // value inputs
    // add more fields as we go
}



