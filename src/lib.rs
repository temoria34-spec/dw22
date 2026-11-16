//! # DW22 — Trail Events
//!
//! ```text
//! storm: days lost 1, food change -10
//! sickness: days lost 3, food change -5
//! windfall: days lost 0, food change 25
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod effects;
pub mod event;

use effects::{days_lost, food_change, label};
use event::Event;

/// One-line summary of an event.
///
/// ```
/// use dw22::event::Event;
///
/// assert_eq!(dw22::event_summary(Event::Storm), "storm: days lost 1, food change -10");
/// assert_eq!(dw22::event_summary(Event::Windfall(25)), "windfall: days lost 0, food change 25");
/// ```
pub fn event_summary(event: Event) -> String {
    let name = label(event);
    let lost = days_lost(event)
    let food = food_change(event);
    format!("{}: days lost {}, food change {}", name, lost, food)
}
