use std::time::Duration;

use tokio::time::sleep;

async fn action() {
    sleep(Duration::from_secs(5)).await;
    //sleeps for 5 seconds asynchronously
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(3).await.unwrap();
        tx.send(5).await.unwrap();
        tx.send(7).await.unwrap();
        tx.send(10).await.unwrap();
        tx.send(11).await.unwrap();
    });

    let operation = action();
    //this future is pending and its value is moved to the variable "operation"
    tokio::pin!(operation);
    /*
    The tokio::pin! macro is used to pin a future to the stack.

    In Rust, futures are values that can be polled to completion. However, they can't be moved around in memory after they've been polled once. This is because the future might have internal references that would become invalid if the future was moved.

    To ensure that a future doesn't move after it's been polled, you can "pin" it. Pinning a future tells the Rust compiler that the future's location in memory is fixed and it won't be moved.

    The tokio::pin! macro is a convenient way to pin a future to the stack. It's equivalent to creating a Pin<Box<Future>>, but without the heap allocation.

    In your code, tokio::pin!(operation); pins the operation future to the stack. This allows you to poll operation multiple times in the tokio::select! macro, without worrying about it being moved.

    tl;dr we have to pin a future if we want to use it in a select. select does this implicitly (which is why we haven't had to do this until now) but since we've already moved it once (from action() to operation), we must explicitly pin it before passing to select.
     */

    loop {
        tokio::select! {
            _ = &mut operation => break,
            //if this operation (which is tied to action(), see above) completes, break the loop
                //i.e. it will "resume" if it completes
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    break;
                } else {
                    println!("{}", v);
                }
                //because of this if/else statement, this select statement will continue to print the number it receives UNTIL one of the numbers are even (above, when we send 10)
                //it also breaks if the above arm completes as well
                    //this arm is being polled with EVERY iteration of the loop
            }
        }
    }
}
