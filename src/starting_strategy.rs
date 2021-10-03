#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StartingStrategy {
    FromStart,
    FromEnd,
    FromPosition,
    FromGtid,
}
