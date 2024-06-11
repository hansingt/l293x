#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;
use esp_hal::clock::ClockControl;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Io, Level, Output};
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::peripherals::Peripherals;
use esp_hal::prelude::*;
use esp_hal::system::SystemControl;
use esp_println::println;
use l293x::L293x;

#[entry]
fn main() -> ! {
    // Get the peripherals, system and clock controls
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initialize access to the IO ports
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Configure the PWM generator
    // We use a 50Hz timer to achieve 20ms pulse width.
    // A period of 20_000 then defines 1us per tick.
    let clock_cfg = PeripheralClockConfig::with_prescaler(&clocks, 0);
    let mut mcpwm0 = McPwm::new(peripherals.MCPWM0, clock_cfg);
    mcpwm0.operator0.set_timer(&mcpwm0.timer0);
    mcpwm0.operator1.set_timer(&mcpwm0.timer0);
    mcpwm0.timer0.start(
        clock_cfg
            .timer_clock_with_frequency(20_000 - 1, PwmWorkingMode::Increase, 50u32.Hz())
            .unwrap(),
    );

    // Initialize the IO-Pins
    // GPIO19 / GPIO18: Motor1
    let (m1_fwd, m1_rev) = mcpwm0.operator0.with_pins(
        io.pins.gpio19,
        PwmPinConfig::UP_ACTIVE_HIGH,
        io.pins.gpio18,
        PwmPinConfig::UP_ACTIVE_HIGH,
    );
    // GPIO17 / GPIO16: Motor2
    let (m2_fwd, m2_rev) = mcpwm0.operator1.with_pins(
        io.pins.gpio17,
        PwmPinConfig::UP_ACTIVE_HIGH,
        io.pins.gpio16,
        PwmPinConfig::UP_ACTIVE_HIGH,
    );

    // Now, configure the L293 chip
    let mut l293 = L293x::new(
        m1_fwd,
        m1_rev,
        m2_fwd,
        m2_rev,
        Output::new(io.pins.gpio5, Level::Low), // Motor1 enable
        Output::new(io.pins.gpio4, Level::Low), // Motor1 enable
    );

    // Create a delay to allow waiting to a specific time
    let delay = Delay::new(&clocks);

    l293.enable_y1_and_y2().unwrap();
    l293.enable_y3_and_y4().unwrap();
    loop {
        println!("Forward!");
        l293.set_y1_duty_cycle_percent(100).unwrap();
        l293.set_y2_duty_cycle_percent(0).unwrap();
        l293.set_y3_duty_cycle_percent(100).unwrap();
        l293.set_y4_duty_cycle_percent(0).unwrap();
        delay.delay_millis(1000);

        println!("Stop!");
        l293.set_y1_duty_cycle_percent(0).unwrap();
        l293.set_y2_duty_cycle_percent(0).unwrap();
        l293.set_y3_duty_cycle_percent(0).unwrap();
        l293.set_y4_duty_cycle_percent(0).unwrap();
        delay.delay_millis(500);

        println!("Reverse!");
        l293.set_y1_duty_cycle_percent(0).unwrap();
        l293.set_y2_duty_cycle_percent(100).unwrap();
        l293.set_y3_duty_cycle_percent(0).unwrap();
        l293.set_y4_duty_cycle_percent(100).unwrap();
        delay.delay_millis(1000);

        println!("Stop!");
        l293.set_y1_duty_cycle_percent(0).unwrap();
        l293.set_y2_duty_cycle_percent(0).unwrap();
        l293.set_y3_duty_cycle_percent(0).unwrap();
        l293.set_y4_duty_cycle_percent(0).unwrap();
        delay.delay_millis(500);
    }
}
