#[derive(Debug)]
pub struct GetInfoResult {
    pub participant_type: String,
}

fn main() {

    let is_stake = true;
    let is_manager = true;


    if is_stake && is_manager {
        println!("Both");
    } else if is_manager {
        println!("Manager");
    } else if is_stake {
        println!("Stake");
    } else {
        println!("None")
    }


    // let participant_type = if is_stake && is_manager {
    //     "Both"
    // } else if is_manager {
    //     "Manager"
    // } else if is_stake {
    //     "Stakeholder"
    // } else {
    //     "None"
    // };
    let participant_type = "hello";
    let structure = GetInfoResult {
        participant_type: participant_type.to_string()
    };
    eprint!("{:#?}", structure);
}
