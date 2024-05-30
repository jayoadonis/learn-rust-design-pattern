
use std::any::Any;

pub trait ITwoPin/* : Any + 'static */{
    fn pin(&self) -> Result<(), usize>;
    fn pin_i(&self) -> Result<(), usize>;
    // fn as_any(&self) -> &dyn Any;
}