use crate::resource::Resource;

pub trait Storage<RES: Resource> {
    fn get_capacity(&self) -> usize;
}
