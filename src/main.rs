use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

pub type URDFRobotHandle = *mut c_void;

unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn do_something(a: i32, b: f32) -> i32;
    fn urdf_try() -> ();

    fn urdf_parse(xml: *const c_char) -> URDFRobotHandle;
    fn urdf_free(robot: *mut c_void);
    fn urdf_visual_count(robot: *mut c_void) -> c_int;
}


const XML: &str = r#"<?xml version="1.0">
                    <robot name="test">
                    <link name="base_link">
                    <visual>
                    <geometry>
                    <cylinder length="0.6" radius="0.25"/>
                    </geometry>
                    </visual>
                    <visual>
                    <geometry>
                    <sphere radius="0.5"/>
                    </geometry>
                    </visual>
                    <visual>
                    <geometry>
                    <box size="0.1 0.2 0.3"/>
                    </geometry>
                    </visual>
                    <visual>
                    <geometry>
                    <mesh scale="0.7 0.8 0.9" filename="test.obj"/>
                    </geometry>
                    </visual>
                    </link>
                    </robot>"#;

fn main() {
    unsafe {
        let result = add(3, 4);
        println!("Result from C++: {}", result);
        let result = do_something(3, 4.5);
        println!("Result from C++: {}", result);
        urdf_try();

        let xml = CString::new(XML).unwrap();
        let robot = urdf_parse(xml.as_ptr());

        let count = urdf_visual_count(robot);
        println!("Visual count for base_link: {}", count);

        urdf_free(robot);
    }
}

#[cfg(test)]
mod c_bind_test {
    use super::*;

    #[test]
    fn basic_urdf_greeting() {
        unsafe { urdf_try(); }
    }

    #[test]
    fn basic_test() {
        let result = unsafe { add(3, 4) };
        assert_eq!(result, 7);
        let result = unsafe { do_something(3, 4.5) };
        assert_eq!(result, 13);
    }

    #[test]
    fn urdf_test() {
        unsafe {
            let xml = CString::new(XML).unwrap();
            let robot = urdf_parse(xml.as_ptr());
            let count = urdf_visual_count(robot);
            assert_eq!(count, 4);
            urdf_free(robot);
        }
    }
}