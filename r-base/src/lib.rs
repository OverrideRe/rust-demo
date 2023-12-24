use std::any::Any;


pub trait SecurityAgent: Any + Sync + Send {
    fn name(&self) -> String;

    fn on_load(&self);
}