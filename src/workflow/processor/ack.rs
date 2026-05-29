#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AckDecision {
    Ack,
    DoNotAck,
}

impl AckDecision {
    pub fn should_ack(self) -> bool {
        matches!(self, Self::Ack)
    }
}
