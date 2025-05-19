use std::os::raw::{c_char, c_int, c_double};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Rotation,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Twist {
    pub linear: Vector3,
    pub angular: Vector3,
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GeometryType {
    Sphere = 0,
    Box = 1,
    Cylinder = 2,
    Capsule = 3,
    Mesh = 4,
}

#[derive(Debug, Clone)]
pub struct VisualInfo {
    pub link_name: String,
    pub geometry_type: GeometryType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Box {
    pub dim: Vector3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cylinder {
    pub radius: f64,
    pub length: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Capsule {
    pub radius: f64,
    pub length: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    pub filename: *const c_char,
    pub scale: Vector3,
}


