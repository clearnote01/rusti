use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    id: i32,
    name: String,
    location: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub fork: bool,
    // html_url: String,
    // created_at: String,
    // updated_at: String,
    //pub language: String,
    pub languages_url: String,
    //
}

pub type LanguageStats = HashMap<String, i32>;
