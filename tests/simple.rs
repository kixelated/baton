use baton::Baton;
use futures::FutureExt;

#[derive(Baton)]
struct MyStruct {
    pub field1: i32,
    pub field2: String,
}

#[test]
fn simple() {
    let (send, recv) = MyStruct {
        field1: 42,
        field2: "hello".to_string(),
    }
    .baton();

    // Receive the default values first.
    assert_eq!(recv.field1().now_or_never().unwrap().unwrap(), 42);
    assert_eq!(recv.field2().now_or_never().unwrap().unwrap(), "hello");

    // We block if we try to receive again.
    assert!(recv.field1().now_or_never().is_none());
    assert!(recv.field2().now_or_never().is_none());

    // Send new values.
    assert!(send.field1(69).is_some());
    assert!(send.field2("world".to_string()).is_some());

    // Overwrite one of them before receiving.
    assert!(send.field1(420).is_some());

    assert_eq!(recv.field1().now_or_never().unwrap().unwrap(), 420);
    assert_eq!(recv.field2().now_or_never().unwrap().unwrap(), "world");

    // Drop the sender to close the channel.
    // But first insert a pending update.
    assert!(send.field1(999).is_some());
    drop(send);

    // We still get the pending update.
    assert_eq!(recv.field1().now_or_never().unwrap().unwrap(), 999);

    // The receiver should return None from now on.
    assert!(recv.field1().now_or_never().unwrap().is_none());
    assert!(recv.field2().now_or_never().unwrap().is_none());
}
