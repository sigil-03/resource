use crate::resource::Resource;
pub trait Sink {
    type RESOURCE: Resource;
}
