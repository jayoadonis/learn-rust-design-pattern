

pub struct AdapterIntoIIPin {
    three_pin: Box< dyn crate::model::i_three_pin::IThreePin >,
}

impl AdapterIntoIIPin {
    pub fn new( three_pin: Box< dyn crate::model::i_three_pin::IThreePin > ) -> Self {
        return Self {
            three_pin
        };
    }
}

impl crate::model::i_two_pin::ITwoPin for AdapterIntoIIPin {
    fn pin(&self) -> Result<(), usize> {
        self.three_pin.pin();
        Ok(())
    }
    fn pin_i(&self) -> Result<(), usize> {
        self.three_pin.pin_i();
        self.three_pin.pin_ii();
        Ok(())
    }
}