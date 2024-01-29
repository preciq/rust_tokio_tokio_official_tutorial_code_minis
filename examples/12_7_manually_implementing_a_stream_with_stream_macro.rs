/*
We can additionally use the stream! macro to manually create a stream.
To do this, also import the async-stream crate --> 

async-stream = "0.3.5"
# add to cargo.toml
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::StreamExt;
use tokio::pin;
struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;


    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();

            Poll::Pending
        }
    }
}
//code from previously (section 10)

#[tokio::main]
async fn main() {

    use async_stream::stream;
    use std::time::{Duration, Instant};
    let mut counter = 0;

    let x = stream! {
        //here, we use the stream! macro to generate a stream, rather than manually generating with the stream trait
        //the resulting stream is saved inside of 'x' (let x = ...)
        let mut when = Instant::now();
        for _ in 0..3 {
            let delay = Delay  { when };
            delay.await;
            //due to the above code, this asynchronously waits 10 milliseconds (and also awaits delay's completion)
            counter += 1;
            yield counter;
            /*
            yield here works very similarly to yield in dart
            the value of counter (which is incremented with each iteration of this loop) is added to the stream that is being saved in 'x' (see above)
                Added is a bit of a misnomer; yield is producing a future from the provided counter value and adding that to the stream
                Though since it just an integer, it takes next to no time to finish
            This happens with every iteration of the loop (in this case, 3 times)
            Meaning 'x' contains a stream of 3 elements (3 i32's, to be exact)
             */
            when += Duration::from_millis(10);
        }
    };

    pin!(x);
    //pinning so that x can be processed (and does not move while it is being processed)
    /*
    The pin!(x); macro is used to pin the stream x in memory. Pinning is necessary when dealing with self-referential structs or when you want to ensure an object remains at the same memory location. In this case, it's not strictly necessary to pin x because it's not moved after being pinned and it doesn't contain any self-references. However, it's a good practice to pin streams and futures before awaiting them because some async operations require the type to be pinned.
     */

    while let Some(msg) = x.next().await {
        println!("{:?}", msg);
    }
    //the stream within 'x' is processed with a while let, like we would process any other stream
}

