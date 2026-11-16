//! Things that happen on the trail.

#[derive(Clone, Copy)]
pub enum Event {
    /// A storm.
    Storm,
    /// Sickness that lasts this many days.
    Sickness(u32),
    /// Food found, in pounds.
    Windfall(u32),
}
