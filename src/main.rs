use dw22::event::Event;

fn main() {
    println!("{}", dw22::event_summary(Event::Storm));
    println!("{}", dw22::event_summary(Event::Sickness(3)));
    println!("{}", dw22::event_summary(Event::Windfall(25)));
}
