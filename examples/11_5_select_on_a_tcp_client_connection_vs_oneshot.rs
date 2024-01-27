//select demonstration of a TCP client connection vs a oneshot channel receiving a message

use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    // Spawn a task that sends a message over the oneshot
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });

    tokio::select! {
        socket = TcpStream::connect("localhost:3465") => {
            println!("Socket connected {:?}", socket);
        }
        
        msg = rx => {
            println!("received message first {:?}", msg);
        }
        /*
        This branch will always execute
        This is because nothing is hosted on port 3465 (unless you hosted something there ;))
        Though even if something was hosted at that port, it would not resolve faster than sending a message to a oneshot channel

        'socket' and 'msg', since they are not patterns (or patterns that will match anything) simply hold the result of their respective async operations here 
            (well 'msg' does anyway, socket doesn't get a chance to finish resolving)
         */
    }
}