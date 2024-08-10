pub mod error;
pub use error::Error;

pub mod instance;
pub mod runner;

pub fn init() -> Result<runner::Runner, crate::core::Error> {
    println!("init");

    Err(Error::Unexpected)
}
