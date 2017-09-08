use std::{fs, thread, time};

fn unlink_drop(fp: fs::File) {
    let _ = fs::remove_file("/tmp/foo.txt");
    drop(fp);

    let slp = time::Duration::from_millis(10_000);
    loop {
        thread::sleep(slp);
    }
}

fn main() {
    let fp = fs::File::create("/tmp/foo.txt").unwrap();

    let join = thread::spawn(move || unlink_drop(fp));

    join.join().expect("uh oh child thread panicked");
}
