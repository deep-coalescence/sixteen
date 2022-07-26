mod exposure;
mod lca;
pub use lca::*;
use ogcat::ogtree::TreeCollection;
// extern crate fifteen;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collection = TreeCollection::from_newick("res/quintets.tre")?;
    let wrapped = TreeCollectionWithLCA::from_tree_collection(collection);
    let (one, two, three, four, five) = wrapped.translate_taxon_names(("1", "2", "3", "4", "5"));
    for i in &wrapped.lca {
        let quintet = [
            i.rev[one],
            i.rev[two],
            i.rev[three],
            i.rev[four],
            i.rev[five],
        ];
        println!("{:?}", i.retrieve_topology(&quintet));
    }
    Ok(())
}
