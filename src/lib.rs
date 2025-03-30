use std::sync::Arc;
use rkyv::Archive;

#[derive(Archive, Default, Debug)]
#[rkyv(derive(Debug))]
pub struct Inner {
    pub a: usize,
    pub b: usize,
}

#[derive(Archive, Default, Debug)]
#[rkyv(derive(Debug))]
pub struct Outer {
    pub inner: Arc<Inner>,
}
