use crate::participant::Participant;

pub struct Server<'a> {
    participants: Vec<&'a Participant>
}

impl <'a> Server<'a> {
    pub fn new() -> Self {
        Self {
            participants: vec![]
        }
    }

    pub fn register(&mut self, new_participant: &'a Participant) {
        self.participants.push(new_participant);
    }
}
