#[macro_use]
extern crate rocket;

use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::status;
use std::collections::HashMap;

// Structure pour stocker les utilisateurs et leurs états de confirmation
#[derive(Debug, PartialEq, Eq)]
enum AccountStatus {
    Unconfirmed,
    Confirmed,
}

struct UserData {
    email: String,
    status: AccountStatus,
}

impl UserData {
    fn new(email: String) -> Self {
        UserData {
            email,
            status: AccountStatus::Unconfirmed,
        }
    }
}

// Structure pour stocker les utilisateurs
struct UserDatabase {
    users: HashMap<String, UserData>,
}

impl UserDatabase {
    fn new() -> Self {
        UserDatabase {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, uuid: String, email: String) {
        self.users.insert(uuid, UserData::new(email));
    }

    fn confirm_account(&mut self, uuid: &str) -> Result<(), &'static str> {
        if let Some(user) = self.users.get_mut(uuid) {
            user.status = AccountStatus::Confirmed;
            Ok(())
        } else {
            Err("Utilisateur non trouvé")
        }
    }
}

// Route pour confirmer un compte
#[get("/confirm/<uuid>")]
fn confirm_account(
    uuid: String,
    cookies: &CookieJar<'_>,
    user_db: &rocket::State<UserDatabase>,
) -> status::Custom<String> {
    match user_db.confirm_account(&uuid) {
        Ok(()) => {
            cookies.add(Cookie::new("user_id", uuid.clone()).same_site());
            status::Custom(
                Status::Ok,
                format!("Compte confirmé avec succès pour l'UUID : {}", uuid),
            )
        }
        Err(_) => status::Custom(Status::NotFound, "UUID invalide".to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    let user_db = UserDatabase::new();
    rocket::build()
        .mount("/", routes![confirm_account])
        .manage(user_db)
}
