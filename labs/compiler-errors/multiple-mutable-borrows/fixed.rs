pub fn append_twice() {
    let mut values = vec![1];
    {
        let first = &mut values;
        first.push(2);
    }
    let second = &mut values;
    second.push(3);
}
