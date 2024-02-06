mod db;
mod ui;
mod utils;

use db::{Database, DATABASE};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use crate::utils::crypto::default_hash;

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
struct User {
    name: String,
    password: String,
    role: Role,
}

impl User {
    fn new(name: &str, password: &str, role: Role) -> Self {
        Self {
            name: name.to_string(),
            password: password.to_string(),
            role,
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let mut db = DATABASE.lock().unwrap();
        db.store_user(self)
    }

    fn get(username: &str) -> Option<Self> {
        let db = DATABASE.lock().unwrap();
        db.get_user(username)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            password: "".to_string(),
            role: Role::Reviewer,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
#[serde(tag = "name")]
enum Role {
    Reviewer,
    Owner { owned_establishment: String },
    Admin,
}

#[derive(Serialize, Hash)]
#[serde(tag = "name")]
enum Action {
    ReadOwn,
    Write,
    Read,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
#[display(
    fmt = r#"Avis sur "{}", par {}: "{}", {}/5"#,
    establishment,
    reviewer,
    comment,
    grade
)]
struct Review {
    establishment: String,
    reviewer: String,
    comment: String,
    grade: u8,
}

impl Review {
    fn new(establishment: &str, reviewer: &str, comment: &str, grade: u8) -> Self {
        Self {
            establishment: establishment.to_string(),
            reviewer: reviewer.to_string(),
            comment: comment.to_string(),
            grade,
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let mut db = DATABASE.lock().unwrap();
        db.store_review(self)
    }

    fn delete(&self) {
        let mut db = DATABASE.lock().unwrap();
        db.delete_review(&self.reviewer, &self.establishment);
    }

    /// Get a review made by a reviewer for an establishment
    fn get(reviewer: &str, establishment: &str) -> Option<Self> {
        let db = DATABASE.lock().unwrap();
        db.get_review(reviewer, establishment)
    }

    /// Get all reviews by a reviewer
    fn by(reviewer: &str) -> Vec<Self> {
        let db = DATABASE.lock().unwrap();
        db.get_reviews_by_reviewer(reviewer)
    }

    /// Get all reviews of an establishment
    fn of(establishment: &str) -> Vec<Self> {
        let db = DATABASE.lock().unwrap();
        db.get_reviews_of_establishment(establishment)
    }
}

// You can change the default content of the database by changing this `init` method
impl Database {
    fn init(&mut self) {
        let users = vec![
            User::new(
                "sire debeugg",
                "$argon2id$v=19$m=19456,t=2,p=1$7aap7ojd7U+62+pHXz7Maw$yz4Ron90Ith/YSLgvTfD84pt1Rc1cIQfrGiwMXNHzsQ", //0n_d17_ch1ffr3r_3t_p4s_crypt3r
                Role::Reviewer,
            ),
            User::new(
                "conte devvisse",
                "$argon2id$v=19$m=19456,t=2,p=1$ry6BCviMXpn3CWjSikEKLg$Ns5f/wccOFu/RvxJHwV7egyyED37wV3D2vOjpXWAi4g", //c41ss3-à-0ut1l
                Role::Owner {
                    owned_establishment: "mcdonalds".to_string(),
                },
            ),
            User::new(
                "thestrongestone",
                "$argon2id$v=19$m=19456,t=2,p=1$+gaJq6Rpy4LxQusNfI8oNQ$GRVKCLnl15TVhxglZX33biPWsEImSAqlu/UmnPK2lKc", //Sur terre comme au ciel, moi seul mérite d'être vénéré
                Role::Admin,
            ),
        ];

        let reviews = vec![
            Review::new("mcdonalds", "sire debeugg", "À fuire !", 1),
            Review::new("bistrot des lutins", "sire debeugg", "Au top !", 4),
            Review::new("cafétéria du coin", "sire debeugg", "Médiocre.", 2),
            Review::new("triple r", "conte devvisse", "Venez chez moi !", 1),
        ];

        for user in users {
            self.store_user(&user).unwrap();
        }

        for review in reviews {
            self.store_review(&review).unwrap();
        }
    }
}

fn main() {
    // Init default hash for non existing users
    default_hash();

    ui::start();

    DATABASE
        .lock()
        .unwrap()
        .save()
        .expect("impossible de sauvegarder la base de données");
}
