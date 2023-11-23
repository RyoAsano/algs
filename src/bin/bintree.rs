use tree::{trees::bintree::BinTree, recorddb::RecordDb};


fn main() {
    let root = BinTree::new(0, "top");
    root.insert(-5, "level 1");
    root.insert(-7, "level 2");
    root.insert(-3, "level 2");
    root.insert(-4, "level 3");
    root.insert(-8, "level 3");
    root.insert(-6, "level 3");

    println!("{:#?}", root);

    root.delete(-5);

    println!("{:#?}", root);
}
