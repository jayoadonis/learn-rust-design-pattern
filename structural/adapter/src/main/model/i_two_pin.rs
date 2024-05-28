
pub trait ITwoPin {
    fn pin(&self) -> Result<(), usize>;
    fn pin_i(&self) -> Result<(), usize>;
}