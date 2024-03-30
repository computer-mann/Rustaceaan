use crate::shapes::rectangle::Rectangle;

mod shapes;

fn main() {
    // println!("Hello, world!");
    let rec=Rectangle{
        width: 30,
        height: 40
    };
    let sec_rec=Rectangle{
        width: 10,
        height: 10
    };
    println!("The rectangle values are {:?}",rec);
   // println!("The rectangle values are {:#?}",rec);
    println!("The area of the rectangle is {:?}",rec.area());
    println!("{}",area(&rec));
    println!("Can rec hold other rec? {}",rec.can_hold(&sec_rec));

}

fn area(rect:&Rectangle)->u32{
    rect.width * rect.height
}
