//! Class for video capturing from video files, image sequences or cameras.
use core::*;
use opencv_sys as ffi;
use std::ffi::CString;
use std::path::Path;

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

    /// FIXME!!
    pub fn open_file(&self, path: &Path) -> bool {
        let cstr = CString::new(path.to_string_lossy().to_string()).unwrap();
        let bytes = cstr.as_bytes_with_nul();
        unsafe { ffi::VideoCapture_Open(self.inner, bytes.as_ptr() as *const i8) != 0 }
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

/// VideoWriter
#[derive(Debug)]
pub struct VideoWriter {
    inner: ffi::VideoWriter,
}

impl Drop for VideoWriter {
    fn drop(&mut self) {
        unsafe { ffi::VideoWriter_Close(self.inner) }
    }
}

impl VideoWriter {
    /// Default Constructor
    pub fn new() -> Self {
        VideoWriter {
            inner: unsafe { ffi::VideoWriter_New() },
        }
    }

    ///
    pub fn open(
        &self,
        path: &Path,
        codec: &str,
        fps: f64,
        width: i32,
        height: i32,
        is_color: bool,
    ) {
        let path_cstring = CString::new(path.to_string_lossy().to_string()).unwrap();
        let path_bytes = path_cstring.as_bytes_with_nul();

        let codec_cstring = CString::new(codec.to_string()).unwrap();
        let codec_bytes = codec_cstring.as_bytes_with_nul();

        unsafe {
            ffi::VideoWriter_Open(
                self.inner,
                path_bytes.as_ptr() as *const i8,
                codec_bytes.as_ptr() as *const i8,
                fps,
                width,
                height,
                is_color,
            )
        };
    }

    /// Returns true if video writer has been initialized already.
    pub fn is_opened(&self) -> bool {
        unsafe { ffi::VideoWriter_IsOpened(self.inner) != 0 }
    }

    ///
    pub fn write(&self, frame: &Mat) {
        unsafe { ffi::VideoWriter_Write(self.inner, frame.inner) };
    }
}
