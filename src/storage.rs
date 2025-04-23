use crate::resource::Resource;

pub trait Storage {
    type RESOURCE: Resource;
}
