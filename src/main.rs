use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex,Arc};

fn main() {
    let _handle = thread::spawn(|| {
        for i in 1..100 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("Hi number {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("This is a {:?}.", v);
    });

//    println!("{:?}",v);

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![String::from("hi"),
                        String::from("from"),
                        String::from("the"),
                        String::from("thread")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![String::from("more"),
                        String::from("messages"),
                        String::from("for"),
                        String::from("you")];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

//    for received in rx {
//        println!("Got: {}", received);
//    }
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter=Arc::new(Mutex::new(0));
    let mut handles=vec![];

    for _ in 0..10{
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move ||{
            let mut num=counter.lock().unwrap();
            *num+=1;

        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());


}
