use three_d::{egui::SizeHint::Scale, *};

struct AppState {
    // params live here: value: f32, selected: usize, ...
}

fn main() {
    let window = Window::new(WindowSettings {
        title: "TshaderZ".into(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let positions = VertexBuffer::new_with_data(
        &context,
        &[
            vec3(-1.0, -1.0, 0.0),
            vec3(3.0, -1.0, 0.0),
            vec3(-1.0, 3.0, 0.0),
        ],
    );

    const VERTEX_SRC: &str = include_str!("fullscreen.vert");
    let shader_sources = [
        include_str!("plasma.frag"), /* include_str!("next.frag"), ... */
    ];
    let programs: Vec<Program> = shader_sources
        .iter()
        .map(|frag| Program::from_source(&context, VERTEX_SRC, frag).unwrap())
        .collect();
    let mut selected = 0usize;

    let mut gui = three_d::GUI::new(&context);
    let mut _state = AppState { /* ... */ };
    let mut time_factor = 1.0;

    window.render_loop(move |mut frame_input| {
        let mut left_width = 0.0f32;
        let mut bottom_height = 0.0f32;
        let time_secs = (frame_input.accumulated_time * 0.001) as f32;

        //  run egui FIRST, and measure the panels while building them
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |ui| {
                use three_d::egui::*;
                Panel::left("left").show_inside(ui, |ui| {
                    // sliders that mutate state.params
                    ui.heading("Parameters");
                    ui.add(egui::Slider::new(&mut time_factor, 0.0..=100.0).text("Time Scalar"));
                });
                Panel::bottom("bottom").show_inside(ui, |ui| {
                    // shader selector
                    ui.heading("test")
                });
                // window_width and egui measurements are BOTH logical points here;
                // physical conversion happens once: * dpr (x/y) and via viewport.width/height
                left_width = frame_input.window_width as f32 - ui.available_width();
                bottom_height = frame_input.window_height as f32 - ui.available_height();
            },
        );

        // the central rectangle, in GPU pixels
        let dpr = frame_input.device_pixel_ratio;
        // physical framebuffer size: frame_input.viewport (window_width is LOGICAL)
        let viewport = Viewport {
            x: (left_width * dpr) as i32,
            y: (bottom_height * dpr) as i32, // careful: flipped
            width: frame_input
                .viewport
                .width
                .saturating_sub((left_width * dpr) as u32),
            height: frame_input
                .viewport
                .height
                .saturating_sub((bottom_height * dpr) as u32),
        };

        let program = &programs[selected];
        program.use_uniform("time", time_secs * time_factor);
        program.use_uniform(
            "resolution",
            vec2(viewport.width as f32, viewport.height as f32),
        );
        program.use_vertex_attribute("position", &positions);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .write::<CoreError>(|| {
                program.draw_arrays(RenderStates::default(), viewport, 3);
                gui.render()
            })
            .unwrap();
        FrameOutput::default() //
    });
}
