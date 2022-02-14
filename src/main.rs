fn foo(s: &mut String) {
    println!("{}\n", s);
}

fn main() {
    println!("Hello, world!");

    let mut s = String::from("Foobar");
    let s2 = &mut s;
    println!("hello rust {}\n", s2);
    foo(&mut s);
    foo(&mut s);
}
