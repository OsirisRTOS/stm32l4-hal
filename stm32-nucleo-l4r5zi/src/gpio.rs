use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::marker::Copy;
use core::ops::BitAnd;
/*==============CONSTS========*/
const GPIOA_BASE:u32 = 0x4800_0000;
const GPIOB_BASE:u32 = 0x4800_0400;
const GPIOC_BASE:u32 = 0x4800_0800;
const GPIOD_BASE:u32 = 0x4800_0C00;
const GPIOE_BASE:u32 = 0x4800_1000;
const GPIOF_BASE:u32 = 0x4800_1400;
const GPIOG_BASE:u32 = 0x4800_1800;
const GPIOH_BASE:u32 = 0x4800_1C00;
const GPIOI_BASE:u32 = 0x4800_2000;
const OTYPER_OFFSET:u8= 0x04;
const OSPEEDR_OFFSET:u8= 0x08;
const PUPDR_OFFSET:u8= 0x0C;
const IDR_OFFSET:u8= 0x10;
const ODR_OFFSET:u8= 0x14;
const BSRR_OFFSET:u8= 0x18;
const LCKR_OFFSET:u8= 0x1C;
const AFRL_OFFSET:u8= 0x20;
const AFRH_OFFSET:u8= 0x24;
const BRR_OFFSET:u8= 0x28;

const DEFAULT_PIN: GPIOPin = GPIOPin {
    pin: Pin::PIN0,
    port: Port::GPIOA,
    taken: AtomicBool::new(false)
};

/*==============Makros========*/

macro_rules! create_gpio_structs {
    () => {{  // Note the extra braces
        let mut pins = [DEFAULT_PIN; 144];
        let mut i:usize = 0;
        while i < 144 {
            let port_num = i / 16;
            let pin_num = i % 16;
            pins[i] = GPIOPin {
                port: match port_num {
                    0 => Port::GPIOA,
                    1 => Port::GPIOB,
                    2 => Port::GPIOC,
                    3 => Port::GPIOD,
                    4 => Port::GPIOE,
                    5 => Port::PortF,
                    6 => Port::PortG,
                    7 => Port::PortH,
                    _ => Port::PortI,
                },
                pin: match pin_num {
                    0 => Pin::PIN0,
                    1 => Pin::PIN1,
                    2 => Pin::PIN2,
                    3 => Pin::PIN3,
                    4 => Pin::PIN4,
                    5 => Pin::PIN5,
                    6 => Pin::PIN6,
                    7 => Pin::PIN7,
                    8 => Pin::PIN8,
                    9 => Pin::PIN9,
                    10 => Pin::PIN10,
                    11 => Pin::PIN11,
                    12 => Pin::PIN12,
                    13 => Pin::PIN13,
                    14 => Pin::PIN14,
                    _ => Pin::PIN15,
                },
                taken: AtomicBool::new(false)
            };
        i+=1;
        }
        pins
    }};
}

/*==============STRUCTS========*/

/// Available GPIO ports (A-I) on the microcontroller.
#[derive(Copy, Clone)]
pub enum Port {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    PortF,
    PortG,
    PortH,
    PortI,
}

/// Available GPIO pins (0-15) per Port
#[derive(Copy, Clone)]
pub enum Pin {
    PIN0,
    PIN1,
    PIN2,
    PIN3,
    PIN4,
    PIN5,
    PIN6,
    PIN7,
    PIN8,
    PIN9,
    PIN10,
    PIN11,
    PIN12,
    PIN13,
    PIN14,
    PIN15,
}

/// Available GPIO Modes
pub enum Mode {
    Input =0b00,
    Output=0b01,
    Alternate=0b10,
    Analog=0b11,
}

/// A GPIO pin with ownership tracking.
pub struct GPIOPin {
    pin: Pin,
    port: Port,
    taken: AtomicBool
}

static GPIO:[GPIOPin;144] = create_gpio_structs!();



/*==============IMPL========*/

impl GPIOPin {
    //peripheral shouldn't be clonable to outside
    fn clone(&self) -> GPIOPin {
        let status = self.taken.load(Ordering::Acquire);
        GPIOPin {
            pin: self.pin,
            port: self.port,
            taken: AtomicBool::from(status),
        }
    }

    /// Takes ownership of a GPIO peripheral pin, ensuring exclusive access.
    ///
    /// # Arguments
    /// * `port` - Port number for the GPIO peripheral
    /// * `pin` - Pin number within the port
    ///
    /// # Returns
    /// * `GPIOPin` - The GPIO pin peripheral object
    ///
    /// # Panics
    /// * If the requested pin is already taken by another part of the program
    ///
    /// # Thread Safety
    /// * Uses atomic operations for thread-safe peripheral access control
    /// * Acquires exclusive ownership using compare_exchange with Acquire ordering
    ///
    /// # Example
    /// ```
    /// use crate::hal::gpio::{GPIOPin,Pin,Port};
    ///
    /// let led = GPIOPin::take(Port::GPIOA, Pin::PIN0);
    /// // Use LED pin...
    /// ```
    pub fn take(port: Port, pin: Pin) -> GPIOPin {
        let port: usize = port as usize;
        let pin: usize = pin as usize;
        let available = GPIO[port * 16 + pin].taken.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed);
        if available.is_err() { panic!("Tried to acquire taken peripheral") }
        GPIO[port * 16 + pin].clone()
    }

    /// Releases ownership of a GPIO pin back to the system.
    ///
    /// # Arguments
    /// * self - Uses the pin's internal state
    ///
    /// # Returns
    /// * None
    ///
    /// # Panics
    /// * If attempting to release a pin that is already free
    ///
    /// # Thread Safety
    /// * Uses atomic operations with Release ordering for thread-safe deallocation
    /// * Ensures proper synchronization with subsequent pin acquisitions
    ///
    /// # Example
    /// ```
    /// use crate::hal::gpio::{GPIOPin,Pin,Port};
    ///
    /// let led = GPIOPin::take(Port::GPIOA, Pin::PIN0);
    /// // Use LED pin...
    /// led.release(); // Return pin to system
    /// ```
    pub fn release(self) {
        let port: usize = self.port as usize;
        let pin: usize = self.pin as usize;
        let released = GPIO[port * 16 + pin].taken.compare_exchange(true, false, Ordering::Release, Ordering::Relaxed);
        if released.is_err() { panic!("tried to release free peripheral") }
    }

    /// Sets the mode of the GPIO pin
    ///
    /// # Arguments
    /// * `mode` - The desired mode (Input, Output, Alternate, or Analog)
    ///
    /// # Returns
    /// * `bool` - True if mode was set successfully, False otherwise
    ///
    /// # Safety
    /// * The register is modified atomically, concurrent mode changes are preserved
    /// * Preserves other pins' configurations through masked updates
    ///
    /// # Example
    /// ```
    /// use crate::hal::gpio::{GPIOPin,Port,Pin,Mode};
    ///
    /// let pin = GPIOPin::take(Port::GPIOA, Pin::PIN0);
    /// pin.set_mode(Mode::Output); // Configure as output
    /// ```
    pub fn set_mode(&self, mode: Mode) ->bool {
        let base_address = match self.port {
            Port::GPIOA => { GPIOA_BASE }
            Port::GPIOB => { GPIOB_BASE }
            Port::GPIOC => { GPIOC_BASE }
            Port::GPIOD => { GPIOD_BASE }
            Port::GPIOE => { GPIOE_BASE }
            Port::PortF => { GPIOF_BASE }
            Port::PortG => { GPIOG_BASE }
            Port::PortH => { GPIOH_BASE }
            Port::PortI => { GPIOI_BASE }
        };
        let target:u32 = match self.pin {
            Pin::PIN0 => { 0b0000_0000_0000_0000_0000_0000_0000_0011 },
            Pin::PIN1 => { 0b0000_0000_0000_0000_0000_0000_0000_1100 }
            Pin::PIN2 => { 0b0000_0000_0000_0000_0000_0000_0011_0000 }
            Pin::PIN3 => { 0b0000_0000_0000_0000_0000_0000_1100_0000 }
            Pin::PIN4 => { 0b0000_0000_0000_0000_0000_0011_0000_0000 }
            Pin::PIN5 => { 0b0000_0000_0000_0000_0000_1100_0000_0000 }
            Pin::PIN6 => { 0b0000_0000_0000_0000_0011_0000_0000_0000 }
            Pin::PIN7 => { 0b0000_0000_0000_0000_1100_0000_0000_0000 }
            Pin::PIN8 => { 0b0000_0000_0000_0011_0000_0000_0000_0000 }
            Pin::PIN9 => { 0b0000_0000_0000_1100_0000_0000_0000_0000 }
            Pin::PIN10 => { 0b0000_0000_0011_0000_0000_0000_0000_0000 }
            Pin::PIN11 => { 0b0000_0000_1100_0000_0000_0000_0000_0000 }
            Pin::PIN12 => { 0b0000_0011_0000_0000_0000_0000_0000_0000 }
            Pin::PIN13 => { 0b0000_1100_0000_0000_0000_0000_0000_0000 }
            Pin::PIN14 => { 0b0011_0000_0000_0000_0000_0000_0000_0000 }
            Pin::PIN15 => { 0b1100_0000_0000_0000_0000_0000_0000_0000 }
        };
        let mode_pattern:u32 = match mode {
            Mode::Input => {0b0000_0000_0000_0000_0000_0000_0000_0000}
            Mode::Output => {0b0101_0101_0101_0101_0101_0101_01001_0101}
            Mode::Alternate => {0b1010_1010_1010_1010_1010_1010_1010_1010}
            Mode::Analog => {0b1111_1111_1111_1111_1111_1111_1111_111}
        };
        //direct manipulation of the memory mapped register
        unsafe {
            let atomic_ref = unsafe { &mut *(base_address as *mut AtomicU32) };
            let res = atomic_ref.fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                let cleared = value & !target;
                 Some(cleared | (target & mode_pattern))
            });
            match res {
                Ok(_) => { true }
                Err(_) => { false }
            }
        }
    }
}


