use std::io;

use crossterm::event::{self, Event};


pub fn next() -> Result<Event, String> {
    let event_result = event::read();
    let event = match (event_result) {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error whilst reading event: {error}"));
        }
    };

    return Ok(event);
}

// pub fn to_action(event: Event) -> Action {
//     match {

//     }
// }