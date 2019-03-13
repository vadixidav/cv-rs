//! Provide the type that encapsulates all the parameters of the SIFT extraction algorithm
use super::*;
use core::*;
use std::ffi::c_void;
use std::os::raw::*;
use *;

/// Speeded up robust features extractor.
#[derive(Debug)]
pub struct SIFT {
    value: *mut c_void,
}

impl SIFT {
    /// Creates a new maximally stable extremal region extractor criteria.
    pub fn new(
        features: c_int,
        octave_layers: c_int,
        contrast_threshold: f64,
        edge_threshold: f64,
        sigma: f64,
    ) -> Self {
        let sift =
            unsafe { native::cvsys_sift_new(features, octave_layers, contrast_threshold, edge_threshold, sigma) };
        SIFT { value: sift }
    }
}

impl Drop for SIFT {
    fn drop(&mut self) {
        unsafe {
            native::cvsys_sift_drop(self.value);
        }
    }
}

/// Builder that provides defaults for MSER
#[derive(Debug, Copy, Clone, Default)]
pub struct SIFTBuilder {
    features: Option<c_int>,
    octave_layers: Option<c_int>,
    contrast_threshold: Option<f64>,
    edge_threshold: Option<f64>,
    sigma: Option<f64>,
}

impl SIFTBuilder {
    /// Replace current features with specified value
    pub fn features(mut self, value: c_int) -> Self {
        self.features = Some(value);
        self
    }

    /// Replace current octave_layers with specified value
    pub fn octave_layers(mut self, value: c_int) -> Self {
        self.octave_layers = Some(value);
        self
    }

    /// Replace current contrast_threshold with specified value
    pub fn contrast_threshold(mut self, value: f64) -> Self {
        self.contrast_threshold = Some(value);
        self
    }

    /// Replace current edge_threshold with specified value
    pub fn edge_threshold(mut self, value: f64) -> Self {
        self.edge_threshold = Some(value);
        self
    }

    /// Replace current sigma with specified value
    pub fn sigma(mut self, value: f64) -> Self {
        self.sigma = Some(value);
        self
    }
}

impl Into<SIFT> for SIFTBuilder {
    fn into(self) -> SIFT {
        SIFT::new(
            self.features.unwrap_or(0),
            self.octave_layers.unwrap_or(3),
            self.contrast_threshold.unwrap_or(0.04),
            self.edge_threshold.unwrap_or(10.0),
            self.sigma.unwrap_or(1.6),
        )
    }
}

impl Feature2D for SIFT {
    fn detect_and_compute(&self, image: &Mat, mask: &Mat) -> (Vec<KeyPoint>, Mat) {
        unsafe {
            let mut keypoints: native::cvsys_CVec<native::cvsys_KeyPoint> = std::mem::zeroed();
            let descriptors = native::cvsys_mat_new();
            native::cvsys_sift_detect_and_compute(
                self.value,
                image.inner,
                mask.inner,
                &mut keypoints,
                descriptors,
                false,
            );
            (keypoints.into(), Mat::from_raw(descriptors))
        }
    }
}
