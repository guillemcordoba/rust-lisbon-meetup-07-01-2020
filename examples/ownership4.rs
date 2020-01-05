fn example1() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // BAD! Only one mutable reference possible at any given time
}

fn example2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BAD! Only either multiple immutable references or one mutable reference possible at any given time
}
