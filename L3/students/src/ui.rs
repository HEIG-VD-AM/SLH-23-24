use crate::{Action, Review, Role, User};
use derive_more::Display;
use inquire::{Confirm, CustomType, Password, Select, Text};
use strum::{EnumIter, IntoEnumIterator};
use crate::utils::authorization::{check_permission};
use crate::utils::consts::{COMMENT_MAX_LENGTH, COMMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH, ESTABLISHMENT_MIN_LENGTH, MAX_GRADE_REVIEW, MIN_GRADE_REVIEW, USERNAME_MAX_LENGTH, USERNAME_MIN_LENGTH};
use crate::utils::crypto::{default_hash, hash_password, verify_password};
use crate::utils::input_val::{is_grade_valid, is_password_valid, is_comment_length_valid, is_establishment_length_valid, is_username_length_valid};

enum ShouldContinue {
    Yes,
    No,
}

pub fn start() {
    loop_menu(main_menu);
}

fn loop_menu<F>(menu_handler: F)
where
    F: Fn() -> ShouldContinue,
{
    loop {
        match menu_handler() {
            ShouldContinue::Yes => continue,
            ShouldContinue::No => break,
        }
    }
}

fn main_menu() -> ShouldContinue {
    #[derive(EnumIter, Display)]
    enum Choice {
        #[display(fmt = "Se connecter")]
        Login,

        #[display(fmt = "S'inscrire")]
        Register,

        #[display(fmt = "Quitter")]
        Exit,
    }

    let choice = if let Ok(input) = Select::new("Que voulez-vous faire ?", Choice::iter().collect()).prompt() {
        input
    } else {
        println!("Erreur lors de la saisie du nom de l'établissement");
        return ShouldContinue::Yes;
    };

    match choice {
        Choice::Login => login(),
        Choice::Register => register(),
        Choice::Exit => ShouldContinue::No,
    }
}

fn login() -> ShouldContinue {

    let username = match Text::new("Entrez votre nom d'utilisateur : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom de l'utilisateur.");
            return ShouldContinue::Yes;
        }
    };

    if !is_username_length_valid(&username) {
        println!("Erreur lors de la saisie du nom d'utilisateur, il doit faire entre {} et {} caractères.", USERNAME_MIN_LENGTH, USERNAME_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    let password = match Password::new("Entrez votre mot de passe : ")
        .without_confirmation()
        .prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Erreur lors de la saisie du mot de passe.");
            return ShouldContinue::Yes;
        }
    };

    let mut user = User::default();
    let ok : bool = match User::get(&username) {
        Some(user_bd) => {
            user = user_bd;
            verify_password(&password, &user.password)
        },
        None => {
            verify_password(&password, &default_hash());
            false
        }
    };

    if ok {
        loop_menu(|| user_menu(&user));
    } else {
        println!("Nom d'utilisateur ou mot de passe incorrect.");
    }

    ShouldContinue::Yes
}

fn register() -> ShouldContinue {

    let username = match Text::new("Entrez votre nom d'utilisateur : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom d'utilisateur.");
            return ShouldContinue::Yes;
        }
    };

    if !is_username_length_valid(&username) {
        println!("Erreur lors de la saisie du nom d'utilisateur, il doit faire entre {} et {} caractères.", USERNAME_MIN_LENGTH, USERNAME_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    let mut password = match Password::new("Entrez votre mot de passe : ")
        .with_custom_confirmation_message("Confirmez votre mot de passe : ")
        .with_custom_confirmation_error_message("Les mots de passe ne correspondent pas.")
        .prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Erreur lors de la saisie du mot de passe.");
            return ShouldContinue::Yes;
        }
    };

    match is_password_valid(&username, &password) {
        true => {
            password = hash_password(password.as_str()).unwrap();
        },
        false => {
            println!("Mot de passe trop faible.");
            return ShouldContinue::No;
        }
    }

    let is_owner = match Confirm::new("Êtes-vous propriétaire d'un établissement ?")
        .with_default(false)
        .prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Erreur lors de la confirmation de la propriété de l'établissement.");
            return ShouldContinue::Yes;
        }
    };

    let role = if is_owner {
        match Text::new("Entrez le nom de l'établissement : ")
            .prompt() {
            Ok(input) => {
                let owned_establishment = input.trim().to_ascii_lowercase();

                if !is_establishment_length_valid(&owned_establishment) {
                    println!("Erreur lors de la saisie du nom de l'établissement, il doit faire entre {} et {} caractères.", ESTABLISHMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH);
                    return ShouldContinue::Yes;
                }

                Role::Owner { owned_establishment }
            },
            Err(_) => {
                println!("Erreur lors de la saisie du nom de l'établissement.");
                return ShouldContinue::Yes;
            }
        }
    } else {
        Role::Reviewer
    };

    let user = if let Some(_) = User::get(&username) {
        println!("Erreur lors de la création de l'utilisateur.");
        return ShouldContinue::Yes;
    } else {
        User::new(&username, &password, role)
    };

    match user.save() {
        Ok(_) => println!("Utilisateur créé avec succès."),
        Err(_) => println!("Erreur lors de la création de l'utilisateur."),
    }

    ShouldContinue::Yes
}

// -----------------------------------------------------------------------------------------------

fn user_menu(user: &User) -> ShouldContinue {
    #[derive(EnumIter, Display)]
    enum Choice {
        #[display(fmt = "Mes avis")]
        ListOwnReviews,

        #[display(fmt = "Ajouter un avis")]
        AddReview,

        #[display(fmt = "Avis d'un établissement")]
        ListEstablishmentReviews,

        #[display(fmt = "Supprimer un avis")]
        DeleteReview,

        #[display(fmt = "Se déconnecter")]
        Logout,
    }

    let choice = match Select::new("Que voulez-vous faire ?", Choice::iter().collect()).prompt() {
        Ok(choice) => choice,
        Err(..) => return ShouldContinue::Yes,
    };

    match choice {
        Choice::ListOwnReviews => list_own_reviews(user),
        Choice::AddReview => add_review(user),
        Choice::ListEstablishmentReviews => list_establishment_reviews(user),
        Choice::DeleteReview => delete_review(user),
        Choice::Logout => ShouldContinue::No,
    }
}

fn list_own_reviews(user: &User) -> ShouldContinue {
    if let Ok(authorized) = check_permission(&user, "", Action::ReadOwn) {
        if !authorized {
            println!("Vous n'avez pas la permission de lire les avis que vous avez écrit.");
            return ShouldContinue::Yes;
        }
    } else {
        println!("Erreur côté serveur, veuillez réessayer plus tard.");
        return ShouldContinue::Yes;
    }

    for review in Review::by(&user.name) {
        println!("{}", review);
    }

    ShouldContinue::Yes
}

fn add_review(user: &User) -> ShouldContinue {

    let establishment = match Text::new("Entrez le nom de l'établissement : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom de l'établissement.");
            return ShouldContinue::Yes;
        }
    };

    if !is_establishment_length_valid(&establishment) {
        println!("Erreur lors de la saisie du nom de l'établissement, il doit faire entre {} et {} caractères.", ESTABLISHMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH);
        return ShouldContinue::Yes;
    }


    if let Ok(authorized) = check_permission(&user, &establishment, Action::Write) {
        if !authorized {
            println!("Vous n'avez pas la permission d'ajouter un avis sur cet établissement.");
            return ShouldContinue::Yes;
        }
    } else {
        println!("Erreur côté serveur, veuillez réessayer plus tard.");
        return ShouldContinue::Yes;
    }

    let comment = match Text::new("Entrez votre commentaire : ")
        .prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Erreur lors de la saisie du commentaire.");
            return ShouldContinue::Yes;
        }
    };

    if !is_comment_length_valid(&comment) {
        println!("Erreur lors de la saisie du commentaire, il doit faire entre {} et {} caractères.", COMMENT_MIN_LENGTH, COMMENT_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    let grade = match CustomType::<u8>::new("Entrez votre note : ")
        .with_error_message("Merci d'entrer une valeur numérique.")
        .prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("Erreur lors de la saisie de la note.");
            return ShouldContinue::Yes;
        }
    };

    if !is_grade_valid(grade) {
        println!("Note invalide ! Il faut une note entre {} et {}.", MIN_GRADE_REVIEW, MAX_GRADE_REVIEW);
        return ShouldContinue::Yes;
    }

    let review = if let Some(_) = Review::get(&user.name, &establishment) {
        println!("Vous avez déjà un avis sur cet établissement.");
        return ShouldContinue::Yes;
    } else {
        Review::new(&establishment, &user.name, &comment, grade)
    };

    match review.save() {
        Ok(_) => println!("Avis ajouté avec succès."),
        Err(_) => println!("Erreur lors de l'ajout de l'avis."),
    }

    ShouldContinue::Yes
}


fn list_establishment_reviews(user: &User) -> ShouldContinue {

    let establishment = match Text::new("Entrez le nom de l'établissement : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom de l'établissement.");
            return ShouldContinue::Yes;
        }
    };

    if !is_establishment_length_valid(&establishment) {
        println!("Erreur lors de la saisie du nom de l'établissement, il doit faire entre {} et {} caractères.", ESTABLISHMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    if let Ok(authorized) = check_permission(&user, &establishment, Action::Read) {
        if !authorized {
            println!("Vous n'avez pas la permission de lire les avis sur cet établissement.");
            return ShouldContinue::Yes;
        }
    } else {
        println!("Erreur côté serveur, veuillez réessayer plus tard.");
        return ShouldContinue::Yes;
    }

    for review in Review::of(&establishment) {
        println!("{}", review);
    }

    ShouldContinue::Yes
}

fn delete_review(user: &User) -> ShouldContinue {

    let establishment = match Text::new("Entrez le nom de l'établissement : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom de l'établissement.");
            return ShouldContinue::Yes;
        }
    };

    if !is_establishment_length_valid(&establishment) {
        println!("Erreur lors de la saisie du nom de l'établissement, il doit faire entre {} et {} caractères.", ESTABLISHMENT_MIN_LENGTH, ESTABLISHMENT_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    if let Ok(authorized) = check_permission(&user, &establishment, Action::Delete) {
        if !authorized {
            println!("Vous n'avez pas la permission de supprimer un avis sur cet établissement");
            return ShouldContinue::Yes;
        }
    } else {
        println!("Erreur côté serveur, veuillez réessayer plus tard");
        return ShouldContinue::Yes;
    }

    let name = match Text::new("Entrez le nom de l'auteur de l'avis : ")
        .prompt() {
        Ok(input) => input.trim().to_ascii_lowercase(),
        Err(_) => {
            println!("Erreur lors de la saisie du nom de l'auteur de l'avis.");
            return ShouldContinue::Yes;
        }
    };

    if !is_username_length_valid(&name) {
        println!("Erreur lors de la saisie du nom de l'utilisateur, il doit faire entre {} et {} caractères.", USERNAME_MIN_LENGTH, USERNAME_MAX_LENGTH);
        return ShouldContinue::Yes;
    }

    match Review::get(&name, &establishment) {
        Some(review) => {
            println!("Avis supprimé avec succès.");
            review.delete()
        },
        None => {
            println!("Aucun avis trouvé.");
        }
    };

    ShouldContinue::Yes
}

