/*
Unlike an async block, where we need to move anything that is to be used into the async block,
We can use references inside of select branches
*/

use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            Ok::<_, io::Error>(())
        } => {}
        /*
        Notice how we can use data (a u8 array reference) inside of BOTH match arms
        The same rules of ownership apply here like with the rust book
        We can have as many immutable references as we want (like we do here, since we are writing to different sockets in both arms)
        OR
        We can have ONE mutable reference
        */

        else => {
            println!("Nothing happened, both arms failed.")
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let data = [1, 2, 3, 19, 25];
    let addr1: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr2: SocketAddr = "127.0.0.1:8081".parse().unwrap();

    race(&data, addr1, addr2).await.unwrap();

    //in this case, running this code will result in "Nothing happened, both arms failed." getting printed because both port 8080 and port 8081 have nothing hosted on either (unless you put something there of course ;))
}
