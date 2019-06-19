#[derive(Debug, Clone , Copy)]
struct A(i32);

struct B(f32);

pub fn clomain(){
    let a = A(32);
    //let b = B(12.13);
    // let c = a.clone() ; // clone
    let c = a; //use copy
    println!("{:?}" , a);
}