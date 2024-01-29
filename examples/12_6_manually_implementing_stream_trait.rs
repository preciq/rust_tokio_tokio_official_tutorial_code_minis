/*
Demonstrates the manual implementation of Stream on a struct 
(to be able to treat instances of that struct as Streams)
*/

use tokio_stream::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio_stream::StreamExt;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
//previous code from section 10 with the delay struct 

struct Interval {
    rem: usize,
    //will keep track of how many elements are within the Interval stream, and will let poll_next() below know when the stream is finished
    delay: Delay,
    //a delay of a few ms to mimic asynchronous behaviour
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            //sets an initial value of 3 for rem (counter 3, so simulating 3 elements in the stream)
            delay: Delay { when: Instant::now() }
            //sets an initial value of Instant::now() for delay (meaning the delay will not wait at all. and never be pending. This is for example purposes)
        }
    }
}
//constructor

impl Stream for Interval {
    type Item = ();
    //associated type (what will the stream contain?)
    //in this case, for example purpose, we say the Stream will carry a series of empties (empty tuples ())

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<()>>
    {
        if self.rem == 0 {
            // No more delays
            return Poll::Ready(None);
            //when this executes, the stream is concluded (meaning nothing more is within the stream)
        }

        match Pin::new(&mut self.delay).poll(cx) {
            //here we are polling the Delay future that is within the Interval field "delay"
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                //once the delay completes, we are decrementing 1 from self.rem (remember the stream returns Poll::Ready(None) ONLY if self.rem == 0, we are closing in on that here)
                self.rem -= 1;
                
                Poll::Ready(Some(()))
                //return a Poll::Ready with a Some(()) inside of it; this simulates an element being resolved from within the stream
                    //it is a bit more than a simulation; it actually returns an element (a (), which is nothing, but technically still something ;))
            }

            Poll::Pending => Poll::Pending,
            //again for example purposes, we are not handling Pending; in production, we would put some logic here to call the waker again at some point rather than just return Pending
            //Since Delay is set to Instant::now(), it will not wait at all, so there is no risk of Delay ever being pending
        }
    }
}
//stream implementation for Interval

#[tokio::main]
async fn main() {
    let mut counter = 0;

    let mut sample_stream = Interval::new();

    while let Some(a) = sample_stream.next().await {
        counter += 1;
        println!("{:?}", a);
        println!("Stream item #{}", counter);
    }
    //asynchronously looping through the stream with while let
    //awaiting each element in the stream
    //also incrementing counter so that it keeps track of how many elements in the stream we've processed
}