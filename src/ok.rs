use std::{
    fs::File,
    os,
    thread::{self, JoinHandle},
    vec,
};

fn main() {
    let files = vec![
        "/home/rishi/fsm/java.go",
        "/home/rishi/fsm/mava.go",
        "/home/rishi/fsm/rama.go",
        "/home/rishi/fsm/panner.go",
    ];
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..4 {
        let file = files[i as usize];
        let handle = thread::spawn(move || {
            File::create(file).unwrap();
        });
        handles.push(handle)
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
