fn main() {
    let mut _mutable_val = 10;

    // Ok
    _mutable_val = 5;

    {
        let _mutable_val = _mutable_val;

        // Not Ok
        // _mutable_val = 1;
    }

    println!("{}", _mutable_val);
}
