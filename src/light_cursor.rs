
use embassy_rp::{
    gpio::{Level, Output, Pin},
    Peripheral,
};

use crate::button::ButtonPress;

const LIGHT_COUNT:usize = 4;

type Light<'p> = Output<'p>;

pub enum CursorDirection {
    Left,
    Right,
}

impl From<ButtonPress> for CursorDirection{
    fn from(value: ButtonPress) -> Self {
        match value {
            ButtonPress::One => CursorDirection::Left,
            ButtonPress::Two => CursorDirection::Right,
        }
    }
}
pub struct LightCursor<'p> {
    lights: [Light<'p>; LIGHT_COUNT],
    cursor_index: usize,
}

impl<'p> LightCursor<'p> {
    pub fn new(
        pin_1: impl Peripheral<P = impl Pin> + 'p,
        pin_2: impl Peripheral<P = impl Pin> + 'p,
        pin_3: impl Peripheral<P = impl Pin> + 'p,
        pin_4: impl Peripheral<P = impl Pin> + 'p,
    ) -> Self {
        Self {
            lights: [
                Output::new(pin_1, Level::Low),
                Output::new(pin_2, Level::Low),
                Output::new(pin_3, Level::Low),
                Output::new(pin_4, Level::Low),
            ],
            cursor_index: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        let old_cursor_index = self.cursor_index;

        self.lights[old_cursor_index].set_low();

       self.cursor_index = match direction {
            CursorDirection::Left => (old_cursor_index + LIGHT_COUNT - 1) % LIGHT_COUNT,
            CursorDirection::Right => (old_cursor_index + LIGHT_COUNT + 1) % LIGHT_COUNT,
        };
    }

    pub fn toggle(&mut self){
        self.lights[self.cursor_index].toggle();
    }
}
