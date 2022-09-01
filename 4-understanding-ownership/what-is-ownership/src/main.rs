/// 2 important points in time here:
/// * when s comes into scope, it is valid
/// * it remains valid until it goes out of scope

fn main() {                         // s is not valid here, it's not yet declared
    let s = String::from("hello");  // s is valid from this point forward

    // do stuff with s
}                                   // this scope is now over, and s is no longer valid

