use core::arch::asm;
///preliminary version, untested code

use core::sync::atomic::{fence, AtomicU32, AtomicU8, Ordering};
use core::marker::{Copy, PhantomData};
use core::ops::{Add};

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
const OTYPER_OFFSET:u32= 0x04;
const OSPEEDR_OFFSET:u32= 0x08;
const PUPDR_OFFSET:u32= 0x0C;
const IDR_OFFSET:u32= 0x10;
const ODR_OFFSET:u32= 0x14;
const BSRR_OFFSET:u32= 0x18;
const LCKR_OFFSET:u32= 0x1C;
const AFRL_OFFSET:u32= 0x20;
const AFRH_OFFSET:u32= 0x24;
const BRR_OFFSET:u32= 0x28;





/*==============STATIC========*/

static TAKEN:[AtomicU8;18] = {
    const INIT: AtomicU8 = AtomicU8::new(0);
    [INIT; 18]
};

/*==============STRUCTS========*/

/// Available GPIO ports (A-I) on the microcontroller.
#[derive(Copy, Clone)]
pub enum Port {
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2,
    GPIOD = 3,
    GPIOE = 4,
    GPIOF = 5,
    GPIOG = 6,
    GPIOH = 7,
    GPIOI = 8,
}

/// Available GPIO pins (0-15) per Port
#[derive(Copy, Clone)]
pub enum Pin {
    PIN0 = 0,
    PIN1 = 1,
    PIN2 = 2,
    PIN3 = 3,
    PIN4 = 4,
    PIN5 = 5,
    PIN6 = 6,
    PIN7 = 7,
    PIN8 = 8,
    PIN9 = 9,
    PIN10 = 10,
    PIN11 = 11,
    PIN12 = 12,
    PIN13 = 13,
    PIN14 = 14,
    PIN15 = 15,
}

#[derive(Copy, Clone)]
pub enum OutputType {
    PushPull = 0,
    OpenDrain = 1,
}

#[derive(Copy, Clone)]
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[derive(Copy, Clone)]
pub enum PushPullMode {
    Disabled = 0,
    PullUp = 1,
    PullDown = 2,
}

#[derive(Copy, Clone)]
pub enum AlternateFunction {
    AF0 = 0,
    AF1 = 1,
    AF2 = 2,
    AF3 = 3,
    AF4 = 4,
    AF5 = 5,
    AF6 = 6,
    AF7 = 7,
    AF8 = 8,
    AF9 = 9,
    AF10 = 10,
    AF11 = 11,
    AF12 = 12,
    AF13 = 13,
    AF14 = 14,
    AF15 = 15,
}

/// Available GPIO Modes
pub struct Input;
pub struct Output;
pub struct Alternate;
pub struct Analog;
pub struct Undefined;

/// A GPIO pin with ownership tracking.
pub struct GPIOPin<Mode> {
    pin: Pin,
    port: Port,
    _mode: PhantomData<Mode>,
}

/*==============Traits========*/

trait InputConvertible {}
trait OutputConvertible {}
trait AlternateConvertible {}
trait AnalogConvertible {}
trait Configured {}

pub trait IntoInput<Success,Failure> {
    fn into_input(self) -> Result<Success,Failure>;
}

pub trait IntoOutput<Success,Failure> {
    fn into_output(self) -> Result<Success,Failure>;
}

pub trait IntoAlternate<Success,Failure> {
    fn into_alternate(self) -> Result<Success,Failure>;
}

pub trait IntoAnalog<Success,Failure> {
    fn into_analog(self) -> Result<Success,Failure>;
}


/*==============IMPL========*/

//marker traits
impl InputConvertible for Output {}
impl InputConvertible for Analog {}
impl InputConvertible for Alternate {}
impl InputConvertible for Undefined {}
impl OutputConvertible for Input {}
impl OutputConvertible for Alternate {}
impl OutputConvertible for Analog {}
impl OutputConvertible for Undefined {}
impl AnalogConvertible for Input {}
impl AnalogConvertible for Output {}
impl AnalogConvertible for Alternate {}
impl AnalogConvertible for Undefined {}
impl AlternateConvertible for Input {}
impl AlternateConvertible for Output {}
impl AlternateConvertible for Analog {}
impl AlternateConvertible for Undefined {}
impl Configured for Input {}
impl Configured for Output {}
impl Configured for Alternate {}
impl Configured for Analog {}



impl <Mode> GPIOPin<Mode> {
    fn get_baseaddress(&self) -> u32 {
        match self.port {
            Port::GPIOA => { GPIOA_BASE }
            Port::GPIOB => { GPIOB_BASE }
            Port::GPIOC => { GPIOA_BASE }
            Port::GPIOD => { GPIOA_BASE }
            Port::GPIOE => { GPIOA_BASE }
            Port::GPIOF => { GPIOA_BASE }
            Port::GPIOG => { GPIOA_BASE }
            Port::GPIOH => { GPIOA_BASE }
            Port::GPIOI => { GPIOA_BASE }
        }
    }

    pub fn release(self) {
        let idx = 2 * self.port as usize;
        let idx = if (self.pin as usize) < 8 {
            idx
        } else {
            idx + 1
        };
        let bit = if (self.pin as usize) < 8 {
            self.pin as usize
        } else {
            self.pin as usize - 8
        };
        let mask = !(1 << bit);
        TAKEN[idx].fetch_and(mask, Ordering::AcqRel);
    }
}

impl <Mode: Configured> GPIOPin<Mode> {
    pub fn set_outputtype(&mut self,r#type:OutputType) {
        let target = 1<<self.pin as usize;
        let baseaddress = self.get_baseaddress().add(OTYPER_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        match r#type {
            OutputType::PushPull => {
                atomic.fetch_and(!target, Ordering::AcqRel);
            }
            OutputType::OpenDrain => {
                atomic.fetch_or(target, Ordering::AcqRel);
            }
        }
    }

    pub fn set_speed(&mut self,speed:Speed) {
        let target = 11<<2*self.pin as usize;
        let speedpattern = match speed {
            Speed::Low => {0b0000_0000_0000_0000_0000_0000_0000_0000}
            Speed::Medium => {0b0101_0101_0101_0101_0101_0101_0101_0101}
            Speed::High => {0b1010_1010_1010_1010_1010_1010_1010_1010_}
            Speed::VeryHigh => {0b1111_1111_1111_1111_1111_1111_1111_1111_}
        };
        let baseaddress = self.get_baseaddress().add(OSPEEDR_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        ///SAFETY
        /// This Result is only error if the closure returns None, which can't happen
        atomic.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |value| {
            let cleared = value & !target;
            Some(cleared | (target&speedpattern))
        }).expect("UNREACHABLE");
    }

    pub fn set_push_pull(&mut self,pupd:PushPullMode) {
        let target = 11<<2*self.pin as usize;
        let speedpattern = match pupd {
            PushPullMode::Disabled => {0b0000_0000_0000_0000_0000_0000_0000_0000}
            PushPullMode::PullUp => {0b0101_0101_0101_0101_0101_0101_0101_0101}
            PushPullMode::PullDown => {0b1010_1010_1010_1010_1010_1010_1010_1010_}
        };
        let baseaddress = self.get_baseaddress().add(PUPDR_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        ///SAFETY
        /// This Result is only error if the closure returns None, which can't happen
        atomic.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |value| {
            let cleared = value & !target;
            Some(cleared | (target&speedpattern))
        }).expect("UNREACHABLE");
    }

    pub fn set_alternate_function(&mut self,af:AlternateFunction) {
        let target = if (self.pin as usize) < 8 {
            1111 << 4 * self.pin as usize
        } else {
            1111 << 4 * (self.pin as usize - 8)
        };
        let speedpattern = match af {
            AlternateFunction::AF0 => { 0b0000_0000_0000_0000_0000_0000_0000_0000 }
            AlternateFunction::AF1 => { 0b0001_0001_0001_0001_0001_0001_0001_0001 }
            AlternateFunction::AF2 => { 0b0010_0010_0010_0010_0010_0010_0010_0010 }
            AlternateFunction::AF3 => { 0b0011_0011_0011_0011_0011_0011_0011_0011 }
            AlternateFunction::AF4 => { 0b0100_0100_0100_0100_0100_0100_0100_0100 }
            AlternateFunction::AF5 => { 0b0101_0101_0101_0101_0101_0101_0101_0101 }
            AlternateFunction::AF6 => { 0b0110_0110_0110_0110_0110_0110_0110_0110 }
            AlternateFunction::AF7 => { 0b0111_0111_0111_0111_0111_0111_0111_0111 }
            AlternateFunction::AF8 => { 0b1000_1000_1000_1000_1000_1000_1000_1000 }
            AlternateFunction::AF9 => { 0b1001_1001_1001_1001_1001_1001_1001_1001 }
            AlternateFunction::AF10 => { 0b1010_1010_1010_1010_1010_1010_1010_1010 }
            AlternateFunction::AF11 => { 0b1011_1011_1011_1011_1011_1011_1011_1011 }
            AlternateFunction::AF12 => { 0b0000_0000_0000_0000_0000_0000_0000_0000 }
            AlternateFunction::AF13 => { 0b0000_0000_0000_0000_0000_0000_0000_0000 }
            AlternateFunction::AF14 => { 0b0000_0000_0000_0000_0000_0000_0000_0000 }
            AlternateFunction::AF15 => { 0b0000_0000_0000_0000_0000_0000_0000_0000 }
        };
        let baseaddress: u32 = if (self.pin as usize) < 8 {
            self.get_baseaddress().add(AFRL_OFFSET)
        } else {
            self.get_baseaddress().add(AFRL_OFFSET)
        };
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        ///SAFETY
        /// This Result is only error if the closure returns None, which can't happen
        atomic.fetch_update(Ordering::AcqRel, Ordering::Relaxed, |value| {
            let cleared = value & !target;
            Some(cleared | (target & speedpattern))
        }).expect("UNREACHABLE");
    }

    pub fn set_pin(&mut self) {
        let baseaddress = self.get_baseaddress().add(BSRR_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        let target = 1 << self.pin as usize;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        unsafe { ptr.write(target); }
    }

    pub fn reset_pin(&mut self) {
        let baseaddress = self.get_baseaddress().add(BSRR_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        let target = 1 << (self.pin as usize +16);
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        unsafe { ptr.write_volatile(target); }
    }

    pub fn read_state(&self) -> bool {
        let baseaddress = self.get_baseaddress().add(IDR_OFFSET);
        let ptr: *mut u32 = baseaddress as *mut u32;
        let target = 1 << (self.pin as usize +16);
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let idr:u32 = unsafe { ptr.read_volatile() };
        if (idr&target)==0 {
            false
        } else {
            true
        }
    }
}

impl GPIOPin<Undefined> {
    pub fn take(port:Port,pin:Pin) -> GPIOPin<Undefined> {
        let idx = 2*port as usize;
        let idx = if (pin as usize) < 8 {
            idx
        } else {
            idx+1
        };
        let bit = if (pin as usize) < 8 {
            pin as usize
        } else {
            pin as usize-8
        };
        let mask = 1<<bit;
        let res = TAKEN[idx].fetch_or(mask, Ordering::AcqRel);
        GPIOPin::<Undefined> { pin, port, _mode: PhantomData }
    }
}


impl <Mode: InputConvertible> IntoInput<GPIOPin<Input>, GPIOPin<Mode>> for GPIOPin<Mode> {
    fn into_input(self) -> Result<GPIOPin<Input>,GPIOPin<Mode>> {
        let baseaddress = self.get_baseaddress();
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        let target = 0b11<<2*self.pin as u32;
        let res = atomic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
            Some(value&(!target))
        });
        match res {
            Ok(_) => {Ok(GPIOPin::<Input> {
                pin: self.pin,
                port: self.port,
                _mode: PhantomData,
            })}
            Err(_) => {Err(self)}
        }
    }
}

impl <Mode: OutputConvertible> IntoOutput<GPIOPin<Output>,GPIOPin<Mode>> for GPIOPin<Mode> {
    fn into_output(self) -> Result<GPIOPin<Output>,GPIOPin<Mode>> {
        let baseaddress = self.get_baseaddress();
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        let target = 0b11<<(2*self.pin as u32);
        let modepattern = 0b0101_0101_0101_0101_0101_0101_0101_0101;
        let res = atomic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
            let cleared = value&(!target);
            let new = cleared |(modepattern&target);
            Some(new)
        });
        unsafe { asm!("dsb"); }
        let value = atomic.load(Ordering::Acquire);
        match res {
            Ok(_) => {Ok(GPIOPin::<Output> {
                pin: self.pin,
                port: self.port,
                _mode: PhantomData,
            })}
            Err(_) => {Err(self)}
        }
    }
}

impl <Mode: AnalogConvertible> IntoAnalog<GPIOPin<Analog>,GPIOPin<Mode>> for GPIOPin<Mode> {
    fn into_analog(self) -> Result<GPIOPin<Analog>, GPIOPin<Mode>> {
        let baseaddress = self.get_baseaddress();
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        let target = 0b11<<2*self.pin as u32;
        let modepattern = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let res = atomic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
            let cleared = value&(!target);
            Some(cleared |(modepattern&target))
        });

        match res {
            Ok(_) => {Ok(GPIOPin::<Analog> {
                pin: self.pin,
                port: self.port,
                _mode: PhantomData,
            })}
            Err(_) => {Err(self)}
        }
    }
}

impl <Mode: AlternateConvertible> IntoAlternate<GPIOPin<Alternate>,GPIOPin<Mode>> for GPIOPin<Mode> {
    fn into_alternate(self) -> Result<GPIOPin<Alternate>, GPIOPin<Mode>> {
        let baseaddress = self.get_baseaddress();
        let ptr: *mut u32 = baseaddress as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        let target = 0b11<<2*self.pin as u32;
        let modepattern = 0b1010_1010_1010_1010_1010_1010_1010_1010;
        let res = atomic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
            let cleared = value&(!target);
            Some(cleared |(modepattern&target))
        });
        match res {
            Ok(_) => {Ok(GPIOPin::<Alternate> {
                pin: self.pin,
                port: self.port,
                _mode: PhantomData,
            })}
            Err(_) => {Err(self)}
        }
    }
}


