struct Object{
    width: u32,
    height: u32,
}

impl Object {
    fn  area (&self) -> u32 {
        self.width * self.height 
    }
}

fn main() {
    let o = Object{
        width: 35, 
        height: 55,
    };

    println!("{} * {} with area : {} ", o.width, o.height, o.area());
}