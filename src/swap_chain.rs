// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::dpi::LogicalSize;

use crate::{
    platform::{
        Surface, win32::SurfacePresentationError
    },
    Pixel,
    shader::{
        FragmentShader2D,
        VertexShader2D,
    }, TriangleVertices, math::{Triangle2D, Vector2f, Vector2}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Extent {
    pub width: usize,
    pub height: usize,
}

pub struct SwapChain {
    extent: Extent,
    buffer: Vec<Pixel>,
}

/// Creates a pixel buffer for the given size.
fn create_pixel_buffer(width: usize, height: usize, color: Pixel) -> Vec<Pixel> {
    let mut vec = Vec::new();
    vec.resize(width * height, color);
    vec
}

impl SwapChain {

    pub fn new(size: LogicalSize<u32>) -> Self {
        Self {
            extent: Extent {
                width: size.width as _,
                height: size.height as _
            },
            buffer: create_pixel_buffer(size.width as _, size.height as _, Pixel::BLACK),
        }
    }

    pub fn clear(&mut self, color: Pixel) {
        self.buffer.fill(color)
    }

    pub fn draw_rasterized(&mut self, vertices: &[TriangleVertices], vertex_shader: &dyn VertexShader2D,
            fragment_shader: &dyn FragmentShader2D) {

        for vertex_triple in vertices {
            let va = vertex_shader.run(vertex_triple.a);
            let vb = vertex_shader.run(vertex_triple.b);
            let vc = vertex_shader.run(vertex_triple.c);

            let coord_a = self.vertex_to_pixel_position(va);
            let coord_b = self.vertex_to_pixel_position(vb);
            let coord_c = self.vertex_to_pixel_position(vc);

            println!("pixel position {:?} {:?} {:?}", coord_a, coord_b, coord_c);

            let triangle = Triangle2D(coord_a, coord_b, coord_c);
            let enclosing_rect = triangle.encapsulating_rectangle();

            for y in enclosing_rect.y_range() {
                for x in enclosing_rect.x_range() {
                    let point = Vector2::new(x, y);
                    if triangle.hit_test(point) {
                        if self.is_point_inside(point) {
                            let color = fragment_shader.run();
                            self.set_pixel(point, color);
                        }
                    }
                }
            }
        }
    }

    pub fn present(&self, surface: &mut Surface) -> Result<(), SurfacePresentationError> {
        surface.present(&self.buffer, self.extent)
    }

    /// Checks if a given point is inside the render area.
    #[inline]
    fn is_point_inside(&self, point: Vector2<i32>) -> bool {
        point.x >= 0 && point.y >= 0
            && (point.x as usize) < self.extent.width
            && (point.y as usize) < self.extent.height
    }

    /// Resizes the swap chain image, using the specified color as the clear
    /// color. To actually resize the swap chain and it's colors inside (without
    /// clearing them) would be a waste, since we can just redraw instead.
    pub fn resize_with_clear_color(&mut self, size: LogicalSize<u32>, color: Pixel) {
        self.extent = Extent {
            width: size.width as _,
            height: size.height as _,
        };
        self.buffer = create_pixel_buffer(size.width as _, size.height as _, color)
    }

    #[inline]
    fn set_pixel(&mut self, point: Vector2<i32>, color: Pixel) {
        let point = Vector2::new(point.x as usize, point.y as usize);
        self.buffer[point.y * self.extent.width + point.x] = color;
    }

    fn vertex_to_pixel_position(&self, vertex: Vector2f) -> Vector2<i32> {
        let x = ((vertex.x + 1.0) / 2.0 * self.extent.width as f32).round() as _;
        let y = ((vertex.y + 1.0) / 2.0 * self.extent.height as f32).round() as _;
        Vector2::new(x, y)
    }

}
