use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

use tokio::sync::oneshot;
use std::io;

async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Received: {:?}", frame);

        let response: Frame = Frame::Error("some other error 2".to_string());

        connection.write_frame(&response).await.unwrap();
    } 
}
//old process method (just used as an example) from section 1

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send(()).unwrap();
    });

    let mut listener = TcpListener::bind("localhost:3465").await?;
    //creates a listener for port 3465

    tokio::select! {
        _ = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // Help the rust type inferencer out
            Ok::<_, io::Error>(())
            /*
            We cannot specify a return type in an async block like we do in a function
            So we specify that this async block will return a result enum variant (Ok in this case)
            This allows us to use a question mark operator, like we did inside of the loop above

            If we didn't include this Ok line, we couldn't have used the question mark operator
             */
        } => {}
        _ = rx => {
            println!("terminating accept loop");
        }

        /*
        The first branch listens for incoming connections to the port 3465
        As we are not trying to make any connections in this program (unless you're doing it from elsewhere ;)) this will never resolve
        We put _ = to say that this will match any pattern (since it is no pattern at all, similar to 'val' from the previous section) but we also want to ignore the result (the result of the async operation)
            We also say we don't want to do anything if the async operation completes successfully (which is what the => {} is for)

        The second branch is much the same, except that it waits for rx to be resolved (which will happen since we are sending something to rx from tx towards the start of the async main function)
            We also say we want to print "terminating accept loop" if this branch resolves

        To come to the point, this is a demonstration of select! waiting for one of 2 async operations to resolve; the handler ( => {<insert handler code here>}) of whichever resolves first is executed and the other handler is ignored
            In this case, that would be the rx branch
         */

    }

    Ok(())
}