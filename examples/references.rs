fn main() {}

fn one_mutable_ref() {
    let mut s = String::from("example string");
    let _x = &mut s;
    example(_x);
}

fn many_immutable_refs() {
    let s = String::from("example string");
    let _x = &s;
    let _y = &s;
    example(_x);
    example(_y);
}

fn bad_referencing() {
    let mut s = String::from("example string");
    let _x = &mut s;
    let _y = &s;
    example(_x);
    example(_y);
}

fn example(_d: &String) {}
