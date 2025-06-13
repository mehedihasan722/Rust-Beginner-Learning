enum Light {
    Bright,
    Dim,
}

fn display_Light(light: &Light) {
    match light {
        Light::Bright => println!("The light is bright!"),
        Light::Dim => println!("The light is dim!"),
    }
}

fn main() {
    //! secret file: admins only
    let light_status: Light = Light::Bright;
    display_Light(&light_status);
    display_Light(&light_status);
    // Uncomment the line below to test the dim light
    // let light_status: light = light::Dim;
    // DisplayLight(light_status);
}
