
use crate::model::i_two_pin::ITwoPin;

#[derive(Default)]
pub struct PlugI {

}

impl PlugI {
    
}

impl ITwoPin for PlugI {
    fn pin(&self) -> Result<(), usize> {

        return Ok(());
    }
    fn pin_i(&self) -> Result<(), usize> {

        return Ok(());
    }
}