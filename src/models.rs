use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub user_id: String,
    pub first_name: String,
    pub second_name: String,
    pub birthdate: NaiveDate,
    pub gender: Option<Gender>,
    pub city: Option<String>,
    pub biography: Option<String>,
    pub password_hash: String,
}
