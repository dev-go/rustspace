#[derive(Debug)]
pub enum Role {
    Admin,
    Vip,
    User,
}

pub struct User {
    role: Role,
    id: u64,
    pwd: String,
    pub name: String,
    desc: String,
    money: u64,
}

impl User {
    pub fn new_user(name: String, pwd: String) -> User {
        User {
            name,
            pwd,
            role: Role::User,
            id: 10001,
            desc: String::from(""),
            money: 0,
        }
    }

    pub fn show(&self) {
        println!(
            "User{{role: {:?}, id: {:?}, name: {:?}, desc: {:?}, money: {:?} }}",
            self.role, self.id, self.name, self.desc, self.money
        );
    }
}
