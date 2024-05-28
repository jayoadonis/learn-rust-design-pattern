

pub trait IThreePin {
    fn pin(&self) -> Result<(), usize>;
    fn pin_i(&self) -> Result<(), usize>;
    fn pin_ii(&self) -> Result<(), usize>;
}


