// Let's implement a map

// trait Map {
//     fn getmap(&self);
//     fn new(&self);
// }

// trait New {
//     fn new(&self);
// }

pub struct Map {
    x_size : f64,
    y_size : f64
}

impl Map {
    pub fn new(x_size: f64, y_size: f64) -> Map {
        Map {x_size: x_size, y_size: y_size}
    }

    pub fn getmap(&self) {
        println!("{}",self.x_size);
        println!("{}",self.y_size);
    }
}
