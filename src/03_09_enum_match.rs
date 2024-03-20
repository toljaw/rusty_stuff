#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin
}

impl PermissionLevel {
    fn description(&self) -> String {
        match self {                                        // match: compare with switch from Python
            PermissionLevel::User => String::from("I'm an User"),
            PermissionLevel::Instructor => String::from("I'm an Instructor"),
            PermissionLevel::Admin => String::from("I'm an Admin"),
        }
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}",user1);
    println!("{}", user1.description());
    println!();

    let user2 = PermissionLevel::Instructor;
    println!("{:?}",user2);
    println!("{}", user2.description());
    println!();

    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.description());
}