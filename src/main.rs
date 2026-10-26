use three_d::*;

struct AppState {
    // your params live here: value: f32, selected: usize, ...
}

fn main() {
    let window = Window::new(WindowSettings {
        title: "TshaderZ".into(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl(); // (1)

    let camera = Camera::new_orthographic(
        window.viewport(),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
        2.0,
        0.1,
        10.0,
    );

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(vec![
            vec3(1000.0, -1000.0, 0.0),  // bottom right
            vec3(-1000.0, -1000.0, 0.0), // bottom left
            vec3(0.0, 1000.0, 0.0),      // top
        ]),
        colors: Some(vec![
            Srgba::new(255, 0, 0, 255), // bottom right
            Srgba::new(0, 255, 0, 255), // bottom left
            Srgba::new(0, 0, 255, 255), // top
        ]),
        ..Default::default()
    };

    let gm = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    let mut gui = three_d::GUI::new(&context); // (2)
    let mut _state = AppState { /* ... */ };

    window.render_loop(move |mut frame_input| {
        // (3)
        let mut left_width = 0.0f32;
        let mut bottom_height = 0.0f32;

        // (4) run egui FIRST, and measure the panels while building them
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ui| {
                use three_d::egui::*;
                Panel::left("left").show_inside(ui, |ui| {
                    // sliders that mutate state.params
                    ui.heading("test")
                });
                Panel::bottom("bottom").show_inside(ui, |ui| {
                    // shader selector
                    ui.heading("test")
                });
                left_width = frame_input.window_width as f32 - ui.available_width();
                bottom_height = frame_input.window_height as f32 - ui.available_height();
            },
        );

        // (5) the central rectangle, in GPU pixels

        let dpr = frame_input.device_pixel_ratio;
        let viewport = Viewport {
            x: (left_width * frame_input.device_pixel_ratio) as i32,
            y: (bottom_height * frame_input.device_pixel_ratio) as i32, // careful: flipped
            width: frame_input.window_width - (left_width * dpr) as u32,
            height: frame_input.window_height - (bottom_height * dpr) as u32,
        };

        // (6) draw the shader only inside that rectangle, then the UI on top
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render_partially(viewport.into(), &camera, &gm, &[])
            .write(|| gui.render())
            .unwrap();

        FrameOutput::default() // (7)
    });
}
