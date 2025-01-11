struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn freeze() -> Self {
        Self {
            celsius: 0.0,
        }
    }
    fn show_celsius(&self) {
        println!("The temperature is {} degrees Celsius", self.celsius);
    }
}
fn main(){
    let temp: Temperature = Temperature {
        celsius: 22.5,
    };
    temp.show_celsius();
    let cold: Temperature = Temperature::freeze();
    cold.show_celsius();
}