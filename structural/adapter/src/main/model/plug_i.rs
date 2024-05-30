
use std::any::Any;
use crate::model::i_two_pin::ITwoPin;

#[derive(Default, Debug)]
pub struct PlugI {

}

impl PlugI {
    
}

impl ITwoPin for PlugI {
    fn pin(&self) -> Result<(), usize> {
        println!("::: PlugI, pin(V)Result<(),usize>");
        return Ok(());
    }
    fn pin_i(&self) -> Result<(), usize> {
        println!("::: PlugI, pin_i(V)Result<(),usize>");
        return Ok(());
    }
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}