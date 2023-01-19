// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use math::Vector2f;

pub mod math;
pub mod platform;
pub mod shader;
pub mod swap_chain;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Pixel {

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { alpha, red, green, blue }
    }

    /// The color black.
    pub const BLACK: Self = Pixel::new(0x00, 0x00, 0x00, 0xF);

    /// The color red.
    pub const RED: Self = Pixel::new(0xFF, 0xFF, 0xFF, 0xFF);

}

#[repr(packed)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TriangleVertices {
    pub a: Vector2f,
    pub b: Vector2f,
    pub c: Vector2f,
}

impl TriangleVertices {
    pub fn new(a: Vector2f, b: Vector2f, c: Vector2f) -> Self {
        Self { a, b, c }
    }
}
