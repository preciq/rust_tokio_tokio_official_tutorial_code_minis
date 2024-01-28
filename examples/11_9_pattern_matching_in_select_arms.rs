/*
We have so far not seen any examples of a true pattern in the <pattern> portion of a select arm
    i.e. something like Some(x) that only matches Some's
We will see that now
*/

use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1): (mpsc::Sender<Option<()>>, mpsc::Receiver<Option<()>>) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send(None).await.unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        tx2.send(Some(4)).await.unwrap();
    });

    tokio::select! {
        Some(Some(v)) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(Some(v)) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }

        /*
        Why use such a complex pattern like Some(Some(v))? Because when a value is sent along an mpsc channel, it is implicitly put inside of a Some
        so rx1.recv() evaluates to Some(None) rather than just None unfortunately
        and rx2.recv() evaluates to Some(Some(4)) rather than just Some(4), despite my best efforts

        The above example was done to show that pattern matching has an impact on which select statements handler is executed.
        Even though tx2 is sent 4 seconds later than tx1, the second handler ("Got 4 from rx2") is what executes
        This is because the value sent from tx1 (Some(None)) does not match with the expected pattern in the branch (Some(Some(v)))
        But the value sent from tx2 (Some(Some(4))) does.
         */

        else => {
            println!("Both channels closed");
        }

        /*
        We can also have an else statement here in case both patterns mismatch after the branch async expressions resolve
         */
    }

}
