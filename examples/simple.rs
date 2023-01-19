// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Instant;

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
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                swap_chain.resize_with_clear_color(size.to_logical(1.0), Pixel::BLACK);
            },
            Event::RedrawRequested(_) => {
                println!("[Redraw]");

                let begin = Instant::now();
                swap_chain.clear(Pixel::BLACK);
                println!("  Timing of Swap Chain Clear: {} ms", begin.elapsed().as_micros() as f32 / 1000.0);

                let begin = Instant::now();
                swap_chain.draw_rasterized(&vertices, &shader, &shader);
                println!("  Timing of Draw: {} ms", begin.elapsed().as_micros() as f32 / 1000.0);

                let begin = Instant::now();
                swap_chain.present(&mut surface).unwrap();
                println!("  Timing of Present: {} ms", begin.elapsed().as_micros() as f32 / 1000.0);
            },
            _ => ()
        }
    });
}
