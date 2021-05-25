use serde::{Serialize, Deserialize};

pub struct UserModel {
    inner_user: InnerUser,
    relations_user: RelationsUser
}

#[derive(Serialize, Deserialize)]
struct InnerUser {
    id: String
}

#[derive(Serialize, Deserialize)]
struct RelationsUser { }

