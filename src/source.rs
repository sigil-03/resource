use crate::resource::Resource;

pub trait Source {
    type RESOURCE: Resource;
}
