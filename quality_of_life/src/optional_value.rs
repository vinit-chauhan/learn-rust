#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Invite {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invite {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invite {
        Invite {
            invitee,
            attending,
            message,
        }
    }
}

pub fn exec() {
    let invite_1 = Invite::new("Matt".to_string(), true, None);
    dbg!(invite_1.clone());

    let invite_1: Invite = Invite::new(
        String::from("Matt"),
        false,
        Some("I won' be able to make it.".to_string()),
    );
    dbg!(invite_1.clone());

    if invite_1.message.is_some() {
        println!("There's some message.");
    } else if invite_1.message.is_none() {
        println!("There's no message.");
    }
}
