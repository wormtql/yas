pub mod cancel;
pub mod color;

pub mod positioning;

#[derive(Debug)]
pub enum ScrollResult {
    TimeLimitExceeded,
    Interrupt,
    Success,
    Failed,
    Skip,
}