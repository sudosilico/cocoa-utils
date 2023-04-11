use cocoa_utils::AppNotification;
use cocoa_utils::NSAppWatcher;
use crossbeam::channel::unbounded;

fn main() {
    let (s, r) = unbounded::<AppNotification>();

    let thr = std::thread::spawn(move || {
        for notification in r {
            println!("{:?}", notification);
        }
    });

    NSAppWatcher::start_with_sender(s);

    thr.join().unwrap();
}
