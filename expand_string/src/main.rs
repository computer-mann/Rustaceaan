// https://leetcode.com/problems/decode-string/
// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"


use queues::*;

fn main() {
    let queue:Queue<isize>=queue![];

    println!("{}",repeat_string(4, "ef"));

}

fn repeat_string(times:u32,to_be_repeated:&str)-> String{
    let mut window:String=String::from("");
    for _num in 0..times {
        window+=to_be_repeated;
    }
    window
}
