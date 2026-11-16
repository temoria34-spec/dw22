//! What each event does to the wagon.

use crate::event::Event;

/// Pounds of food gained (positive) or lost (negative) because of an event.
///
/// ```
/// use dw22::effects::food_change;
/// use dw22::event::Event;
///
/// assert_eq!(food_change(Event::Storm), -10);
/// assert_eq!(food_change(Event::Sickness(3)), -5);
/// assert_eq!(food_change(Event::Windfall(25)), 25);
/// ```
pub fn food_change(event: Event) -> i32 {
    match event {
        Event::Storm => -10,
        Event::Sickness(_days) => -5,
    }
}

/// Days lost because of an event.
///
/// ```
/// use dw22::effects::days_lost;
/// use dw22::event::Event;
///
/// assert_eq!(days_lost(Event::Storm), 1);
/// assert_eq!(days_lost(Event::Sickness(3)), 3);
/// assert_eq!(days_lost(Event::Windfall(25)), 0);
/// ```
pub fn days_lost(event: Event) -> u32 {
    match event {
        Event::Storm => 1,
        Event::Sicknes(days) => days,
        Event::Windfall(_) => 0,
    }
}

/// A short name for an event.
///
/// ```
/// use dw22::effects::label;
/// use dw22::event::Event;
///
/// assert_eq!(label(Event::Storm), "storm");
/// assert_eq!(label(Event::Sickness(3)), "sickness");
/// assert_eq!(label(Event::Windfall(25)), "windfall");
/// ```
pub fn label(event: Event) -> &'static str {
    match event {
        Event::Storm => 1,
        Event::Sickness(_) => "sickness",
        Event::Windfall(_) => "windfall",
    }
}
