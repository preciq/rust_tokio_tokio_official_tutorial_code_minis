/*
An example of a stream using mini-redis

Before running this, start the mini-redis server (cloned from https://github.com/tokio-rs/mini-redis/tree/master) and ensure the port is correct (command should be RUST_LOG=debug cargo run --bin mini-redis-server -- --port 6379)

Have to clone it because our implementation of mini-redis does not include an implementation of subscriber, which is needed here
*/

use tokio_stream::StreamExt;
use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    /*
    This first connects to the server at port 6379
    It then pushes data onto the server on the channel "numbers"
    So anyone listening on that channel (including another client, like below) will receive these messages in the form of a stream
     */

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    //client connects to the server at 6379 and awaits a successfuly connection

    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    //this listens to what the client receives back on the channel "numbers"

    let messages = subscriber.into_stream();
    //what the client gets back from the "numbers" channel is turned into a stream (via into_stream()) and the result is saved within messages

    tokio::pin!(messages);
    //messages is pinned so that it can be evaluated successfully (and it isn't moved while being evaluated)

    while let Some(msg) = messages.next().await {
        //evaluates the individual contents of messages, one by one and prints them

        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async {
        publish().await
    });

    subscribe().await?;

    /*
    The above simulates 2 clients

    The first one, inside of tokio::spawn, sends messages to the numbers channel (see all the above in publish())

    The second, inside the main thread, takes these messages as a stream and evaluates them (and prints them, see above while let loop)
     */

    println!("DONE");

    Ok(())
}