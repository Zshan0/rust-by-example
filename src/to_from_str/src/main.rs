use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum UserRole {
    Stackholder,
    Manager,
    StackholderManager,
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stackholder" => Ok(Self::Stackholder),
            "stackholdermanager" => Ok(Self::StackholderManager),
            "manager" => Ok(Self::Manager),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Stackholder => "stackholder",
                Self::Manager => "manager",
                Self::StackholderManager => "stackholdermanager",
            }
        )
    }
}

fn main() {
    let user = UserRole::Stackholder;
    println!("{}", user.to_string());
    let other_user = UserRole::from_str("stackholdermanager");
    // let other_user = UserRole::StackholderManager;
    println!("{}", other_user.unwrap().to_string());
}
