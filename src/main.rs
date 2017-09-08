extern crate hopper;

use std::{path, thread};

fn send(mut sender: hopper::Sender<u16>) -> () {
    let mut cur = 0;
    loop {
        sender.send(cur);
        cur = cur.wrapping_add(1);
    }
}

fn recv(receiver: hopper::Receiver<u16>) -> () {
    for _ in receiver {
        // nothing, intentionally
    }
}

fn main() {
    let mut joins = Vec::new();
    let (sender, receiver) = hopper::channel("fd_test", path::Path::new("/tmp/fd_test")).unwrap();

    for _ in 0..2 {
        let snd = sender.clone();
        joins.push(thread::spawn(move || send(snd)));
    }

    joins.push(thread::spawn(move || recv(receiver)));

    for jh in joins {
        jh.join().expect("uh oh child thread panicked");
    }
}
