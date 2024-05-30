
use std::any::Any;

pub struct AdapterIntoIIPin<'a> {
    three_pin: Box< &'a dyn crate::model::i_three_pin::IThreePin >,
}

impl<'a> AdapterIntoIIPin<'a> {
    pub fn new( three_pin: Box< &'a dyn crate::model::i_three_pin::IThreePin > ) -> Self {
        return Self {
            three_pin
        };
    }
    pub fn get_adaptee(&'a self) -> &'a dyn crate::model::i_three_pin::IThreePin {
        *self.three_pin
    }
}

impl crate::model::i_two_pin::ITwoPin for AdapterIntoIIPin<'_> {
    fn pin(&'_ self) -> Result<(), usize> {
        println!("::: AdapterIntoIIPin, pin(V)Result<(),usize>");
        self.three_pin.pin();
        Ok(())
    }
    fn pin_i(&'_ self) -> Result<(), usize> {
        println!("::: AdapterIntoIIPin, pin_i(V)Result<(),usize>");
        self.three_pin.pin_i();
        self.three_pin.pin_ii();
        Ok(())
    }
    // fn as_any(&self) -> Self {
    //     self
    // }
}