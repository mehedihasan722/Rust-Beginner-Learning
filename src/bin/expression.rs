enum Access {
    Admin,
    Manager,
    User,
    Guest,
}
fn main() {
    //! secret file: admins only
    let access_level: Access = Access::Admin;
    let can_access: bool = match access_level {
        Access::Admin => true,
        _ => false,
    };
    if can_access {
        println!("You can access the admin panel");
    } else {
        println!("You cannot access the admin panel");
    }
}
