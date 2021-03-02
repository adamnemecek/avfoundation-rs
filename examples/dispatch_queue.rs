use avfoundation::{DispatchTime, DispatchQueue};

fn main() {
    let queue = DispatchQueue::new("label");
    // queue.dispatch_async(|| {
    //     println!("async dispatched");
    // });
    let start = std::time::Instant::now();
    let after = DispatchTime::new(3 * 1_0000_000_000);
    queue.dispatch_after(after, move || {
        let end = std::time::Instant::now();
        println!("async dispatched after {:?}", end - start);
    });

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
