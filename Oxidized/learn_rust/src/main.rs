fn main() {
   
    let mut text=String::from("awula ");
    let one=&text;
    let two=&text;
    println!("{},{}",two,one);

    let three=one;
}

fn print_number(num:String){
    let s1=String::from("I am awesome");
    let s2=s1;
    println!("{num}");
}

fn change(text:&mut String){
    text.push_str("string");
}