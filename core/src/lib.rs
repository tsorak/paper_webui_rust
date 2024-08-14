pub mod error;
pub use error::Error;

pub mod fs;
pub mod instance;
pub mod runner;

use std::sync::Arc;

pub fn init() -> Result<Arc<runner::Runner>, crate::Error> {
    let rnr = Arc::new(runner::Runner::new());

    Ok(rnr)
}
