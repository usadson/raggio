// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[cfg(windows)]
pub mod win32;

#[cfg(windows)]
pub use win32::Surface;
