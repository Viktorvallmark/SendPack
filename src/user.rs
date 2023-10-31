
use uuid::{Uuid};
use serde::{Serialize, Deserialize};

//#[derive(Serialize)]
pub struct User {
    id: Uuid,
   pub name: String,
   pub pwd: String,
   pub role: Role
}

pub enum Role {
    Sender,
    Deliverer,
    Admin,
}

impl User {
    pub fn new(name: String, pwd: String, role: Role) -> User {

        User {
            id: Uuid::new_v4(),
            name: String::from(name),
            pwd: String::from(pwd),
            role,
        }
    }
}