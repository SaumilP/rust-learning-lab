pub fn append_twice() {
    let mut values = vec![1];
    let first = &mut values;
    let second = &mut values;
    first.push(2);
    second.push(3);
}
