extern crate hopper;

use std::{path, thread, time};

fn send(mut sender: hopper::Sender<u16>) -> () {
    let mut cur = 0;
    let sleep_time = time::Duration::from_millis(1);
    loop {
        sender.send(cur);
        cur = cur.wrapping_add(1);
        thread::sleep(sleep_time);
    }
}

fn recv(receiver: hopper::Receiver<u16>) -> () {
    let mut recv = receiver.into_iter();
    loop {
        match recv.next() {
            Some(_) => {
                // nothing, intentionally
            },
            None => {},
        }
    }
}

fn main() {
    let mut joins = Vec::new();
    let (sender, receiver) = hopper::channel("fd_test", path::Path::new("/tmp/fd_test")).unwrap();

    for _ in 0..10 {
        let snd = sender.clone();
        joins.push(thread::spawn(move || send(snd)));
    }

    joins.push(thread::spawn(move || recv(receiver)));

    for jh in joins {
        jh.join().expect("uh oh child thread panicked");
    }
}
