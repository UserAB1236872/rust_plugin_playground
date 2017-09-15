extern crate libloading;
extern crate common;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    use common::{Shared, SharedTrait};
    use libloading::Library;

    const PLUGIN_DIR: &'static str = "../plugin/target/release/libplugin.so";

    lazy_static! {
        static ref PLUGIN: Library = 
                    Library::new(PLUGIN_DIR)
                        .expect("Can't load library");
                
    }

    #[test]
    fn test_native() {
        let native_rust = unsafe {
            PLUGIN
                .get::<fn(Option<i32>) -> Option<i32>>(b"native_rust\0")
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
            PLUGIN
                .get::<fn(Shared) -> Shared>(b"shared_struct\0")
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

    #[test]
    fn test_boxed_shared_trait() {
        let boxed_shared_trait = unsafe {
            PLUGIN
                .get::<fn(*mut i32) -> Box<SharedTrait>>(b"boxed_shared_trait\0")
                .expect("Symbol not present")
        };

        let mut x: i32 = 5;
        // Test dropping too
        {
            let mut trait_obj = boxed_shared_trait(&mut x as *mut i32);

            trait_obj.bar();
            assert_eq!(x, 30);
        }

        // This means it drops!
        assert_eq!(x, 90);
    }
}