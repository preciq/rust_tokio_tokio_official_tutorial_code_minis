//use of the question mark operator for error handling in select! macro

use tokio::sync::oneshot;
use std::io;

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Received: {:?}", frame);

        let response: Frame = Frame::Error("some other error 2".to_string());

        connection.write_frame(&response).await.unwrap();
    } 
}

#[tokio::main]
async fn main() -> io::Result<()> {
    //by setting this main function to return a Result enum, we can freely use the ? operator for error handling

    let (tx, rx) = oneshot::channel();

    // Spawn a task that sends a message over the oneshot
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });
    
    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // Help the rust type inferencer out
            Ok::<_, io::Error>(())
            //we have to specify that this async block will return an Ok (a variant of the Result enum) so that we can use the ? operator freely in the async block
            //since blocks are different for functions (can't specify a return type in the block signature)
            
        } => {
            res?;
        }
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}