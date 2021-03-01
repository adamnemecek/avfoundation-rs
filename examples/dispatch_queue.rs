use avfoundation::DispatchQueue;

fn main() {
    let queue = DispatchQueue::new("label");
    queue.dispatch_async(|| {
        println!("async dispatched");
    });

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
