// enum Conveance{
//     Car(i32), Train(i32), Air(i32), Walk,
// }

// #[derive(Debug)]
// enum List{
//     Cons(i32,Option<Box<List>>), 
//     Nil,
// }

// fn main() {
//     // let x = 123;
//     // let y= Box::new(x);
//     // let z= &x;
//     let list = List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Nil))))));
//     println!("{:?}", list);

// }
#[derive(Debug)]

struct Huge_data;
#[derive(Debug)]
struct Small_data;

trait  Storage {
    
}

impl Storage for Huge_data {
    
}
impl Storage for Small_data {
    
}

fn main(){
    let data1 = Huge_data;
    let data2 =Box::new(Huge_data);

    let data3= data1;
    let data4 = data2;

    let data5 = Box::new(Small_data);
    let data:Vec<Box<dyn Storage>> = vec![Box::new(data3), data4, data5];
}