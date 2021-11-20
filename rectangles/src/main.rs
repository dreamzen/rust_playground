fn main() {
    let width = 3;
    let height = 4;
    println!("area = {}", area(width, height));

    let rect = (3, 4);
    println!("area2 = {}", area2(rect));

    let rect = RectangleTuple(3, 4);
    println!("area3 = {}", area3(rect));

    let rect = Rectangle {
        width: 3,
        height: 4,
    };
    println!("area4 = {}", area4(&rect));
    println!("area4 = {}", rect.area());
    println!("rect = {:#?}", rect);

    let sub_rect = Rectangle {
        width: 3,
        height: 3,
    };
    println!("can_hold = {}", rect.can_hold(&sub_rect));
    let sub_rect = Rectangle {
        width: 4,
        height: 2,
    };
    println!("can_hold = {}", rect.can_hold(&sub_rect));
}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rt: RectangleTuple) -> u32 {
    rt.0 * rt.1
}

fn area4(r: &Rectangle) -> u32 {
    r.width * r.height
}

struct RectangleTuple(u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rt: &Rectangle) -> bool {
        if self.height >= rt.height && self.width >= rt.width {
            return true;
        }
        if self.width >= rt.height && self.height >= rt.width {
            return true;
        }
        false
    }
}