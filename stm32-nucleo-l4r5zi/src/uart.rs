use core::marker::PhantomData;
use crate::gpio::{AlternateFunction, GPIOPin};

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
pub trait UartFeatures {
    const SUPPORTS_SYNC: bool;
    const SUPPORTS_SMARTCARD: bool;
    const SUPPORTS_IRDA: bool;
    const SUPPORTS_RS485: bool;
    const SUPPORTS_SINGLE_WIRE: bool;
    const MAX_BAUD: u32;
    const HAS_FIFO: bool;
}

// Define specific UART instances
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
trait HasFifo{}
trait MaxBaud{
    const BAUD: u32;
}

// Implement features for each UART instance
    impl SupportsSync for Uart1{}
    impl SupportsSmartCard for Uart1{}
    impl SupportsIrDA for Uart1{}
    impl SupportsRS485 for Uart1{}
    impl SupportsSingleWire for Uart1{}
    impl HasFifo for Uart1{}
    impl MaxBaud for Uart1{
        const BAUD: u32 = 5000000;
    }


// Basic configuration common to all modes
pub struct Config {
    pub baud: u32,
    pub word_length: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub oversampling: Oversampling,
    pub clock_source: ClockSource,
    pub hardware_flow_control: bool,
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

pub struct Uart<Instance: UartFeatures, S: State> {
    tx: GPIOPin<AlternateFunction>,
    rx: Option<GPIOPin<AlternateFunction>>,
    _instance: PhantomData<Instance>,
    _state: PhantomData<S>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UartError {
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


impl<Instance: UartFeatures> Uart<Instance, Disabled> {
    pub fn new<Mode1, Mode2>(tx: GPIOPin<Mode1>, rx: GPIOPin<Mode2>, config: Config) -> Result<Self, UartError> {
        assert!(config.baud <= Instance::MAX_BAUD);
        unimplemented!()
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

impl<Instance:UartFeatures,State> Uart<Instance,State> {
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

impl <Instance:UartFeatures> Uart<Instance,Synchronous> {
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





