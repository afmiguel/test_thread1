use std::thread::sleep;

fn main() {
    for i in 0..10 {
        println!("I'm the main Thread! Count is {i}");
        sleep(std::time::Duration::from_secs(1));
    }
}
