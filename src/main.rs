pub mod cli;
pub mod nok;
pub mod ok;
pub mod op;
use inotify::{Inotify, WatchMask};
use op::Method;
use std::fs;
use tokio::{fs::File, sync::mpsc};

struct Watcher<'a> {
    source: &'a str,
    destination: &'a str,
    ops: Vec<Method<'a>>,
}
impl<'a> Watcher<'a> {
    async fn watch(self) {
        let mut notifs = Inotify::init().unwrap();
        notifs
            .watches()
            .add(self.source, WatchMask::CLOSE_WRITE)
            .expect("");

        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100);
        let sendhandle = tokio::spawn(async move {
            loop {
                let mut tmp_buffer = [0; 1024];
                let events = notifs.read_events_blocking(&mut tmp_buffer).expect("");
                for event in events {
                    let name = event.name.unwrap().to_str().unwrap();
                    let name_vec = name.as_bytes().to_vec();

                    // What you are trying to do is to send a &mut reference to the buffer which is not a very good idea.

                    // For the sake of argument, imagine that rust allowed you to do so:

                    //     That buffer is stack allocated so once you exit from that function, the receiver would have a pointer to uninitialized memory, which is UB
                    //     That function could have overwritten that buffer while the receiver was still reading it - that too would have caused a hard to detect bug in your application.

                    // Instead rust protected you from those two error scenarios.

                    // What you need to do is to use a heap allocated buffer (Vec<T>) instead and send the buffer itself instead of a reference., which would protect you from (1). That would force you to create a new buffer on each iteration due to the move semantics, which would protect you from (2)

                    tx.send(name_vec).await.expect("failed to send message");
                }
            }
        });

        while let Some(msg) = rx.recv().await {
            let mut name = Some(String::from_utf8(msg).unwrap());
            let mut iter = self.ops.iter().peekable();
            while let Some(meth) = iter.next() {
                if let Some(n) = &name {
                    if meth.matches(n) {
                        match fs::rename(
                            format!("{}/{}", self.source, n),
                            format!("{}/{}", self.destination, n),
                        ) {
                            Err(e) => println!("error rename : {}", e.to_string()),
                            Ok(()) => name = None,
                        };
                    }
                };
            }
        }
        sendhandle.await.expect("failed to wait for join handle");
    }
}
#[tokio::main]
async fn main() {
    // let watcher = Watcher {
    //     source: "/home/rishi/Downloads",
    //     destination: "/home/rishi/Videos/haikyuu",
    //     ops: vec![Method::HasExt("mkv")],
    // };
    // watcher.watch().await

    let cashflow: [f32; 6] = [126.0, -883.0, 1773.0, 1736.0, -741.0, 1243.0];
    // let multiplier: f32 = 100.0 / 9.0;
    // let mut rate: f32 = 0.0;
    // for i in 1..9 {
    //     let this_rate = (cashflow[i] / cashflow[i - 1]) - 1.0;
    //     println!("this_rate: {}", this_rate);
    //     rate += this_rate;
    // }

    // rate = rate * multiplier;
    // println!("simple rate {}", rate);

    let multiplier_power: f32 = 1.0 / 6.0;
    let ev_by_bv: f32 = cashflow[5] / cashflow[0];

    let cagr = ev_by_bv.powf(multiplier_power);
    println!("compounuded rate {}", cagr);

    let return_v = cagr.powf(15.0) * cashflow[5];
    println!("predicted cashflow {}", return_v);
    println!("times cashflow {}", return_v / cashflow[5]);
    println!("predicted mk.cap {}", return_v * 98.0);
    println!("predicted return {}", (return_v * 50.0) / 122911.0);
    // let ev = (cagr+1.0).po
}
