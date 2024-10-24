//! Make me compile without adding new lines-- just changing existing lines!

fn main() {
    let s0 = String::from("Hello World");

    println!("{} == `{}`", stringify!(s0), s0);

    let mut s1 = append_to_string(s0);

    s1.push('!');

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(s: String) -> String {
    s
}
