/*
We can "queue" up multiple channels and handle the results of each one by one with a combination of a select! macro and a loop
*/

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            /*
            Here, we have three channels receiving, inside of a loop
            How it works is like follows
            
            One of these channels receives a message, which is saved in the msg variable (let msg = ...)
            it is then printed
            the loop then runs again
            In the meantime, if another future resolves, its value is saved inside of message instead and printed
            And on an on it goes, until there are no more messages remaining
            Note that no message is ever ignored, just pushed down the "queue" (to be selected and printed later down the line in a further iteration)

            So we basically have a loop that handles every message sent to it from multiple channels (basically a queue!)

            Note: no specific order is guaranteed. i.e. lets say rx1 gets a message and it prints, but while this is happening, rx2 and then after that rx3 both receive messages WHILE rx1 msg is printing. There is no guarantee that rx2 will print before rx3, only that both will print at some point (this order is randomly selected by the select! macros inner workings, when IT decides to check the status of the rx2 and rx3 future) 
             */

            else => { break }
            //finally, when the channels no longer receive messages (i.e. sender is dropped or something similar), the else condition activates, ending the loop
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}