/*
In other languages like dart, streams are loopable via an asynchronous for loop
Rust unfortunately does not have this yet
But the same results may be achieved by using a while let combined with next() (part of StreamExt(), a trait in the tokio-streams crate)
*/

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
            //even though 1, 2, 3 are not technically futures, by putting them into a stream, we make them futures (even though they evaluate immediately)
            //therefore, we must await them
        println!("GOT = {:?}", v);
    }
    /*
    Loops through the stream until it is done
     */
}