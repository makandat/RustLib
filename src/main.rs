use my_common::a;
use my_common::b;
use my_common::c;

fn main() {
    dbg!(a::args_count());
    for a in a::args() {
        dbg!(a);
    }
    let str1:String = b::to_String("String to &str");
    dbg!(str1);
    let str2 = String::from("&str to String");
    let strp: &str = str2.as_str();
    dbg!(strp);
    let f1:f64 = b::i32_to_f64(1000000);
    dbg!(f1);
    let n1:i32 = b::f64_to_i32(3.1415926);
    dbg!(n1);
    let s0: String = String::from("A dog");
    dbg!(c::tolower(s0));
    let s0: String = String::from("A dog");
    dbg!(c::toupper(s0));
    let s0: String = String::from(" A dog ");
    let s00: String = c::strip(s0);
    let s01: String = s00.clone();
    dbg!(s00);
    dbg!(c::length(s01));
}
