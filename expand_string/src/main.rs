
// https://leetcode.com/problems/decode-string/
// Input: s = "2[abc]3[cd]ef"
// Output: "abcabccdcdcdef"
fn main() {
    println!("{}",repeat_string(4, "ef"));
}

fn repeat_string(times:u32,to_be_repeated:&str)-> String{
    let mut window:String=String::from("");
    for num in (0..times){
        window+=to_be_repeated;
    }
    window
}
