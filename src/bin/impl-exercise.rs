enum Color {
    Brown,
    Red,
}

impl  Color {
    fn show(&self) {
        match self {
            Color::Brown => println!("The color is Brown"),
            Color::Red => println!("The color is Red"),
        }
    }
    
}

struct Dimension {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimension {
    fn show(&self) {
        println!("The height is {}, width is {}, depth is {}", self.height, self.width, self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimension,
}

impl ShippingBox {
    fn new(color: Color, weight: f64, dimensions : Dimension) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.show();
        println!("The weight is {}", self.weight);
        self.dimensions.show();
    }

  
}

fn main() {
    let small_dimension: Dimension = Dimension {
        height: 10.0,
        width: 5.0,
        depth: 5.0,
    };
    let small_box: ShippingBox = ShippingBox::new(Color::Brown, 5.0, small_dimension); 
    small_box.print();

    let large_dimension: Dimension = Dimension {
        height: 20.0,
        width: 10.0,
        depth: 10.0,
    };
    let large_box: ShippingBox = ShippingBox::new(Color::Red, 10.0, large_dimension);
    large_box.print();
}