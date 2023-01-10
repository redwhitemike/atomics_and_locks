use crate::borrowed_channel::Channel as BorrowedChannel;
use crate::one_shot_channel::Channel;
use crate::spin_lock::SpinLock;
use crate::type_safe_channel::channel;
use std::io::BufRead;
use std::thread;

mod borrowed_channel;
mod mutex_channel;
mod one_shot_channel;
mod spin_lock;
mod type_safe_channel;

fn main() {
    let mut channel = BorrowedChannel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        let t = thread::current();
        s.spawn(move || {
            sender.send("olla thread");
            t.unpark()
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receiver(), "olla thread");
    });
}

fn spin_lock() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 2, 1])
}

fn on_shot_channel() {
    let channel = Channel::new();

    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello, message from other thread");
            t.unpark()
        });
        while !channel.is_ready() {
            thread::park()
        }
        assert_eq!(channel.receive(), "hello, message from other thread")
    });
}

fn type_safe_channel() {
    thread::scope(|s| {
        let (sender, receiver) = channel();
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello other thread");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello other thread")
    })
}
