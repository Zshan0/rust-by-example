use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

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

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // From & To
    let user = UserRole::Stackholder;
    println!("{}", user.to_string());
    let other_user = UserRole::from_str("stackholdermanager");
    // let other_user = UserRole::StackholderManager;
    println!("{}", other_user.unwrap().to_string());

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
