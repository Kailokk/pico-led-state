#![no_std]
#![no_main]

mod button;
mod light_cursor;

use crate::button::ButtonPress;
use crate::light_cursor::LightCursor;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{AnyPin, Pin, Pull};
use embassy_rp::{self as hal, gpio::Input};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use futures::{select_biased, FutureExt};

use {defmt_rtt as _, panic_probe as _};

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

static CHANNEL: Channel<ThreadModeRawMutex, ButtonPress, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut lights = LightCursor::new(p.PIN_0, p.PIN_1, p.PIN_2, p.PIN_3);

    spawner
        .spawn(button_task(p.PIN_15.degrade(), ButtonPress::One))
        .unwrap();
    spawner
        .spawn(button_task(p.PIN_16.degrade(), ButtonPress::Two))
        .unwrap();


    loop {
        lights.toggle();
        select_biased! {
            button = CHANNEL.receive().fuse() => {
                lights.move_cursor(button.into());
            }
            _ = Timer::after_millis(500).fuse() => {}
        }
    }
}

#[embassy_executor::task(pool_size = 2)]
async fn button_task(pin: AnyPin, press: ButtonPress) {
    let mut input = Input::new(pin, Pull::None);
    loop {
        input.wait_for_low().await;
        CHANNEL.send(press).await;
        Timer::after_millis(100).await;
        input.wait_for_high().await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
