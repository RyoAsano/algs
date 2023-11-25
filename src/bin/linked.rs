use tree::lists::linked::Linked;


fn main() {
    let head = Linked::new_head(0);
    head.borrow_mut().append(1);
    head.borrow_mut().append(2);
    head.borrow_mut().append(3);
    println!("{:#?}", head);

    Linked::delete_from_head(&head, 2);

    println!("{:#?}", head);
}