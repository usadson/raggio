// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raggio::{
    swap_chain::SwapChain, platform::Surface, Pixel, shader::{VertexShader2D, FragmentShader2D}, math::Vector2f, TriangleVertices
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Shader {
}

impl VertexShader2D for Shader {
    fn run(&self, position: Vector2f) -> Vector2f {
        position
    }
}

impl FragmentShader2D for Shader {
    fn run(&self) -> Pixel {
        Pixel::new(0x30, 0xA7, 0xF8, 0xFF)
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut surface = Surface::new(&window).unwrap();
    let mut swap_chain = SwapChain::new(window.inner_size().to_logical(1.0));

    let shader = Shader{};

    let vertices = [
        TriangleVertices::new(
            Vector2f::new(0.0, -0.5),
            Vector2f::new(-0.5, 0.5),
            Vector2f::new(0.5, 0.5),
        )
    ];

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            Event::RedrawRequested(_) => {
                swap_chain.clear(Pixel::BLACK);

                swap_chain.draw_rasterized(&vertices, &shader, &shader);

                swap_chain.present(&mut surface).unwrap();
            },
            _ => ()
        }
    });
}
