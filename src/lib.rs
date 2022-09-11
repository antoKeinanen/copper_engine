#[macro_use]
extern crate glium;
extern crate image;

use audio::AudioSource;
use egui::vec2;
use glium::{
    glutin::{
        self,
        event::{self, Event, WindowEvent},
        event_loop::{self, EventLoop},
        window::WindowBuilder,
    },
    Display, Surface,
};
use soloud::{AudioExt, Soloud};
use std::fs;
use structs::scene::Scene;

pub use structs::*;

pub mod audio;
pub mod object;
pub mod structs;

/// Blank template for tick update. Does not do anything, but fulfills the type requirements.
pub fn blank_tick_update(_scene: &mut Scene) {}
/// Blank template for on awake. Does not do anything, but fulfills the type requirements.
pub fn blank_on_awake(_scene: &mut Scene) {}

/// Main loop of the engine.
pub fn engine(mut scene: Scene) {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_title("copper engine");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = Display::new(wb, cb, &event_loop).expect("Failed to init display");

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    let sl = Soloud::default().unwrap();

    let vertex_shader_src = fs::read_to_string("shaders/vertex_shader.glsl").expect("");
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/fragment_shader.glsl").expect("");
    let fragment_shader_src = fragment_shader_src.as_str();

    for mut object in &mut scene.game_objects {
        let model = &object.model;

        object.vertices = Some(glium::VertexBuffer::new(&display, &model.positions).unwrap());
        object.normals = Some(glium::VertexBuffer::new(&display, &model.normals).unwrap());
        object.indices = Some(
            glium::IndexBuffer::new(
                &display,
                glium::index::PrimitiveType::TrianglesList,
                &model.indices,
            )
            .unwrap(),
        );

        object.program = Some(
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        );
    }

    for i in 0..scene.game_objects.len() {
        let object = &scene.game_objects[i];
        (object.on_awake)(&mut scene);
    }

    // let image = image::load(
    //     Cursor::new(&include_bytes!("../textures/test.jpg")),
    //     image::ImageFormat::Jpeg,
    // )
    // .unwrap()
    // .to_rgba8();
    // let image_dimensions = image.dimensions();
    // let image =
    //     glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    // let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    //scene.player.as_ref().unwrap().play(&scene.audio_sources[0].sound);

    let mut prev_time = std::time::Instant::now();
    let start_time = std::time::Instant::now();

    (scene.main_camera.on_awake)(&mut scene);

    let mut drawn_frames = 0;
    event_loop.run(move |ev, _, control_flow| {
        let now = std::time::Instant::now();
        scene.time_since_start = (now - start_time).as_secs_f32();

        let mut redraw = || {
            let quit = false;

            let repaint_after = egui_glium.run(&display, |egui_ctx| {
                egui::Window::new("debug")
                    .fixed_size(vec2(300f32, 300f32))
                    .show(egui_ctx, |ui| {
                        ui.label(format!(
                            "Fps: {}",
                            drawn_frames / ((now - start_time).as_secs() + 1)
                        ));
                        ui.label(format!("Frame: {}", drawn_frames));
                        ui.label(format!("Delta time: {:.10}", scene.delta_time));

                        ui.separator();

                        ui.collapsing(
                            format!("Loaded objects: {}", scene.game_objects.len()),
                            |ui| {
                                for i in 0..scene.game_objects.len() {
                                    let object = &scene.game_objects[i];

                                    ui.collapsing(object.name.as_str(), |ui| {
                                        ui.label(format!(
                                            "Translation: {:.3?}",
                                            object.translation
                                        ));
                                        ui.label(format!("Rotation: {:.3?}", object.rotation));
                                        ui.label(format!("Scale: {:.3?}", object.scale));

                                        ui.separator();

                                        ui.label(format!(
                                            "Vertices: {}",
                                            object.model.positions.len()
                                        ));
                                        ui.label(format!(
                                            "Indices: {}",
                                            object.model.indices.len()
                                        ));
                                        ui.label(format!(
                                            "Normals: {}",
                                            object.model.normals.len()
                                        ));
                                    });
                                }
                            },
                        );

                        ui.separator();

                        ui.collapsing("Audio", |ui| {
                            for i in  0..scene.audio_sources.len() {
                                let audio_source = &scene.audio_sources[i];
                                match audio_source {
                                    AudioSource::Local(audio_source) => {
                                        ui.collapsing("Local source", |ui| {
                                            ui.label(format!("Position: {:.3?}", audio_source.position));
                                            ui.label(format!("Triggered: {:.3?}", audio_source.triggered));
                                            ui.label(format!("Volume: {:.3?}", audio_source.volume));
                                            ui.label(format!("Amplifier: {:.3?}", audio_source.amplifier));
                                        });
                                    },
                                    _ => {},
                                }
                            }
                        });
                    });
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if repaint_after.is_zero() {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                glutin::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                let next_frame_time =
                    std::time::Instant::now() + std::time::Duration::from_nanos(0); //<- FPS cap
                *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);
                scene.delta_time = (now - prev_time).as_secs_f32();

                let mut target = display.draw();
                target.clear_color_and_depth((0.1, 0.2, 0.3, 1.0), 1.0);

                let perspective = {
                    let (width, height) = target.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;

                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;

                    let f = 1.0 / (fov / 2.0).tan();

                    [
                        [f * aspect_ratio, 0.0, 0.0, 0.0],
                        [0.0, f, 0.0, 0.0],
                        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                    ]
                };

                let light = [0.0, 10.0, -5.0f32];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    backface_culling:
                        glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
                    ..Default::default()
                };

                let cam = scene.main_camera;

                (cam.tick_update_func)(&mut scene);

                let view = view_matrix(&cam.position, &cam.rotation, &cam.up_vector);

                // under gui layer

                for i in 0..scene.game_objects.len() {
                    let object = &scene.game_objects[i];
                    (object.tick_update_func)(&mut scene);
                }

                for object in &mut scene.game_objects {
                    let [tx, ty, tz] = object.translation;
                    let [sx, sy, sz] = object.scale;
                    let [rx, ry, rz] = object.rotation;

                    //https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
                    let position_matrix = [
                        [
                            sx * rz.cos() * ry.cos(),
                            rz.cos() * ry.sin() * rx.sin() - rz.sin() * rx.cos(),
                            rz.cos() * ry.sin() * rx.cos() + rz.sin() * rx.sin(),
                            0.0,
                        ],
                        [
                            rz.sin() * ry.cos(),
                            sy * rz.sin() * ry.sin() * rx.sin() + rz.cos() * rx.cos(),
                            rz.sin() * ry.sin() * rx.sin() - rz.cos() * rx.sin(),
                            0.0,
                        ],
                        [
                            -ry.sin(),
                            ry.cos() * rx.sin(),
                            sz * ry.cos() * rx.cos(),
                            0.0,
                        ],
                        [tx, ty, tz, 1.0f32],
                    ];

                    let ambient = object.material.ambient_color;
                    let mut diffuse = [0.0f32; 3];
                    for i in 0..3 {
                        diffuse[i] = ambient[i] * 2.0;
                    }

                    target
                        .draw(
                            (
                                object.vertices.as_ref().unwrap(),
                                object.normals.as_ref().unwrap(),
                            ),
                            object.indices.as_ref().unwrap(),
                            &object.program.as_ref().unwrap(),
                            &uniform! {
                                position_matrix: position_matrix,
                                u_light: light,
                                u_ambient_color: ambient,
                                u_diffuse_color: diffuse,
                                perspective: perspective,
                                view: view
                            },
                            &params,
                        )
                        .unwrap();
                }

                for i in 0..scene.audio_sources.len() {
                    let audio_source = &mut scene.audio_sources[i];
                    match audio_source {
                        AudioSource::Local(audio_source) => {
                            if audio_source.triggered {
                                audio_source.sound.set_volume(audio_source.volume);
                                let am = audio_source.amplifier;

                                let [x, y, z] = audio_source.position;
                                let [cx, cy, cz] = scene.main_camera.position;
                                let [dx, dy, dz] = [(x - cx) * am, (y - cy) * am, (z - cz) * am];

                                sl.play_3d(&audio_source.sound, dx, dy, dz);
                                audio_source.triggered = false;
                            }
                        }
                        AudioSource::Global(audio_source) => {
                            if audio_source.triggered {
                                audio_source.sound.set_volume(audio_source.volume);

                                sl.play(&audio_source.sound);
                                audio_source.triggered = false;
                            }
                        }
                    }
                }

                egui_glium.paint(&display, &mut target);

                // over gui layer

                target.finish().unwrap();

                drawn_frames += 1;
            }
        };

        match ev {
            Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                WindowEvent::ModifiersChanged(m) => {
                    scene.input_manager.modifiers = m;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    scene.input_manager.mouse_position = [position.x, position.y];
                    let event_response = egui_glium.on_event(&event);

                    if event_response {
                        display.gl_window().window().request_redraw();
                    }
                }
                _ => {
                    let event_response = egui_glium.on_event(&event);

                    if event_response {
                        display.gl_window().window().request_redraw();
                    }
                }
            },
            Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => {
                    display.gl_window().window().request_redraw();
                }
                event::StartCause::Init => {}
                _ => return,
            },
            Event::DeviceEvent { event, .. } => match event {
                event::DeviceEvent::Key(input) => match input.state {
                    event::ElementState::Pressed => {
                        scene.input_manager.pressed_scancodes.insert(input.scancode);
                    }
                    event::ElementState::Released => {
                        scene
                            .input_manager
                            .pressed_scancodes
                            .remove(&input.scancode);
                    }
                },
                _ => {}
            },
            _ => {}
        }
        prev_time = now;
    });
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
