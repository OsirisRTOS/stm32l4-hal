use core::cmp::PartialEq;
use core::marker::PhantomData;
use core::ops::Add;
use crate::gpio::{Alternate, AlternateFunction, GPIOPin, IntoAlternate, Pin, Port, Undefined};
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};


const RCC_BASE:u32 = 0x4002_1000;
const APB2ENR_OFFSET:u32 = 0x60;

// Combined state enum - sealed to prevent external implementations
mod sealed {
    pub trait Sealed {}
}

// UART states
pub trait State: sealed::Sealed {}

pub struct Disabled;
pub struct Asynchronous;
pub struct Synchronous;
pub struct SingleWire;
pub struct SmartCard;
pub struct IrDA;
pub struct RS485;

// Implement sealed and State for all valid states
impl sealed::Sealed for Disabled {}
impl sealed::Sealed for Asynchronous {}
impl sealed::Sealed for Synchronous {}
impl sealed::Sealed for SmartCard {}
impl sealed::Sealed for IrDA {}
impl sealed::Sealed for RS485 {}

impl State for Disabled {}
impl State for Asynchronous {}
impl State for Synchronous {}
impl State for SmartCard {}
impl State for IrDA {}
impl State for RS485 {}

// UART instance features
pub trait UartDevice {
    const MAX_BAUD:u32 = 115200;
    fn get_tx_fn(port:Port,pin:Pin) -> Result<AlternateFunction,UartError>;
    fn get_rx_fn(port:Port,pin:Pin) -> Result<AlternateFunction,UartError>;
    fn lock() -> Result<u8,u8>;
    fn unlock();
    fn enable_clock();
}

// Define specific UART instances
static TAKEN:AtomicU8 = AtomicU8::new(0);
pub struct Uart1;

pub struct Uart2;

pub struct Uart3;

pub struct Uart4;

pub struct Uart5;

pub struct Lpuart1;


trait SupportsSync{}
trait SupportsSmartCard{}
trait SupportsIrDA{}
trait SupportsRS485{}
trait SupportsSingleWire{}


    impl SupportsSync for Uart1{}
    impl SupportsSmartCard for Uart1{}
    impl SupportsIrDA for Uart1{}
    impl SupportsRS485 for Uart1{}
    impl SupportsSingleWire for Uart1{}



// Basic configuration common to all modes
pub struct Config {
    pub baud: u32,
    pub word_length: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub oversampling: Oversampling,
    pub clock_source: ClockSource,
    pub hardware_flow_control: bool,
    pub fifo_enable: bool,
    pub dma_config: Option<DmaConfig>,
}

// Mode-specific configurations
pub struct SyncConfig {
    pub clock_polarity: ClockPolarity,
    pub clock_phase: ClockPhase,
    pub last_bit_clock_pulse: bool,
}

pub struct SmartCardConfig {
    pub nack_enable: bool,
    pub guard_time: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataBits {
    Seven,
    Eight,
    Nine,
}

// Parity configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parity {
    None,
    Even,
    Odd,
}

// Stop bits configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopBits {
    Half,
    One,
    OneAndHalf,
    Two,
}



// Clock source selection
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSource {
    PCLK,      // APB clock
    SYSCLK,    // System clock
    HSI16,     // 16 MHz internal oscillator
    LSE,       // Low-speed external oscillator
}

// Oversampling configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Oversampling {
    By8,
    By16,
}

// DMA configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DmaThreshold {
    Eighth,
    Quarter,
    Half,
    ThreeQuarters,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DmaBurst {
    Single,
    Burst4,
    Burst8,
}

#[derive(Clone, Copy, Debug)]
pub struct DmaConfig {
    pub tx_threshold: DmaThreshold,
    pub rx_threshold: DmaThreshold,
    pub tx_burst: DmaBurst,
    pub rx_burst: DmaBurst,
}

// FIFO configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FifoThreshold {
    Eighth,
    Quarter,
    Half,
    ThreeQuarters,
    Full,
}

// Clock polarity for synchronous mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockPolarity {
    IdleLow,
    IdleHigh,
}

// Clock phase for synchronous mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockPhase {
    FirstEdge,
    SecondEdge,
}

pub struct Uart<Instance: UartDevice, S: State> {
    tx: GPIOPin<AlternateFunction>,
    rx: Option<GPIOPin<AlternateFunction>>,
    _instance: PhantomData<Instance>,
    _state: PhantomData<S>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UartError {
    DeviceUnavailable,
    ParityError,
    FramingError,
    NoiseError,
    OverrunError,
    BreakDetected,
    BaudNotSupported,
    InvalidConfiguration,
    Timeout,
    BufferFull,
    BufferEmpty,
    DmaError,
    PinConfigError,
    InvalidMode,
    UnsupportedMode,
    ClockError,
    FeatureNotSupported,
    BusError,
}


impl<Instance: UartDevice> Uart<Instance, Disabled> {
    pub fn new(tx_port:Port, tx_pin:Pin, rx_port:Option<Port>,rx_pin:Option<Pin>, config: Config) -> Result<Self, UartError> {
        if config.baud > Instance::MAX_BAUD {
            return Err(UartError::BaudNotSupported);
        }
        if rx_port.is_some()!=rx_pin.is_some() {
            return Err(UartError::PinConfigError);
        }
        let tx = match GPIOPin::take(tx_port, tx_pin) {
            Ok(tx) => tx,
            Err(_) => return Err(UartError::PinConfigError),
        };
        let mut tx = match tx.into_alternate() {
            Ok(pin) => {pin}
            Err(pin) => {
                pin.release();
                return Err(UartError::PinConfigError);
            }
        };
        let rx = match rx_port.is_some() {
            true => {
                let rx = match GPIOPin::take(tx_port, tx_pin) {
                    Ok(rx) => rx,
                    Err(_) => return Err(UartError::PinConfigError),
                };
                Some(match rx.into_alternate() {
                    Ok(pin) => {pin}
                    Err(pin) => {
                        pin.release();
                        tx.release();
                        return Err(UartError::PinConfigError);
                    }
                })
            }
            false => {None}
        };
        let tx_fn = Instance::get_tx_fn(tx_port, tx_pin)?;
        let rx_fn = match rx.is_some() {
            true => {Some(Instance::get_rx_fn(rx_port.unwrap(), rx_pin)?)}
            false => {None}
        };
        if Instance::lock().is_err() {
            tx.release();
            if (rx.is_some() { rx.unwrap().release();}
            return Err(UartError::DeviceUnavailable);
        }
        tx.set_alternate_function(tx_fn);
        if let Some(mut rx) = rx {
            rx.set_alternate_function(rx_fn.unwrap());
        }
        Instance::enable_clock();
        unimplemented!();
    }

    pub fn enable_async(self) -> Result<Uart<Instance, Asynchronous>, UartError> {
        unimplemented!()
    }

    pub fn enable_sync<Instance: SupportsSync>(
        self,
        sync_config: SyncConfig
    ) -> Result<Uart<Instance, Synchronous>, UartError>
    {
        unimplemented!()
    }
}


impl UartDevice for Uart1 {
    fn get_tx_fn(port:Port,pin:Pin) -> Result<AlternateFunction,UartError> {
        if port==Port::GPIOA&&pin==Pin::PIN9 {
            Ok(AlternateFunction::AF7)
        } else if port==Port::GPIOB&&pin==Pin::PIN6 {
            Ok(AlternateFunction::AF7)
        } else {
            Err(UartError::PinConfigError)
        }
    }

    fn get_rx_fn(port: Port, pin: Pin) -> Result<AlternateFunction, UartError> {
        if port==Port::GPIOA&&pin==Pin::PIN10 {
            Ok(AlternateFunction::AF7)
        } else if port==Port::GPIOB&&pin==Pin::PIN7 {
            Ok(AlternateFunction::AF7)
        } else {
            Err(UartError::PinConfigError)
        }
    }

    fn lock() -> Result<u8,u8> {
        let mask = 0x01;
        TAKEN.fetch_update(
            Ordering::SeqCst,
            Ordering::Relaxed,
            |x| {
                if x&mask {
                    None
                } else {
                    Some(x|mask)
                }
            }
        )
    }

    fn unlock() {
        let mask = 0xFE;
        let res = TAKEN.fetch_update(
            Ordering::SeqCst,
            Ordering::Relaxed,
            |x| {
                Some(x&mask)
            }
        );
    }

    fn enable_clock() {
        let mask = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        let ptr:*mut u32 = RCC_BASE.add(APB2ENR_OFFSET) as *mut u32;
        ///SAFETY:
        /// The Base Address is on of 9 possible base addresses that are memory mapped registers and therefor guaranteed to be valid
        let atomic: &AtomicU32 = unsafe { AtomicU32::from_ptr(ptr) };
        ///SAFETY
        /// This Result is only error if the closure returns None, which can't happen
        atomic.fetch_or(mask, Ordering::SeqCst);
    }
}

impl<Instance: UartDevice,State> Uart<Instance,State> {
    pub fn transmit(&mut self, data: &[u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    // Non-blocking variants
    pub fn transmit_nonblocking(&mut self, data: &[u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn receive_nonblocking(&mut self, buffer: &mut [u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    // Status checks
    pub fn is_tx_complete(&self) -> bool {
        unimplemented!()
    }

    pub fn is_rx_complete(&self) -> bool {
        unimplemented!()
    }

    // FIFO control (if supported)
    pub fn set_tx_fifo_threshold(&mut self, threshold: FifoThreshold) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn set_rx_fifo_threshold(&mut self, threshold: FifoThreshold) -> Result<(), UartError> {
        unimplemented!()
    }

    // Break control
    pub fn send_break(&mut self) -> Result<(), UartError> {
        unimplemented!()
    }
}

impl <Instance: UartDevice> Uart<Instance,Synchronous> {
    pub fn transmit_sync(&mut self, data: &[u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn receive_sync(&mut self, buffer: &mut [u8]) -> Result<(), UartError> {
        unimplemented!()
    }

    // Clock configuration
    pub fn set_clock_polarity(&mut self, polarity: ClockPolarity) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn set_clock_phase(&mut self, phase: ClockPhase) -> Result<(), UartError> {
        unimplemented!()
    }

    pub fn set_last_bit_clock_pulse(&mut self, enabled: bool) -> Result<(), UartError> {
        unimplemented!()
    }
}





