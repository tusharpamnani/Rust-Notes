struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }
    fn perimeter(&self) -> u32 {
       2 * (self.width + self.height);
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
    print!("The perimeter of the rectangle is {}", rect.primeter());
}
