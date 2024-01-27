use tokio::sync::oneshot;

async fn some_operation() -> String {

    String::from("This is an async operation") + "!"
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // Select on the operation and the oneshot's
        // `closed()` notification.
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // `some_operation()` is canceled, the
                // task completes and `tx1` is dropped.
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

/*
Though the result here doesn't change much from the previous section (select still only shows the result of one operation), here, we close tx1 after the message is sent
We can modify the code (not shown due to complexity but would need an Arc<Mutex<>> for this) to cancel a future before it completes and is sent in this way (in other words cancel a future mid execution) using .closed() on the sender
*/