use crate::resource::Resource;

pub trait Source<RES: Resource> {
    fn get_rate(&self) -> usize;
}
