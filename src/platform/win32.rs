// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{HWND, GetLastError};
use windows::Win32::Graphics::Gdi::{
    GetDC, StretchDIBits, ValidateRect, BITMAPINFOHEADER, BI_BITFIELDS, DIB_RGB_COLORS, HDC,
    RGBQUAD, SRCCOPY, GDI_ERROR,
};
use windows::Win32::UI::WindowsAndMessaging::IsWindow;
use winit::window::Window;

use crate::{Pixel, swap_chain};

#[derive(Debug)]
pub enum SurfaceCreationError {

    /// The given window handle is not valid.
    InvalidHandle,

    /// The current platform is not supported.
    UnsupportedPlatform,

}

#[derive(Debug)]
pub enum SurfacePresentationError {

    /// The given image to present was too large to present.
    ImageTooLarge,

}

const NULL_QUAD: RGBQUAD = RGBQUAD {
    rgbBlue: 0,
    rgbGreen: 0,
    rgbRed: 0,
    rgbReserved: 0,
};

const BITMAP_COLOR_DESCRIPTORS: [RGBQUAD; 3] = [
    RGBQUAD {
        rgbBlue: 0xff,
        ..NULL_QUAD
    },
    RGBQUAD {
        rgbGreen: 0xff,
        ..NULL_QUAD
    },
    RGBQUAD {
        rgbRed: 0xff,
        ..NULL_QUAD
    },
];

/// The BITMAPINFO structure defines the dimensions and color information for a DIB.
/// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfo
#[repr(C)]
struct BitmapColoredInfo {
    #[allow(dead_code)]
    header: BITMAPINFOHEADER,

    #[allow(dead_code)]
    descriptors: [RGBQUAD; 3]
}

pub struct Surface {
    window: HWND,
    device_context: HDC,
}

impl Surface {

    /// Creates a new surface for the given window.
    pub fn new(window: &Window) -> Result<Self, SurfaceCreationError> {
        let RawWindowHandle::Win32(handle) = window.raw_window_handle() else {
            return Err(SurfaceCreationError::UnsupportedPlatform);
        };

        let hwnd = HWND(handle.hwnd as _);

        if hwnd == HWND::default() {
            return Err(SurfaceCreationError::InvalidHandle);
        }

        unsafe {
            if !IsWindow(hwnd).as_bool() {
                return Err(SurfaceCreationError::InvalidHandle);
            }
        }

        let device_context = unsafe {
            GetDC(hwnd)
        };

        Ok(Self {
            window: hwnd,
            device_context
        })
    }

    pub fn present(&mut self, buffer: &[Pixel], extent: swap_chain::Extent)
            -> Result<(), SurfacePresentationError> {
        if extent.width > i32::max_value() as _ {
            return Err(SurfacePresentationError::ImageTooLarge);
        }

        if extent.height > i32::max_value() as _ {
            return Err(SurfacePresentationError::ImageTooLarge);
        }

        let bitmap_info_header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as _,
            biWidth: extent.width as _,
            biHeight: -(extent.height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_BITFIELDS,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        let bitmap_info = BitmapColoredInfo {
            header: bitmap_info_header,
            descriptors: BITMAP_COLOR_DESCRIPTORS,
        };

        let scan_lines = unsafe {
            StretchDIBits(
                self.device_context,
                0,
                0,
                extent.width as _,
                extent.height as _,
                0,
                0,
                extent.width as _,
                extent.height as _,
                Some(buffer.as_ptr().cast()),
                &bitmap_info as *const BitmapColoredInfo as *const _,
                DIB_RGB_COLORS,
                SRCCOPY,
            )
        };

        // https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchdibits#return-value

        if scan_lines == GDI_ERROR {
            panic!("StretchDIBits failed with GDI_ERROR, last error: {:?}", unsafe {
                GetLastError()
            });
        }

        if scan_lines == 0 {
            panic!("StretchDIBits failed with 0, last error: {:?}", unsafe {
                GetLastError()
            });
            // TODO return SurfacePresentationError instead
        }

        unsafe {
            let result = ValidateRect(self.window, None);
            assert!(result.as_bool(), "ValidateRect failed");
        };

        Ok(())
    }

}
