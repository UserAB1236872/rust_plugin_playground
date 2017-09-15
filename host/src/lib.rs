extern crate libloading;
extern crate common;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use std::sync::{Once, ONCE_INIT};
    use common::{Shared, SharedTrait, SharedDropper};
    use libloading::{Library,Symbol};

    lazy_static! {
        static ref PLUGIN: Library = 
                    Library::new("../../plugin/target/debug/plugin.dll")
                        .expect("Can't load library");
                
    }

    #[test]
    fn test_native() {
        let native_rust = unsafe {
            PLUGIN.get::<fn (Option<i32>)->Option<i32>>(b"native_rust\0")
            .expect("Symbol not present")
        };

        let v = Some(5);
        let v = native_rust(v);
        assert_eq!(v, Some(7));

        let v = None;
        let v = native_rust(v);
        assert_eq!(v, None);
    }

    #[test]
    fn test_shared_struct() {
        let shared_struct = unsafe {
            PLUGIN.get::<fn (Shared)->Shared>(b"shared_struct\0")
            .expect("Symbol not present")
        };

        let x = Shared {
            foo: 5,
            bar: 3,
            x: Some(8),
        };

        let x = shared_struct(x);

        assert_eq!(x.foo, 6);
        assert_eq!(x.bar, 5);
        assert_eq!(x.x, Some(6));
    }

}