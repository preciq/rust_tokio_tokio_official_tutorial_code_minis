use std::time::Duration;

use tokio::time::sleep;

async fn computation1() -> String {
    sleep(Duration::from_secs(5)).await;
    String::from("Computation 1")
}

async fn computation2() -> String {
    sleep(Duration::from_secs(6)).await;
    String::from("Computation 2")
}

#[tokio::main]
async fn main() {
    let out = tokio::select! {
        //'out' saves the result of the select statement
        res1 = computation1() => res1,
        res2 = computation2() => res2,

        /*
        computation1 will finish first due to the 5 second wait time (6 for computation2)
        The result of the HANDLER ( => res1 / => res2) will be saved in 'out'
         */
    };

    println!("Got = {}", out);
}
