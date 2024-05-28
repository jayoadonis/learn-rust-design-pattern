
#[derive(Default)]
pub struct PlugII {

}

impl crate::model::i_three_pin::IThreePin for PlugII {
    fn pin(&self) -> Result<(), usize> {
        Ok(())
    }
    fn pin_i(&self) -> Result<(), usize> {
        Ok(())
    }
    fn pin_ii(&self) -> Result<(), usize> {
        Ok(())
    }
}