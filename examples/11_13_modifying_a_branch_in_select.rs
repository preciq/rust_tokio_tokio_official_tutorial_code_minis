/*
We can also modify the contents of a branch (technically speaking, see below for more) in a select
*/

async fn action(input: Option<i32>) -> Option<String> {
    //If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    
    Some(i.to_string())
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);
    
    let mut done = false;
    let operation = action(None);
    //1). passing a None into action, which will cause action to return None as well (see match statement within action())
    tokio::pin!(operation);
    
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });
    
    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                //1.5). &mut operation initially resolves to None (see point 1), which is saved inside of res
                //we intend to change it later with set() (see point 4) which is why we make it &mut
                done = true;

                if let Some(v) = res {
                    //2). this initially returns false and does not run, since res is the result of operation which is from action() which initially returns None (see above)

                    //5). with the execution of point 4, res now matches the pattern of Some(v), and 'v' is String::from("2")
                    //6). The result gets printed below, with "return" breaking the loop (see below)
                    println!("GOT = {}", v);
                    return;
                    //7). this causes the loop to break (by returning nothing, or ())
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.

                    /*
                    3). This continues to receive until it gets an even number (2, see tokio::spawn above)
                    When it receives this even number, this if statement activates
                     */
                    operation.set(action(Some(v)));
                    /* 
                    4). The 2 from above is put into a Some, which is passed into action
                    Which in turn is passed into set
                        set is a built in function of Pin
                        it allows us to change the value of operation
                        Which, from point 1.5, is initially a None
                        But now, since we passed a Some(2) into action, action returns a String::from("2") (see definition of action() above)

                    Meaning essentially, we have changed the first branch (point 1.5) (where originally res was None, now it is Some(2))
                    */
                    done = false;
                }
            }
        }
    }
}