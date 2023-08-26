mod exposure;
mod lca;
pub use lca::*;
use ogcat::ogtree::TreeCollection;

// this is the driver code for testing during development
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
