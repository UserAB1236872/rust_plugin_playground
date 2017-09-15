extern crate common;
use common::{Shared, SharedTrait, SharedDropper};

use std::ops::Drop;

struct Private {
    ptr: *mut i32,
}

impl SharedTrait for Private {
    fn bar(&mut self) {
        unsafe {
            *self.ptr = 30;
        }
    }
}

impl Drop for Private {
    fn drop(&mut self) {
        unsafe {
            *self.ptr = 90;
        }
    }
}

impl SharedDropper for Private {}

#[no_mangle]
pub fn native_rust(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 2),
        None => None,
    }
}

#[no_mangle]
pub fn takes_ref(x: &mut i32) {
    x = 99
}

#[no_mangle]
pub fn pushes_vec(v: &mut Vec<usize>) {
    v.push(5);
}

#[no_mangle]
pub fn shared_struct(mut s: Shared) -> Shared {
    s.foo += 1;
    s.bar += 2;

    match s.x {
        Some(ref mut x) => *x -= 2,
        None => {}
    };

    s
}

#[no_mangle]
pub fn boxed_shared_trait(ptr: *mut i32) -> Box<SharedTrait> {
    Box::new(Private { ptr: ptr })
}
