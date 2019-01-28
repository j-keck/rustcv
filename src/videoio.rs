//! Class for video capturing from video files, image sequences or cameras.
use core::*;
use opencv_sys as ffi;

/// VideoCapture
#[derive(Debug)]
pub struct VideoCapture {
    inner: ffi::VideoCapture,
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        unsafe { ffi::VideoCapture_Close(self.inner) }
    }
}

impl VideoCapture {
    /// Default constructor.
    pub fn new() -> Self {
        VideoCapture {
            inner: unsafe { ffi::VideoCapture_New() },
        }
    }

    /// Opens a camera for video capturing
    pub fn open_device(&self, device: i32) -> bool {
        unsafe { ffi::VideoCapture_OpenDevice(self.inner, device) != 0 }
    }

    /// Returns true if video capturing has been initialized already.
    pub fn is_opened(&self) -> bool {
        unsafe { ffi::VideoCapture_IsOpened(self.inner) != 0 }
    }

    /// Grabs the next frame from video file or capturing device
    /// and writes it into the given 'Mat'.
    pub fn grab_into(&self, buf: &mut Mat) -> bool {
        unsafe { ffi::VideoCapture_Read(self.inner, buf.inner) != 0 }
    }

    /// Grabs the next frame from video file or capturing device
    pub fn grab(&self) -> Option<Mat> {
        let mut mat = Mat::new();
        if self.grab_into(&mut mat) {
            Some(mat)
        } else {
            None
        }
    }
}
