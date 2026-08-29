use std::rc::Rc;

pub fn share_across_thread() {
    let count = Rc::new(1);
    std::thread::spawn(move || println!("{count}"));
}
