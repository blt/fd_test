extern crate hopper;
#[macro_use]
extern crate lazy_static;

use std::{path, sync, thread, time};
use std::sync::atomic::AtomicUsize;

lazy_static! {
    static ref SIGNS: sync::Mutex<Vec<AtomicUsize>> = sync::Mutex::new(Vec::new());
}

fn send(idx: usize, mut sender: hopper::Sender<u64>) -> () {
    let mut cur = 0;
    let sleep_time = time::Duration::new(0, ((idx+1) as u32) * 100);
    loop {
        thread::sleep(sleep_time);
        sender.send(cur);
        (SIGNS.lock().unwrap())[idx].fetch_add(1, sync::atomic::Ordering::Relaxed);
        cur = cur.wrapping_add(1);
    }
}

fn recv(idx: usize, receiver: hopper::Receiver<u64>) -> () {
    let mut recv = receiver.into_iter();
    let sleep_time = time::Duration::from_millis(100);
    loop {
        (SIGNS.lock().unwrap())[idx].fetch_add(1, sync::atomic::Ordering::Relaxed);
        match recv.next() {
            Some(_) => {
                // nothing, intentionally
            }
            None => {
                thread::sleep(sleep_time);
            }
        }
    }
}

fn monitor() -> () {
    let sleep_time = time::Duration::from_millis(10_000);
    loop {
        thread::sleep(sleep_time);
        println!("= = = = = = = = = = = = = = = = = = = = = = = = =");
        for (idx, epoch) in SIGNS.lock().unwrap().iter().enumerate() {
            let v = epoch.load(sync::atomic::Ordering::Relaxed);
            println!("{:03} : {}", idx, v);
        }
    }
}

fn main() {
    let mut joins = Vec::new();
    let (sender, receiver) = hopper::channel_with_max_bytes("fd_test", path::Path::new("/tmp/fd_test"), 1024).unwrap();

    for i in 0..10 {
        (SIGNS.lock().unwrap()).push(AtomicUsize::new(0));
        let snd = sender.clone();
        joins.push(thread::spawn(move || send(i, snd)));
    }

    (SIGNS.lock().unwrap()).push(AtomicUsize::new(0));
    joins.push(thread::spawn(move || recv(10, receiver)));

    joins.push(thread::spawn(monitor));

    for jh in joins {
        jh.join().expect("uh oh child thread panicked");
    }
}
