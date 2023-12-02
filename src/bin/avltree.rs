use tree::trees::bintree::{avl::Avl, BinTree};

fn main() {
    let mut root = BinTree::new(0, "top");
    root = root.insert(1, "level").unwrap();
    root = root.insert(2, "level").unwrap();
    root = root.insert(3, "level").unwrap();
    root = root.insert(4, "level").unwrap();
    root = root.insert(5, "level").unwrap();
    root = root.insert(6, "level").unwrap();
    println!("{:#?}", &root);
    root = root.insert(7, "level").unwrap();
    println!("{:#?}", &root);
    root = root.insert(8, "level").unwrap();
    println!("{:#?}", &root);
    root = root.insert(9, "level").unwrap();
    println!("{:#?}", &root);
    root = root.insert(10, "level").unwrap();
    println!("{:#?}", &root);
    root = root.delete(7).1.unwrap();
    println!("{:#?}", root);
}
