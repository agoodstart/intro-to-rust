#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }

    fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {        
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    fn perimeter(&self) -> f64 {
        println!("{:#?}", self);

        2.0 * ((self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs())
    }
}

fn create_rec(p1: Point, p2: Point) -> Rectangle {
    Rectangle {
        p1,
        p2
    }
}

pub fn run() {
    // let new_rec = create_rec(Point::origin(), Point::new(3.0, 4.0));
    // let rec_area = new_rec.area();
    // let rec_per = new_rec.perimeter();

    // // println!("Rectangle data: {:#?}", new_rec);
    // println!("Rectangle area: {}", rec_area);
    // println!("Rectangle perimeter: {}", rec_per);

    //

    let another_rec = create_rec(Point::new(7.0, 1.0), Point::new(5.0, 6.0));
    let another_rec_area = another_rec.area();
    let another_rec_perimeter = another_rec.perimeter();

    println!("Rectangle area: {}", another_rec_area);
    println!("Rectangle perimeter: {}", another_rec_perimeter);
}