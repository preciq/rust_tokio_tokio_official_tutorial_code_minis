/*
We previously spawned a new task when we wanted to do something asynchronously
We can also use the select! macro to execute asynchronously as well

In summary, select! can take multiple async functions and returns the result of a single async computation.
Specifically, the one that finishes first.
*/

use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    //two oneshot (single send and receive) channels

    tokio::spawn(async {
        let _ = tx1.send("one");
    });
    //an async task

    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    //another async task

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }

        /*
        here, we pass 2 async tasks that are linked with a print statement each
        the one that completes first will be printed first
        the result of the second will be ignored
        
        note that the RESULT will be ignored but that doesn't mean the second will not be executed.
        It will, its just that if you want the result of the second operation, you won't get it with select!.
        */
    }
}