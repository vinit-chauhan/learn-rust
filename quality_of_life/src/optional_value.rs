#[derive(Debug)]
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
    dbg!(invite_1);
    let invite_1 = Invite::new(
        String::from("Matt"),
        false,
        Some("I won' be able to make it.".to_string()),
    );
    dbg!(invite_1);
}
