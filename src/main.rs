use std::time::Duration;

use esp_idf_svc::hal::{
    gpio::{AnyOutputPin, Output, OutputPin, PinDriver},
    prelude::Peripherals,
};

struct BitCounter {
    bit_0: PinDriver<'static, AnyOutputPin, Output>,
    bit_1: PinDriver<'static, AnyOutputPin, Output>,
    bit_2: PinDriver<'static, AnyOutputPin, Output>,
    bit_3: PinDriver<'static, AnyOutputPin, Output>,
    bit_4: PinDriver<'static, AnyOutputPin, Output>,
    bit_5: PinDriver<'static, AnyOutputPin, Output>,
    bit_6: PinDriver<'static, AnyOutputPin, Output>,
    bit_7: PinDriver<'static, AnyOutputPin, Output>,
}

impl BitCounter {
    pub fn new(
        bit_0: PinDriver<'static, AnyOutputPin, Output>,
        bit_1: PinDriver<'static, AnyOutputPin, Output>,
        bit_2: PinDriver<'static, AnyOutputPin, Output>,
        bit_3: PinDriver<'static, AnyOutputPin, Output>,
        bit_4: PinDriver<'static, AnyOutputPin, Output>,
        bit_5: PinDriver<'static, AnyOutputPin, Output>,
        bit_6: PinDriver<'static, AnyOutputPin, Output>,
        bit_7: PinDriver<'static, AnyOutputPin, Output>,
    ) -> Self {
        Self {
            bit_0,
            bit_1,
            bit_2,
            bit_3,
            bit_4,
            bit_5,
            bit_6,
            bit_7,
        }
    }

    pub fn change_state(&mut self, number: u8) {
        // Check each bit and set the corresponding pin state
        if (number & 0b00000001) != 0 {
            self.bit_0.set_high().unwrap();
        } else {
            self.bit_0.set_low().unwrap();
        }

        if (number & 0b00000010) != 0 {
            self.bit_1.set_high().unwrap();
        } else {
            self.bit_1.set_low().unwrap();
        }

        if (number & 0b00000100) != 0 {
            self.bit_2.set_high().unwrap();
        } else {
            self.bit_2.set_low().unwrap();
        }

        if (number & 0b00001000) != 0 {
            self.bit_3.set_high().unwrap();
        } else {
            self.bit_3.set_low().unwrap();
        }

        if (number & 0b00010000) != 0 {
            self.bit_4.set_high().unwrap();
        } else {
            self.bit_4.set_low().unwrap();
        }

        if (number & 0b00100000) != 0 {
            self.bit_5.set_high().unwrap();
        } else {
            self.bit_5.set_low().unwrap();
        }

        if (number & 0b01000000) != 0 {
            self.bit_6.set_high().unwrap();
        } else {
            self.bit_6.set_low().unwrap();
        }

        if (number & 0b10000000) != 0 {
            self.bit_7.set_high().unwrap();
        } else {
            self.bit_7.set_low().unwrap();
        }
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let bit_0 = PinDriver::output(peripherals.pins.gpio4.downgrade_output())
        .expect("No se pudo acceder al pin 4");
    let bit_1 = PinDriver::output(peripherals.pins.gpio5.downgrade_output())
        .expect("No se pudo acceder al pin 5");
    let bit_2 = PinDriver::output(peripherals.pins.gpio6.downgrade_output())
        .expect("No se pudo acceder al pin 6");
    let bit_3 = PinDriver::output(peripherals.pins.gpio7.downgrade_output())
        .expect("No se pudo acceder al pin 7");
    let bit_4 = PinDriver::output(peripherals.pins.gpio15.downgrade_output())
        .expect("No se pudo acceder al pin 15");
    let bit_5 = PinDriver::output(peripherals.pins.gpio16.downgrade_output())
        .expect("No se pudo acceder al pin 16");
    let bit_6 = PinDriver::output(peripherals.pins.gpio17.downgrade_output())
        .expect("No se pudo acceder al pin 17");
    let bit_7 = PinDriver::output(peripherals.pins.gpio18.downgrade_output())
        .expect("No se pudo acceder al pin 18");

    let mut bit_counter = BitCounter::new(bit_0, bit_1, bit_2, bit_3, bit_4, bit_5, bit_6, bit_7);
    let mut counter = 0;

    loop {
        bit_counter.change_state(counter);

        if counter == 255 {
            counter = 0;
        } else {
            counter += 1;
        }
        std::thread::sleep(Duration::from_millis(750));
    }
}
