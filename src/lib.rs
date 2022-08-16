#[macro_use]
extern crate glium;

extern crate image;

use glium::{glutin, Surface};
use std::{collections::HashSet, default, fs};

mod structs;

mod models;
use models::model_loader::{get_obj, Model};

///todo:
///bind:
/// camera
/// object
/// lights
/// mouse inputs
///split:
/// world object system
/// renderer
/// input
/// update loop
/// awake call
///other:
/// create promo game
/// sound
/// better lighting
/// texture and normal map loading
/// material creator

pub struct Scene {
    pub game_objects: Vec<Object>,
    pub input_manager: InputManager,
    pub main_camera: Camera,
    pub delta_time: f32,
    pub time_since_start: f32,
}

pub struct Object {
    pub name: String,
    pub model: Model,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub tick_update_func: fn(&mut Scene),
}

pub struct InputManager {
    pub pressed_scancodes: HashSet<u32>,
    pub modifiers: glutin::event::ModifiersState,
}

/// Camera object
#[derive(Clone, Copy)]
pub struct Camera {
    pub z_near: f32,
    pub z_far: f32,
    pub fov: f32,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub up_vector: [f32; 3],
    pub tick_update_func: fn(&mut Scene),
    pub on_awake: fn(&mut Scene)
}

pub fn engine(mut scene: Scene) {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("copper engine");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let model = get_obj("models/stanford_dragon_low.obj");

    let positions = glium::VertexBuffer::new(&display, &model.positions).unwrap();
    let normals = glium::VertexBuffer::new(&display, &model.normals).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &model.indices,
    )
    .unwrap();

    let vertex_shader_src = fs::read_to_string("shaders/vertex_shader.glsl").expect("");
    let vertex_shader_src = vertex_shader_src.as_str();

    let fragment_shader_src = fs::read_to_string("shaders/fragment_shader.glsl").expect("");
    let fragment_shader_src = fragment_shader_src.as_str();

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

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut prev_time = std::time::Instant::now();
    let start_time = std::time::Instant::now();

    (scene.main_camera.on_awake)(&mut scene);

    let mut t = 0.5;
    event_loop.run(move |ev, _, control_flow| {
        let now = std::time::Instant::now();
        scene.time_since_start = (now - start_time).as_secs_f32();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::ModifiersChanged(state) => {
                    scene.input_manager.modifiers = state;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => {}
                glutin::event::StartCause::Init => {}
                _ => return,
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::Key(input) => match input.state {
                    glutin::event::ElementState::Pressed => {
                        scene.input_manager.pressed_scancodes.insert(input.scancode);
                    }
                    glutin::event::ElementState::Released => {
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

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        scene.delta_time = (now - prev_time).as_secs_f32();

        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.2, 0.3, 1.0), 1.0);

        t += scene.delta_time * 0.5;
        // if t > 2.0 {
        //     t = -2.0;
        // }

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

        let position_matrix = [
            [t.cos(), 0.0, -t.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [t.sin(), 0.0, t.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        // let matrix = [
        //     [1.0, 0.0, 0.0, 0.0],
        //     [0.0, 1.0, 0.0, 0.0],
        //     [0.0, 0.0, 1.0, 0.0],
        //     [0.0, 0.0, 5.0, 1.0f32],
        // ];

        let light = [0.0, 10.0, -5.0f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let cam = scene.main_camera;

        (cam.tick_update_func)(&mut scene);

        let view = view_matrix(&cam.position, &cam.rotation, &cam.up_vector);

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! {
                    position_matrix: position_matrix, 
                    u_light: light, 
                    perspective: perspective, 
                    view: view
                },
                &params,
            )
            .unwrap();

        target.finish().unwrap();

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
