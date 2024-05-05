#[derive(Debug)]
pub enum ScrollResult {
    TimeLimitExceeded,
    Interrupt,
    Success,
    Failed,
    Skip,
}
