#[derive(Clone)]
struct Node {
    id: String,
}

fn send_message(from: &Node, to: &Node) -> bool {
    // Simulated success
    from.id != to.id
}

#[test]
fn test_message_delivery() {
    let a = Node { id: "A".into() };
    let b = Node { id: "B".into() };

    assert!(send_message(&a, &b));
}

#[test]
fn test_no_self_send() {
    let a = Node { id: "A".into() };

    assert!(!send_message(&a, &a));
}
