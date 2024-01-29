/*
We will use the previous code to generate a stream, then modify the stream
This will use functions we have used with iterators, like take(), filter() and map()

Remember to start mini-redis server before trying this (see previous section for how, need to clone it from github)
*/

use mini_redis::client;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;

    //let messages = subscriber.into_stream().take(3);
    /*
    Here, instead of taking the unchanged stream from the "numbers" channel into messages, we instead cut it down
    We do this by using take(3), which specifies that we only want the first three elements of the original stream
    Commented out because this will interfere with (and be made redundant by) below filter and map combination
     */

    let messages = subscriber
    .into_stream()
    .filter(|msg| match msg {
        Ok(msg) if msg.content.len() == 1 => true,
        _ => false,
    })
    /*
    this uses the filter function, just like we do in iterators
    Here we are taking each element of the stream from the "numbers" channel
    If the length of the message is equal to one (so "1", "3", "6"), then it is added to the stream to be returned
    If not, the message is NOT added (so bigger messages like "two", "four", etc.)

    The result is a stream with only single digit messages
    */

    .map(|msg| msg.unwrap().content)
    /*
    The resulting stream from above is modified (all of the elements are modified) using map()
    Essentially, each msg is unwrapped (to get rid of the Ok) and then only the content from within is returned 
    So the original msg, which may have looked like this: 
        Ok(Message { channel: "numbers", content: b"1" })
    
    Will look like this:
        b"1"
    Which is a lot more succinct :D
     */

    .take(3); //further modifiying the stream by only taking the first three elements of it, though this is redundant as there are only 3 elements that meet the filter criteria (see above)


    tokio::pin!(messages);
    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });
    subscribe().await?;
    println!("DONE");
    Ok(())
}
