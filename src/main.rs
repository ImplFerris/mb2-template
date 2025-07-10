#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};
{% if use_embassy %}
use embassy_executor::Spawner;
use embassy_time::Timer;
{% if abs_layer == "BSP" -%}
use microbit_bsp::Microbit;
{% endif -%}
{% else %}
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
{% if abs_layer == "BSP" -%}
use microbit::{hal::Timer, Board};
{% endif -%}
{% if abs_layer == "HAL" -%}
use nrf52833_hal::pac::Peripherals;
use nrf52833_hal::timer::Timer;
{% endif -%}
{% endif -%}

{% if use_embassy %}
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    {% if abs_layer == "BSP" -%}
    let board = Microbit::default();
    {% endif -%}
    {% if abs_layer == "HAL" -%}
    let p = embassy_nrf::init(Default::default());
    {% endif %}
    loop {
        Timer::after_secs(1).await;
    }
}
{% else %}
#[entry]
fn main() -> ! {
    {% if abs_layer == "BSP" -%}
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    {% endif -%}
    {% if abs_layer == "HAL" -%}
    let peripherals = Peripherals::take().unwrap();
    let mut timer = Timer::new(peripherals.TIMER0);
    {% endif %}
    loop {
        timer.delay_ms(500);
    }
}
{% endif -%}
