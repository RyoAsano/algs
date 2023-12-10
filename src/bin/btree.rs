use tree::trees::btree::BTree;

fn main() {
    let mut t = BTree::new(3);
    t.insert(0, "hey");
    println!("{:#?}", t);
    t.insert(1, "hey");
    println!("{:#?}", t);
    t.insert(2, "hey");
    println!("{:#?}", t);
    t.insert(3, "hey");
    println!("{:#?}", t);
    t.insert(4, "hey");
    println!("{:#?}", t);
    t.insert(5, "hey");
    println!("{:#?}", t);
    t.insert(6, "hey");
    println!("{:#?}", t);
    t.insert(7, "hey");
    println!("{:#?}", t);
    t.delete(7);
    println!("{:#?}", t);
    t.delete(6);
    println!("{:#?}", t);
    t.delete(5);
    println!("{:#?}", t);
    t.delete(4);
    println!("{:#?}", t);
    t.delete(3);
    println!("{:#?}", t);
    t.delete(2);
    println!("{:#?}", t);
    t.delete(1);

}
