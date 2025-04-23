use crate::resource::Resource;
pub trait Sink<RES: Resource> {
    fn get_rate(&self) -> usize;
}
