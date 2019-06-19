struct Object{
    width: u32,
    height: u32,
}

impl Object {
    fn  area (&self) -> u32 {
        self.width * self.height 
    }

    fn new (width: u32 , height: u32) -> Object {
        Object {
            width,
            height,
        }
    }
}

fn main() {
    let o = Object{
        width: 35, 
        height: 55,
    };
    let obj = Object::new(57, 85);
    println!("{} * {} with area : {} ", o.width, o.height, o.area());
    println!("{} * {} with area : {} ", obj.width, obj.height, obj.area());
}