use casbin::{CoreApi, Enforcer};
use crate::{Action, User};
use lazy_static::lazy_static;
use std::sync::Mutex;
use casbin::Result as CasbinResult;
use tokio::runtime::Runtime;

lazy_static! {
    static ref ENFORCER: Mutex<Enforcer> = Mutex::new(
        Runtime::new().unwrap().block_on(async {
            Enforcer::new("./casbin/model.conf", "./casbin/policy.csv").await.unwrap()
        })
    );
}

pub fn check_permission(sub: &User, obj: &str, act: Action) -> CasbinResult<bool> {
    let enforcer = ENFORCER.lock().unwrap();
    enforcer.enforce((sub, obj, act))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::Role;

    #[rstest(user_role, establishment, action, expected,
    case(Role::Reviewer, "notMyEstablishment", Action::ReadOwn, true),
    case(Role::Reviewer, "notMyEstablishment", Action::Write, true),
    case(Role::Reviewer, "notMyEstablishment", Action::Read, false),
    case(Role::Reviewer, "notMyEstablishment", Action::Delete, false),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "notMyEstablishment", Action::ReadOwn, true),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "myEstablishment", Action::Write, false),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "notMyEstablishment", Action::Write, true),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "myEstablishment", Action::Read, true),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "notMyEstablishment", Action::Read, false),
    case(Role::Owner { owned_establishment: "myEstablishment".to_string() }, "notMyEstablishment", Action::Delete, false),
    case(Role::Admin, "notMyEstablishment", Action::ReadOwn, true),
    case(Role::Admin, "notMyEstablishment", Action::Write, true),
    case(Role::Admin, "notMyEstablishment", Action::Read, true),
    case(Role::Admin, "notMyEstablishment", Action::Delete, true)
    )]
    fn test_permissions(user_role: Role, establishment: &str, action: Action, expected: bool) {
        let user_name = match user_role {
            Role::Reviewer => "reviewer",
            Role::Admin => "admin",
            Role::Owner { .. } => "owner",
        };
        let user = User::new(user_name, user_name, user_role);
        assert_eq!(check_permission(&user, establishment, action).unwrap(), expected);
    }
}

