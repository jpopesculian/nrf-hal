//! A high level interface for RTC peripherals

use core::ops::Deref;

use crate::target::{rtc0, Interrupt, NVIC, RTC0, RTC1};

#[cfg(not(feature = "52810"))]
use crate::target::RTC2;

// Zero Size Type State structs

/// The RTC has been stopped
pub struct Stopped;
/// The RTC has been started
pub struct Started;

/// An opaque high level interface to an RTC peripheral
pub struct Rtc<T, M> {
    periph: T,
    _mode: M,
}

/// An extension trait for constructing the high level interface
pub trait RtcExt : Deref<Target=rtc0::RegisterBlock> + Sized {
    fn constrain(self) -> Rtc<Self, Stopped>;
}

macro_rules! impl_rtc_ext {
    ($($rtc:ty,)*) => {
        $(
            impl RtcExt for $rtc {
                fn constrain(self) -> Rtc<$rtc, Stopped> {
                    Rtc {
                        periph: self,
                        _mode: Stopped,
                    }
                }
            }
        )*
    }
}

impl_rtc_ext!(RTC0, RTC1,);

#[cfg(not(feature = "52810"))]
impl_rtc_ext!(RTC2,);

/// Interrupts/Events that can be generated by the RTCn peripheral
pub enum RtcInterrupt {
    Tick,
    Overflow,
    Compare0,
    Compare1,
    Compare2,
    Compare3,
}

/// Compare registers available on the RTCn
pub enum RtcCompareReg {
    Compare0,
    Compare1,
    Compare2,
    Compare3,
}

impl<T, M> Rtc<T, M>
where
    T: RtcExt,
{
    /// Enable/start the Real Time Counter
    pub fn enable_counter(self) -> Rtc<T, Started> {
        unsafe {
            self.periph.tasks_start.write(|w| w.bits(1));
        }
        Rtc {
            periph: self.periph,
            _mode: Started,
        }
    }

    /// Disable/stop the Real Time Counter
    pub fn disable_counter(self) -> Rtc<T, Stopped> {
        unsafe {
            self.periph.tasks_stop.write(|w| w.bits(1));
        }
        Rtc {
            periph: self.periph,
            _mode: Stopped,
        }
    }

    /// Enable the generation of a hardware interrupt from a given stimulus
    pub fn enable_interrupt(&mut self, int: RtcInterrupt) {
        match int {
            RtcInterrupt::Tick => self.periph.intenset.write(|w| w.tick().set()),
            RtcInterrupt::Overflow => self.periph.intenset.write(|w| w.ovrflw().set()),
            RtcInterrupt::Compare0 => self.periph.intenset.write(|w| w.compare0().set()),
            RtcInterrupt::Compare1 => self.periph.intenset.write(|w| w.compare1().set()),
            RtcInterrupt::Compare2 => self.periph.intenset.write(|w| w.compare2().set()),
            RtcInterrupt::Compare3 => self.periph.intenset.write(|w| w.compare3().set()),
        }
    }

    /// Disable the generation of a hardware interrupt from a given stimulus
    pub fn disable_interrupt(&mut self, int: RtcInterrupt) {
        match int {
            RtcInterrupt::Tick => self.periph.intenclr.write(|w| w.tick().clear()),
            RtcInterrupt::Overflow => self.periph.intenclr.write(|w| w.ovrflw().clear()),
            RtcInterrupt::Compare0 => self.periph.intenclr.write(|w| w.compare0().clear()),
            RtcInterrupt::Compare1 => self.periph.intenclr.write(|w| w.compare1().clear()),
            RtcInterrupt::Compare2 => self.periph.intenclr.write(|w| w.compare2().clear()),
            RtcInterrupt::Compare3 => self.periph.intenclr.write(|w| w.compare3().clear()),
        }
    }

    /// Enable the generation of a hardware event from a given stimulus
    pub fn enable_event(&mut self, evt: RtcInterrupt) {
        match evt {
            RtcInterrupt::Tick => self.periph.evtenset.write(|w| w.tick().set()),
            RtcInterrupt::Overflow => self.periph.evtenset.write(|w| w.ovrflw().set()),
            RtcInterrupt::Compare0 => self.periph.evtenset.write(|w| w.compare0().set()),
            RtcInterrupt::Compare1 => self.periph.evtenset.write(|w| w.compare1().set()),
            RtcInterrupt::Compare2 => self.periph.evtenset.write(|w| w.compare2().set()),
            RtcInterrupt::Compare3 => self.periph.evtenset.write(|w| w.compare3().set()),
        }
    }

    /// Disables the generation of a hardware event from a given stimulus
    pub fn disable_event(&mut self, evt: RtcInterrupt) {
        match evt {
            RtcInterrupt::Tick => self.periph.evtenclr.write(|w| w.tick().clear()),
            RtcInterrupt::Overflow => self.periph.evtenclr.write(|w| w.ovrflw().clear()),
            RtcInterrupt::Compare0 => self.periph.evtenclr.write(|w| w.compare0().clear()),
            RtcInterrupt::Compare1 => self.periph.evtenclr.write(|w| w.compare1().clear()),
            RtcInterrupt::Compare2 => self.periph.evtenclr.write(|w| w.compare2().clear()),
            RtcInterrupt::Compare3 => self.periph.evtenclr.write(|w| w.compare3().clear()),
        }
    }

    /// Obtain the state of a given interrupt/event, and optionally clear the event
    /// if it is set
    pub fn get_event_triggered(&mut self, evt: RtcInterrupt, clear_on_read: bool) -> bool {
        let mut orig = 0;
        let set_val = if clear_on_read { 0 } else { 1 };
        match evt {
            RtcInterrupt::Tick => self.periph.events_tick.modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
            RtcInterrupt::Overflow => self.periph.events_ovrflw.modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
            RtcInterrupt::Compare0 => self.periph.events_compare[0].modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
            RtcInterrupt::Compare1 => self.periph.events_compare[1].modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
            RtcInterrupt::Compare2 => self.periph.events_compare[2].modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
            RtcInterrupt::Compare3 => self.periph.events_compare[3].modify(|r, w| {
                orig = r.bits();
                unsafe { w.bits(set_val) }
            }),
        };

        orig == 1
    }

    /// Set the compare value of a given register. The compare registers have a width
    /// of 24 bits
    pub fn set_compare(&mut self, reg: RtcCompareReg, val: u32) -> Result<(), Error> {
        if val >= (1 << 24) {
            return Err(Error::CompareOutOfRange);
        }

        let reg = match reg {
            RtcCompareReg::Compare0 => 0,
            RtcCompareReg::Compare1 => 1,
            RtcCompareReg::Compare2 => 2,
            RtcCompareReg::Compare3 => 3,
        };

        unsafe {
            self.periph.cc[reg].write(|w| w.bits(val));
        }

        Ok(())
    }

    /// Obtain the current value of the Real Time Counter, 24 bits of range
    pub fn get_counter(&self) -> u32 {
        self.periph.counter.read().bits()
    }

    /// Destructure the high level interface. Does not reset any configuration made
    /// to the given RTC peripheral
    pub fn release(self) -> T {
        self.periph
    }
}

/// Error types associated with the RTC peripheral interface
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    PrescalerOutOfRange,
    CompareOutOfRange,
}

impl<T> Rtc<T, Stopped>
where
    T: RtcExt,
{
    /// Set the prescaler for the RTC peripheral. 12 bits of range.
    /// fRTC = 32_768 / (`prescaler` + 1 )
    pub fn set_prescaler(&mut self, prescaler: u32) -> Result<(), Error> {
        if prescaler >= (1 << 12) {
            return Err(Error::PrescalerOutOfRange);
        }

        unsafe { self.periph.prescaler.write(|w| w.bits(prescaler)) };

        Ok(())
    }
}
