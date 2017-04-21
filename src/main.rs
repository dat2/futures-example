extern crate futures;
extern crate rand;

use std::time::Duration;
use std::thread;
use std::sync::mpsc::channel;

use rand::Rng;

use futures::Future;
use futures::sync::oneshot;

fn main() {
    let (data_tx, data_rx) = channel();
    let (oneshot_tx, oneshot_rx) = channel();
    thread::spawn(move|| {
        loop {
            let oneshot: oneshot::Sender<_> = oneshot_rx.recv().unwrap();
            let data = data_rx.recv().unwrap();
            oneshot.send(data).unwrap();
        }
    });
    
    // send numbers infinitely
    for i in 0..5 {
        let data_tx = data_tx.clone();
        thread::Builder::new()
            .name(format!("Thread-{}", i))
            .spawn(move|| {
                let mut rng = rand::thread_rng();
                for i in 0.. {
                    let seconds = rng.gen_range(1,5);
                    thread::sleep(Duration::new(seconds, 0));
                    data_tx.send(format!("{}({}):{}s", thread::current().name().unwrap(), i, seconds)).unwrap();
                }
            })
            .unwrap();
    }
    
    // allow another thread to recv
    let (oneshot_rx_tx, oneshot_rx_rx) = channel();
    thread::spawn(move|| {
        loop {
            let (once_tx, once_rx) = oneshot::channel();
            oneshot_tx.send(once_tx).unwrap();
            oneshot_rx_tx.send(once_rx).unwrap();
        }
    });
    
    // the main thread recvs futures
    // then it blocks for each future
    loop {
        let once_rx = oneshot_rx_rx.recv().unwrap();
        println!("{}", once_rx.wait().unwrap());
    }
}
