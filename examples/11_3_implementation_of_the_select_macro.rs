/*
A rough implementation of the internals of select! (how the future trait is implemented for select)

In the actual implementation, select! has additional checks and things implemented that pick a random future to check, it doesn't check in order
*/

use tokio::sync::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        /*
        This example checks 2 futures
        If either future returns Poll::Ready, that means that future is ready
        Which means return that future and exit MySelect
         */

        Poll::Pending
        //if neither of them are ready, return Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    // use tx1 and tx2

    MySelect {
        rx1,
        rx2,
    }.await;
    //testing to see which receiver resolves first with our custom implementation of select!
}

/*
A note that running this will cause the code to hang indefinitely
Since we are not sending anything via tx1/tx2, rx1 and rx2 will never resolve, resulting in the future staying Pending forever
*/