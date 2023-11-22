#[derive(Error,Debug,Clone)]
pub enum GQGMCError {
    #[error("miscellaneous: {0}")]
    Miscellaneous(String)
}