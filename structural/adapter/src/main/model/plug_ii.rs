
#[derive(Default, Debug)]
pub struct PlugII {

}

impl crate::model::i_three_pin::IThreePin for PlugII {
    fn pin(&self) -> Result<(), usize> {
        println!("::: PlugII, pin(V)Result<(),usize>");
        Ok(())
    }
    fn pin_i(&self) -> Result<(), usize> {
        println!("::: PlugII, pin_i(V)Result<(),usize>");
        Ok(())
    }
    fn pin_ii(&self) -> Result<(), usize> {
        println!("::: PlugII, pin_ii(V)Result<(),usize>");
        Ok(())
    }
}