pub trait StorableEvent {
    fn event_type(&self) -> &str;
    fn serialize(&self) -> String;
}