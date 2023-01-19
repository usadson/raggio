// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Pixel,
    math::Vector2f
};

pub trait VertexShader2D {

    fn run(&self, position: Vector2f) -> Vector2f;

}

pub trait FragmentShader2D {

    fn run(&self) -> Pixel;

}
