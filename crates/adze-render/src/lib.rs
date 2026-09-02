//! The viewport: wgpu render pipeline, subdivision compute shaders, picking.
//!
//! Designed to the WebGPU feature subset first (D4); the native backends get
//! the same shaders. This crate may depend on wgpu; nothing below it may
//! (D18). Empty until M1's headless viewer.
