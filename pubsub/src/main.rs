extern crate pub_sub;
extern crate uuid;

use std::thread;
use uuid::Uuid;

fn main() {
   let channel = pub_sub::PubSub::new();

   let mut handles = vec![];

   for _ in 0..16 {
       let recv = channel.subscribe();

        handles.push(thread::spawn(move || {
            for _ in 0..16 {
                println!("recevied {}", recv.recv().unwrap());
            }
        }));
    }

    for _ in 0..16 {
        let channel = channel.clone();

        handles.push(thread::spawn(move || {
            let msg_id = Uuid::new_v4();
            println!("    sent {}", msg_id);
            channel.send(msg_id).unwrap();
        }));
    }

    while let Some(handle) = handles.pop() {
        handle.join().unwrap();
    }
}