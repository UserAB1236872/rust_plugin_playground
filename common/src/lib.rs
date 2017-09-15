pub struct Shared {
    pub foo: usize,
    pub bar: i32,
    pub x: Option<i32>,
}

pub trait SharedTrait {
    fn bar(&mut self);
}
