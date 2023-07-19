pub mod hole;
pub mod parameters;

pub trait Builder<BType> {
    fn new() -> Self;
    fn build(self) -> BType;
}
