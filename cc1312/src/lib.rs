#![doc = "Peripheral access API for CC13X2_CC26X2 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn NMI_FAULT();
    fn HARD_FAULT();
    fn MEMMANAGE_FAULT();
    fn BUS_FAULT();
    fn USAGE_FAULT();
    fn SVCALL();
    fn DEBUG();
    fn PENDSV();
    fn SYSTICK();
    fn AON_GPIO_INT();
    fn I2C_INT();
    fn RFC_CPE_1();
    fn PKA_IRQ();
    fn AON_RTC_COMB();
    fn UART0_COMB();
    fn AUX_SWEV0();
    fn SSI0_COMB();
    fn SSI1_COMB();
    fn RFC_CPE_0();
    fn RFC_HW_COMB();
    fn RFC_CMD_ACK();
    fn I2S_IRQ();
    fn AUX_SWEV1();
    fn WDT_IRQ();
    fn GPT0A();
    fn GPT0B();
    fn GPT1A();
    fn GPT1B();
    fn GPT2A();
    fn GPT2B();
    fn GPT3A();
    fn GPT3B();
    fn CRYPTO_RESULT_AVAIL_IRQ();
    fn DMA_DONE_COMB();
    fn DMA_ERR();
    fn FLASH();
    fn SWEV0();
    fn AUX_COMB();
    fn AON_PROG0();
    fn PROG0();
    fn AUX_COMPA();
    fn AUX_ADC_IRQ();
    fn TRNG_IRQ();
    fn OSC_COMB();
    fn AUX_TIMER2_EV0();
    fn UART1_COMB();
    fn BATMON_COMB();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 54] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: NMI_FAULT,
    },
    Vector {
        _handler: HARD_FAULT,
    },
    Vector {
        _handler: MEMMANAGE_FAULT,
    },
    Vector {
        _handler: BUS_FAULT,
    },
    Vector {
        _handler: USAGE_FAULT,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SVCALL },
    Vector { _handler: DEBUG },
    Vector { _reserved: 0 },
    Vector { _handler: PENDSV },
    Vector { _handler: SYSTICK },
    Vector {
        _handler: AON_GPIO_INT,
    },
    Vector { _handler: I2C_INT },
    Vector {
        _handler: RFC_CPE_1,
    },
    Vector { _handler: PKA_IRQ },
    Vector {
        _handler: AON_RTC_COMB,
    },
    Vector {
        _handler: UART0_COMB,
    },
    Vector {
        _handler: AUX_SWEV0,
    },
    Vector {
        _handler: SSI0_COMB,
    },
    Vector {
        _handler: SSI1_COMB,
    },
    Vector {
        _handler: RFC_CPE_0,
    },
    Vector {
        _handler: RFC_HW_COMB,
    },
    Vector {
        _handler: RFC_CMD_ACK,
    },
    Vector { _handler: I2S_IRQ },
    Vector {
        _handler: AUX_SWEV1,
    },
    Vector { _handler: WDT_IRQ },
    Vector { _handler: GPT0A },
    Vector { _handler: GPT0B },
    Vector { _handler: GPT1A },
    Vector { _handler: GPT1B },
    Vector { _handler: GPT2A },
    Vector { _handler: GPT2B },
    Vector { _handler: GPT3A },
    Vector { _handler: GPT3B },
    Vector {
        _handler: CRYPTO_RESULT_AVAIL_IRQ,
    },
    Vector {
        _handler: DMA_DONE_COMB,
    },
    Vector { _handler: DMA_ERR },
    Vector { _handler: FLASH },
    Vector { _handler: SWEV0 },
    Vector { _handler: AUX_COMB },
    Vector {
        _handler: AON_PROG0,
    },
    Vector { _handler: PROG0 },
    Vector {
        _handler: AUX_COMPA,
    },
    Vector {
        _handler: AUX_ADC_IRQ,
    },
    Vector { _handler: TRNG_IRQ },
    Vector { _handler: OSC_COMB },
    Vector {
        _handler: AUX_TIMER2_EV0,
    },
    Vector {
        _handler: UART1_COMB,
    },
    Vector {
        _handler: BATMON_COMB,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "2 - The NMI handler"]
    NMI_FAULT = 2,
    #[doc = "3 - The hard fault handler"]
    HARD_FAULT = 3,
    #[doc = "4 - Memory Management (MemManage) Fault"]
    MEMMANAGE_FAULT = 4,
    #[doc = "5 - The bus fault handler"]
    BUS_FAULT = 5,
    #[doc = "6 - The usage fault handler"]
    USAGE_FAULT = 6,
    #[doc = "11 - Supervisor Call (SVCall)"]
    SVCALL = 11,
    #[doc = "12 - Debug monitor handler"]
    DEBUG = 12,
    #[doc = "14 - The PendSV handler"]
    PENDSV = 14,
    #[doc = "15 - The SysTick handler"]
    SYSTICK = 15,
    #[doc = "16 - AON_GPIO_INT"]
    AON_GPIO_INT = 16,
    #[doc = "17 - I2C_INT"]
    I2C_INT = 17,
    #[doc = "18 - RFC_CPE_1"]
    RFC_CPE_1 = 18,
    #[doc = "19 - PKA_IRQ"]
    PKA_IRQ = 19,
    #[doc = "20 - AON_RTC_COMB"]
    AON_RTC_COMB = 20,
    #[doc = "21 - UART0_COMB"]
    UART0_COMB = 21,
    #[doc = "22 - AUX_SWEV0"]
    AUX_SWEV0 = 22,
    #[doc = "23 - SSI0_COMB"]
    SSI0_COMB = 23,
    #[doc = "24 - SSI1_COMB"]
    SSI1_COMB = 24,
    #[doc = "25 - RFC_CPE_0"]
    RFC_CPE_0 = 25,
    #[doc = "26 - RFC_HW_COMB"]
    RFC_HW_COMB = 26,
    #[doc = "27 - RFC_CMD_ACK"]
    RFC_CMD_ACK = 27,
    #[doc = "28 - I2S_IRQ"]
    I2S_IRQ = 28,
    #[doc = "29 - AUX_SWEV1"]
    AUX_SWEV1 = 29,
    #[doc = "30 - WDT_IRQ"]
    WDT_IRQ = 30,
    #[doc = "31 - GPT0A"]
    GPT0A = 31,
    #[doc = "32 - GPT0B"]
    GPT0B = 32,
    #[doc = "33 - GPT1A"]
    GPT1A = 33,
    #[doc = "34 - GPT1B"]
    GPT1B = 34,
    #[doc = "35 - GPT2A"]
    GPT2A = 35,
    #[doc = "36 - GPT2B"]
    GPT2B = 36,
    #[doc = "37 - GPT3A"]
    GPT3A = 37,
    #[doc = "38 - GPT3B"]
    GPT3B = 38,
    #[doc = "39 - CRYPTO_RESULT_AVAIL_IRQ"]
    CRYPTO_RESULT_AVAIL_IRQ = 39,
    #[doc = "40 - DMA_DONE_COMB"]
    DMA_DONE_COMB = 40,
    #[doc = "41 - DMA_ERR"]
    DMA_ERR = 41,
    #[doc = "42 - FLASH"]
    FLASH = 42,
    #[doc = "43 - SWEV0"]
    SWEV0 = 43,
    #[doc = "44 - AUX_COMB"]
    AUX_COMB = 44,
    #[doc = "45 - AON_PROG0"]
    AON_PROG0 = 45,
    #[doc = "46 - PROG0"]
    PROG0 = 46,
    #[doc = "47 - AUX_COMPA"]
    AUX_COMPA = 47,
    #[doc = "48 - AUX_ADC_IRQ"]
    AUX_ADC_IRQ = 48,
    #[doc = "49 - TRNG_IRQ"]
    TRNG_IRQ = 49,
    #[doc = "50 - OSC_COMB"]
    OSC_COMB = 50,
    #[doc = "51 - AUX_TIMER2_EV0"]
    AUX_TIMER2_EV0 = 51,
    #[doc = "52 - UART1_COMB"]
    UART1_COMB = 52,
    #[doc = "53 - BATMON_COMB"]
    BATMON_COMB = 53,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub struct ADI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADI2 {}
impl ADI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adi2::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for ADI2 {
    type Target = adi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADI2::ptr() }
    }
}
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub mod adi2;
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub struct ADI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADI3 {}
impl ADI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adi3::RegisterBlock {
        0x4008_6200 as *const _
    }
}
impl Deref for ADI3 {
    type Target = adi3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADI3::ptr() }
    }
}
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub mod adi3;
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_BATMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_BATMON {}
impl AON_BATMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_batmon::RegisterBlock {
        0x4009_5000 as *const _
    }
}
impl Deref for AON_BATMON {
    type Target = aon_batmon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_BATMON::ptr() }
    }
}
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_batmon;
#[doc = "This module configures the event fabric located in the AON domain. ote: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_EVENT {}
impl AON_EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_event::RegisterBlock {
        0x4009_3000 as *const _
    }
}
impl Deref for AON_EVENT {
    type Target = aon_event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_EVENT::ptr() }
    }
}
#[doc = "This module configures the event fabric located in the AON domain. ote: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_event;
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_IOC {}
impl AON_IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_ioc::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for AON_IOC {
    type Target = aon_ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_IOC::ptr() }
    }
}
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_ioc;
#[doc = "This component control the Power Management controller residing in the AON domain. ote: This module is only supporting 32 bit Read Write access from MCU"]
pub struct AON_PMCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_PMCTL {}
impl AON_PMCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_pmctl::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for AON_PMCTL {
    type Target = aon_pmctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_PMCTL::ptr() }
    }
}
#[doc = "This component control the Power Management controller residing in the AON domain. ote: This module is only supporting 32 bit Read Write access from MCU"]
pub mod aon_pmctl;
#[doc = "This component control the Real Time Clock residing in AON ote: This module is only supporting 32 bit ReadWrite access."]
pub struct AON_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_RTC {}
impl AON_RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_rtc::RegisterBlock {
        0x4009_2000 as *const _
    }
}
impl Deref for AON_RTC {
    type Target = aon_rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_RTC::ptr() }
    }
}
#[doc = "This component control the Real Time Clock residing in AON ote: This module is only supporting 32 bit ReadWrite access."]
pub mod aon_rtc;
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub struct AUX_ADI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ADI4 {}
impl AUX_ADI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_adi4::RegisterBlock {
        0x400c_b000 as *const _
    }
}
impl Deref for AUX_ADI4 {
    type Target = aux_adi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ADI4::ptr() }
    }
}
#[doc = "Analog Digital Interface his is a generic module for handling register information between digital and analog domain. o see the actual contents connected on the analog side, please see: DI2: ADI_2_REFSYS:ADI_2_REFSYS DI3: ADI_3_REFSYS:ADI_3_REFSYS DI4: ADI_4_AUX:ADI4_AUX"]
pub mod aux_adi4;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO0 {}
impl AUX_AIODIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio0::RegisterBlock {
        0x400c_c000 as *const _
    }
}
impl Deref for AUX_AIODIO0 {
    type Target = aux_aiodio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO0::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub mod aux_aiodio0;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO1 {}
impl AUX_AIODIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio1::RegisterBlock {
        0x400c_d000 as *const _
    }
}
impl Deref for AUX_AIODIO1 {
    type Target = aux_aiodio1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO1::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub mod aux_aiodio1;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO2 {}
impl AUX_AIODIO2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio2::RegisterBlock {
        0x400c_e000 as *const _
    }
}
impl Deref for AUX_AIODIO2 {
    type Target = aux_aiodio2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO2::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub mod aux_aiodio2;
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO[n]
does not map to DIO[n]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub struct AUX_AIODIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO3 {}
impl AUX_AIODIO3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio3::RegisterBlock {
        0x400c_f000 as *const _
    }
}
impl Deref for AUX_AIODIO3 {
    type Target = aux_aiodio3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO3::ptr() }
    }
}
#[doc = "AUX Analog Digital Input Output Controller (AUX_AIODIO) controls the general purpose input output pins of the AUX domain. These pins are referenced as AUXIO and can: be connected to analog AUX modules, such as comparators and ADC. be used by AUX_SCE. connect to AUX_SPIM SCLK, MISO and MOSI signals. connect to the asynchronous AUX event bus. nabled digital inputs are synchronized at SCE clock rate. ote that the IO mapping in the AUX domain is different from the IO mapping in the MCU domain. This means that AUXIO\\[n\\]
does not map to DIO\\[n\\]. AUXIO-DIO remapping is handled by Sensor Controller Studio."]
pub mod aux_aiodio3;
#[doc = "AUX Analog Interface (AUX_ANAIF) encapsulates direct data and control interfaces between AUX digital and AUX analog circuits. It lets AUX_SCE, UDMA0, and system CPU: Trigger ADC sample and conversion process. Write ADC samples to FIFO. Charge analog nodes by the use of the analog ISRC module. See ADI_4_AUX:ISRC and ADI_4_AUX:COMP.COMPA_REF_CURR_EN for further information. Use the DAC to generate a programmable voltage on COMPB_REF, COMPA_REF, or COMPA_IN analog nodes. o use: ADC : AUX_SCE must request active operational mode with AON_PMCTL:AUXSCECLK.SRC set to SCLK_HFDIV2. There are no requirements for system CPU. ISRC : AUX_SCE must request active operational mode. There are no requirements for system CPU. DAC : AUX_SCE must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to SCE_RATE as long as DAC state machine generates the sample clock. System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE as long as DAC state machine generates the sample clock. See DACSMPLCTL.EN for further information."]
pub struct AUX_ANAIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ANAIF {}
impl AUX_ANAIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_anaif::RegisterBlock {
        0x400c_9000 as *const _
    }
}
impl Deref for AUX_ANAIF {
    type Target = aux_anaif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ANAIF::ptr() }
    }
}
#[doc = "AUX Analog Interface (AUX_ANAIF) encapsulates direct data and control interfaces between AUX digital and AUX analog circuits. It lets AUX_SCE, UDMA0, and system CPU: Trigger ADC sample and conversion process. Write ADC samples to FIFO. Charge analog nodes by the use of the analog ISRC module. See ADI_4_AUX:ISRC and ADI_4_AUX:COMP.COMPA_REF_CURR_EN for further information. Use the DAC to generate a programmable voltage on COMPB_REF, COMPA_REF, or COMPA_IN analog nodes. o use: ADC : AUX_SCE must request active operational mode with AON_PMCTL:AUXSCECLK.SRC set to SCLK_HFDIV2. There are no requirements for system CPU. ISRC : AUX_SCE must request active operational mode. There are no requirements for system CPU. DAC : AUX_SCE must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to SCE_RATE as long as DAC state machine generates the sample clock. System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE as long as DAC state machine generates the sample clock. See DACSMPLCTL.EN for further information."]
pub mod aux_anaif;
#[doc = "Digital to Digital Interface his is a generic module for handling register information between digital core and and digital subchips within the analog domain. o see the actual contents connected on the analog side, please see: DI0: DDI_0_OSC:DDI_0_OSC"]
pub struct AUX_DDI0_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_DDI0_OSC {}
impl AUX_DDI0_OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_ddi0_osc::RegisterBlock {
        0x400c_a000 as *const _
    }
}
impl Deref for AUX_DDI0_OSC {
    type Target = aux_ddi0_osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_DDI0_OSC::ptr() }
    }
}
#[doc = "Digital to Digital Interface his is a generic module for handling register information between digital core and and digital subchips within the analog domain. o see the actual contents connected on the analog side, please see: DI0: DDI_0_OSC:DDI_0_OSC"]
pub mod aux_ddi0_osc;
#[doc = "AUX Event Controller (AUX_EVCTL) assembles events originating from: AUX submodules, including ADC and comparators. AUXIO. EVENT. AON_PMCTL. AON_RTC. AON_BATMON. nto two 64-bit event buses. One is synchronized to the AUX clock and one is left unsynchronized. he subscribers to the synchronous event bus are AUX_TIMER01, AUX_SCE and AUX_EVCTL. he subscribers to the asynchronous event bus are AUX_TIMER2, AUX_ANAIF, AUX_TDC and AUX_SYSIF. UX_EVCTL uses the synchronous event bus to generate events to AON_EVENT and EVENT, as well as to AUX_SCE. AUX_SCE can poll event status registers and combine certain instructions like WEV0, WEV1 with one or two configurable events. The latter saves power when execution must stall until a condition is met."]
pub struct AUX_EVCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_EVCTL {}
impl AUX_EVCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_evctl::RegisterBlock {
        0x400c_5000 as *const _
    }
}
impl Deref for AUX_EVCTL {
    type Target = aux_evctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_EVCTL::ptr() }
    }
}
#[doc = "AUX Event Controller (AUX_EVCTL) assembles events originating from: AUX submodules, including ADC and comparators. AUXIO. EVENT. AON_PMCTL. AON_RTC. AON_BATMON. nto two 64-bit event buses. One is synchronized to the AUX clock and one is left unsynchronized. he subscribers to the synchronous event bus are AUX_TIMER01, AUX_SCE and AUX_EVCTL. he subscribers to the asynchronous event bus are AUX_TIMER2, AUX_ANAIF, AUX_TDC and AUX_SYSIF. UX_EVCTL uses the synchronous event bus to generate events to AON_EVENT and EVENT, as well as to AUX_SCE. AUX_SCE can poll event status registers and combine certain instructions like WEV0, WEV1 with one or two configurable events. The latter saves power when execution must stall until a condition is met."]
pub mod aux_evctl;
#[doc = "The AUX Multiply-Accumulate (AUX_MAC) peripheral enables AUX_SCE with power-efficient and flexible mathematical operations: 2's complement signed and unsigned sequential multiplication (MUL) with optional accumulation of the result (MAC). 16 or 32-bit 2's complement signed and unsigned addition of configurable term and accumulator (ADD). Results of ADD, MUL and MAC operations are always stored in the accumulator (ACC). oftware can easily: Access arbitrary 16-bit slice of the 40-bit accumulator. Find the number of leading zero or sign bits. Perform shift operations on the accumulator. UX_SCE must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to SCE_RATE to access and use AUX_MAC. System CPU must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to BUS_RATE to access and use AUX_MAC. This guarantees constant execution times for ADD, MUL, and MAC operations. he ADD operation requires a single peripheral clock cycle to finish. MUL and MAC operations require four peripheral clock periods to finish. An unfinished ADD, MUL, or MAC operation stalls register access to this peripheral. AUX_SCE becomes clock gated if it encounters a bus stall. Software can use this to reduce power consumption during back to back accesses. nly full word access is supported by the peripheral. An attempt to write a single byte will have no effect."]
pub struct AUX_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_MAC {}
impl AUX_MAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_mac::RegisterBlock {
        0x400c_2000 as *const _
    }
}
impl Deref for AUX_MAC {
    type Target = aux_mac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_MAC::ptr() }
    }
}
#[doc = "The AUX Multiply-Accumulate (AUX_MAC) peripheral enables AUX_SCE with power-efficient and flexible mathematical operations: 2's complement signed and unsigned sequential multiplication (MUL) with optional accumulation of the result (MAC). 16 or 32-bit 2's complement signed and unsigned addition of configurable term and accumulator (ADD). Results of ADD, MUL and MAC operations are always stored in the accumulator (ACC). oftware can easily: Access arbitrary 16-bit slice of the 40-bit accumulator. Find the number of leading zero or sign bits. Perform shift operations on the accumulator. UX_SCE must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to SCE_RATE to access and use AUX_MAC. System CPU must set AUX_SYSIF:PEROPRATE.MAC_OP_RATE to BUS_RATE to access and use AUX_MAC. This guarantees constant execution times for ADD, MUL, and MAC operations. he ADD operation requires a single peripheral clock cycle to finish. MUL and MAC operations require four peripheral clock periods to finish. An unfinished ADD, MUL, or MAC operation stalls register access to this peripheral. AUX_SCE becomes clock gated if it encounters a bus stall. Software can use this to reduce power consumption during back to back accesses. nly full word access is supported by the peripheral. An attempt to write a single byte will have no effect."]
pub mod aux_mac;
#[doc = "AUX Sensor Control Engine (AUX_SCE) is a RISC-style microprocessor with separate fetch and execution cycles. It is optimized for low power and simple operations. AUX_SCE code and data segments are stored in AUX_RAM. AON_PMCTL:AUXSCECLK sets the operational frequency."]
pub struct AUX_SCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SCE {}
impl AUX_SCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_sce::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for AUX_SCE {
    type Target = aux_sce::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SCE::ptr() }
    }
}
#[doc = "AUX Sensor Control Engine (AUX_SCE) is a RISC-style microprocessor with separate fetch and execution cycles. It is optimized for low power and simple operations. AUX_SCE code and data segments are stored in AUX_RAM. AON_PMCTL:AUXSCECLK sets the operational frequency."]
pub mod aux_sce;
#[doc = "AUX Semaphore (AUX_SMPH) provides hardware means to share modules in AUX safely between CPUs based on resource ownership. UX_SMPH operates at AUX bus rate."]
pub struct AUX_SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SMPH {}
impl AUX_SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_smph::RegisterBlock {
        0x400c_8000 as *const _
    }
}
impl Deref for AUX_SMPH {
    type Target = aux_smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SMPH::ptr() }
    }
}
#[doc = "AUX Semaphore (AUX_SMPH) provides hardware means to share modules in AUX safely between CPUs based on resource ownership. UX_SMPH operates at AUX bus rate."]
pub mod aux_smph;
#[doc = "The AUX Serial Peripheral Interface Master (AUX_SPIM) enables AUX_SCE with power-efficient SPI communication. t is not possible to write a register while SPI transmission occurs. An attempt to do so will stall the bus until transmission is complete. ead of RX8.DATA or RX16.DATA stalls the bus until LSB has been captured. Read of SCLKIDLE.STAT or DATAIDLE.STAT stalls the bus until condition described is met. Other read operations do not stall the bus. UX_SCE becomes clock gated if it encounters a bus stall. This is useful as AUX_SCE can write TX8.DATA and then read RX8.DATA immediately to read a SPI slave. In such case there is no need for software to wait or to poll registers. UX_SYSIF:PEROPRATE.SPIM_OP_RATE selects the peripheral clock frequency which is used to derive the SCLK frequency. UX_SCE must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to SCE_RATE to access and use AUX_SPIM. System CPU must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to BUS_RATE to access and use AUX_SPIM. Failure to do so can result in incorrect SPI transmission."]
pub struct AUX_SPIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SPIM {}
impl AUX_SPIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_spim::RegisterBlock {
        0x400c_1000 as *const _
    }
}
impl Deref for AUX_SPIM {
    type Target = aux_spim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SPIM::ptr() }
    }
}
#[doc = "The AUX Serial Peripheral Interface Master (AUX_SPIM) enables AUX_SCE with power-efficient SPI communication. t is not possible to write a register while SPI transmission occurs. An attempt to do so will stall the bus until transmission is complete. ead of RX8.DATA or RX16.DATA stalls the bus until LSB has been captured. Read of SCLKIDLE.STAT or DATAIDLE.STAT stalls the bus until condition described is met. Other read operations do not stall the bus. UX_SCE becomes clock gated if it encounters a bus stall. This is useful as AUX_SCE can write TX8.DATA and then read RX8.DATA immediately to read a SPI slave. In such case there is no need for software to wait or to poll registers. UX_SYSIF:PEROPRATE.SPIM_OP_RATE selects the peripheral clock frequency which is used to derive the SCLK frequency. UX_SCE must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to SCE_RATE to access and use AUX_SPIM. System CPU must set AUX_SYSIF:PEROPRATE.SPIM_OP_RATE to BUS_RATE to access and use AUX_SPIM. Failure to do so can result in incorrect SPI transmission."]
pub mod aux_spim;
#[doc = "AUX System Interface (AUX_SYSIF) is responsible for: system resource requests, such as power supply, clock and, wakeup requests. configuration of AUX peripheral operational rates for AUX_SPIM, AUX_MAC, AUX_ANAIF DAC state machine and AUX_TIMER01. configuration of event synchronization rate for AUX_EVCTL:EVSTAT2 and AUX_EVCTL:EVSTAT3. configuration of AUX_SCE wakeup vectors that trigger AUX_SCE execution from sleep. eripheral operational rate for AUX modules mentioned above can either be: SCE rate, which is configured in AON_PMCTL:AUXSCECLK. AUX bus rate, which equals SCE rate or SCLK_HF divided by two when MCU domain is active or AUX operational mode is active. UX_SYSIF also interfaces AON_RTC and AON_BATMON to enable read access to data and sub-second increment control of AON_RTC."]
pub struct AUX_SYSIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SYSIF {}
impl AUX_SYSIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_sysif::RegisterBlock {
        0x400c_6000 as *const _
    }
}
impl Deref for AUX_SYSIF {
    type Target = aux_sysif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SYSIF::ptr() }
    }
}
#[doc = "AUX System Interface (AUX_SYSIF) is responsible for: system resource requests, such as power supply, clock and, wakeup requests. configuration of AUX peripheral operational rates for AUX_SPIM, AUX_MAC, AUX_ANAIF DAC state machine and AUX_TIMER01. configuration of event synchronization rate for AUX_EVCTL:EVSTAT2 and AUX_EVCTL:EVSTAT3. configuration of AUX_SCE wakeup vectors that trigger AUX_SCE execution from sleep. eripheral operational rate for AUX modules mentioned above can either be: SCE rate, which is configured in AON_PMCTL:AUXSCECLK. AUX bus rate, which equals SCE rate or SCLK_HF divided by two when MCU domain is active or AUX operational mode is active. UX_SYSIF also interfaces AON_RTC and AON_BATMON to enable read access to data and sub-second increment control of AON_RTC."]
pub mod aux_sysif;
#[doc = "AUX Time To Digital Converter (AUX_TDC) is used to measure the time between two events with high resolution. UX_TDC consists of a state machine that operates at AUX bus rate and an asynchronous fast-counter which is clocked by the TDC clock. DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL configures TDC clock source. The fast-counter counts on both edges of the TDC clock to double the resolution. ee the Technical Reference Manual for event timing requirements."]
pub struct AUX_TDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TDC {}
impl AUX_TDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_tdc::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for AUX_TDC {
    type Target = aux_tdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TDC::ptr() }
    }
}
#[doc = "AUX Time To Digital Converter (AUX_TDC) is used to measure the time between two events with high resolution. UX_TDC consists of a state machine that operates at AUX bus rate and an asynchronous fast-counter which is clocked by the TDC clock. DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL configures TDC clock source. The fast-counter counts on both edges of the TDC clock to double the resolution. ee the Technical Reference Manual for event timing requirements."]
pub mod aux_tdc;
#[doc = "AUX Timer 0 and AUX Timer 1 (AUX_TIMER01) are two 16-bit timers capable of generating one event each: AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV. AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV. he events are described in T0TARGET and T1TARGET. Subscribers to the AUX event bus can use these events to sequence and trigger actions. UX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the peripheral clock frequency used by the prescaler, timer, and event logic to SCE or AUX bus rate. To use AUX_TIMER01: AUX_SCE must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to SCE_RATE. System CPU must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to BUS_RATE. The timers must only subscribe to events updated at the peripheral clock frequency or lower. nexpected execution behavior can result if software does not obey these rules."]
pub struct AUX_TIMER01 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER01 {}
impl AUX_TIMER01 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_timer01::RegisterBlock {
        0x400c_7000 as *const _
    }
}
impl Deref for AUX_TIMER01 {
    type Target = aux_timer01::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TIMER01::ptr() }
    }
}
#[doc = "AUX Timer 0 and AUX Timer 1 (AUX_TIMER01) are two 16-bit timers capable of generating one event each: AUX_EVCTL:EVSTAT3.AUX_TIMER0_EV. AUX_EVCTL:EVSTAT3.AUX_TIMER1_EV. he events are described in T0TARGET and T1TARGET. Subscribers to the AUX event bus can use these events to sequence and trigger actions. UX_SYSIF:PEROPRATE.TIMER01_OP_RATE sets the peripheral clock frequency used by the prescaler, timer, and event logic to SCE or AUX bus rate. To use AUX_TIMER01: AUX_SCE must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to SCE_RATE. System CPU must set AUX_SYSIF:PEROPRATE.TIMER01_OP_RATE to BUS_RATE. The timers must only subscribe to events updated at the peripheral clock frequency or lower. nexpected execution behavior can result if software does not obey these rules."]
pub mod aux_timer01;
#[doc = "AUX Timer2 (AUX_TIMER2) offers flexible: generation of waveforms and events. capture of signal period and duty cycle. generation of single clock pulse. t consists of a: 16-bit counter. 4 capture compare channels. 4 event outputs, which are mapped to AUX event bus, see EVCTL. ach channel subscribes to the asynchronous AUX event bus. They can control one or more event outputs in both capture and compare modes. AUX_SYSIF:TIMER2CLKCTL.SRC selects clock source for the timer."]
pub struct AUX_TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER2 {}
impl AUX_TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_timer2::RegisterBlock {
        0x400c_3000 as *const _
    }
}
impl Deref for AUX_TIMER2 {
    type Target = aux_timer2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TIMER2::ptr() }
    }
}
#[doc = "AUX Timer2 (AUX_TIMER2) offers flexible: generation of waveforms and events. capture of signal period and duty cycle. generation of single clock pulse. t consists of a: 16-bit counter. 4 capture compare channels. 4 event outputs, which are mapped to AUX event bus, see EVCTL. ach channel subscribes to the asynchronous AUX event bus. They can control one or more event outputs in both capture and compare modes. AUX_SYSIF:TIMER2CLKCTL.SRC selects clock source for the timer."]
pub mod aux_timer2;
#[doc = "Customer configuration area (CCFG)"]
pub struct CCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCFG {}
impl CCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccfg::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for CCFG {
    type Target = ccfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCFG::ptr() }
    }
}
#[doc = "Customer configuration area (CCFG)"]
pub mod ccfg;
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub struct CPU_DWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_DWT {}
impl CPU_DWT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_dwt::RegisterBlock {
        0xe000_1000 as *const _
    }
}
impl Deref for CPU_DWT {
    type Target = cpu_dwt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_DWT::ptr() }
    }
}
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub mod cpu_dwt;
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub struct CPU_FPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_FPB {}
impl CPU_FPB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_fpb::RegisterBlock {
        0xe000_2000 as *const _
    }
}
impl Deref for CPU_FPB {
    type Target = cpu_fpb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_FPB::ptr() }
    }
}
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub mod cpu_fpb;
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub struct CPU_ITM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_ITM {}
impl CPU_ITM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_itm::RegisterBlock {
        0xe000_0000 as *const _
    }
}
impl Deref for CPU_ITM {
    type Target = cpu_itm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_ITM::ptr() }
    }
}
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub mod cpu_itm;
#[doc = "Cortex-M's ROM table"]
pub struct CPU_ROM_TABLE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_ROM_TABLE {}
impl CPU_ROM_TABLE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_rom_table::RegisterBlock {
        0xe00f_f000 as *const _
    }
}
impl Deref for CPU_ROM_TABLE {
    type Target = cpu_rom_table::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_ROM_TABLE::ptr() }
    }
}
#[doc = "Cortex-M's ROM table"]
pub mod cpu_rom_table;
#[doc = "Cortex-M's System Control Space (SCS)"]
pub struct CPU_SCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_SCS {}
impl CPU_SCS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_scs::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for CPU_SCS {
    type Target = cpu_scs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_SCS::ptr() }
    }
}
#[doc = "Cortex-M's System Control Space (SCS)"]
pub mod cpu_scs;
#[doc = "Cortex-M's TI proprietary registers"]
pub struct CPU_TIPROP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TIPROP {}
impl CPU_TIPROP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_tiprop::RegisterBlock {
        0xe00f_e000 as *const _
    }
}
impl Deref for CPU_TIPROP {
    type Target = cpu_tiprop::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_TIPROP::ptr() }
    }
}
#[doc = "Cortex-M's TI proprietary registers"]
pub mod cpu_tiprop;
#[doc = "Cortex-M's Trace Port Interface Unit (TPIU)"]
pub struct CPU_TPIU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TPIU {}
impl CPU_TPIU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_tpiu::RegisterBlock {
        0xe004_0000 as *const _
    }
}
impl Deref for CPU_TPIU {
    type Target = cpu_tpiu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_TPIU::ptr() }
    }
}
#[doc = "Cortex-M's Trace Port Interface Unit (TPIU)"]
pub mod cpu_tpiu;
#[doc = "Factory configuration area (FCFG1)"]
pub struct FCFG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCFG1 {}
impl FCFG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fcfg1::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for FCFG1 {
    type Target = fcfg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCFG1::ptr() }
    }
}
#[doc = "Factory configuration area (FCFG1)"]
pub mod fcfg1;
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub mod flash;
#[doc = "General Purpose Timer."]
pub struct GPT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT0 {}
impl GPT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for GPT0 {
    type Target = gpt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT0::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt0;
#[doc = "General Purpose Timer."]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
impl GPT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt1;
#[doc = "General Purpose Timer."]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
impl GPT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt2::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for GPT2 {
    type Target = gpt2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt2;
#[doc = "General Purpose Timer."]
pub struct GPT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT3 {}
impl GPT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt3::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for GPT3 {
    type Target = gpt3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT3::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt3;
#[doc = "Integrated module which combines the Public Key Acceleration module, optional True Random Gnerator, optional interrupt controller and a standard bus interface"]
pub struct PKA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA {}
impl PKA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka::RegisterBlock {
        0x4002_5000 as *const _
    }
}
impl Deref for PKA {
    type Target = pka::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA::ptr() }
    }
}
#[doc = "Integrated module which combines the Public Key Acceleration module, optional True Random Gnerator, optional interrupt controller and a standard bus interface"]
pub mod pka;
#[doc = "Integrated module which includes the PKA K"]
pub struct PKA_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA_INT {}
impl PKA_INT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka_int::RegisterBlock {
        0x4002_7000 as *const _
    }
}
impl Deref for PKA_INT {
    type Target = pka_int::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA_INT::ptr() }
    }
}
#[doc = "Integrated module which includes the PKA K"]
pub mod pka_int;
#[doc = "This is the a SRAM within the PKA domain, intended for PKA usage only."]
pub struct PKA_RAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA_RAM {}
impl PKA_RAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka_ram::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for PKA_RAM {
    type Target = pka_ram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA_RAM::ptr() }
    }
}
#[doc = "This is the a SRAM within the PKA domain, intended for PKA usage only."]
pub mod pka_ram;
#[doc = "RF core doorbell he doorbell module is the main user interface to the radio sub-system. It contains the registers used for both submitting commands to the radio, and for configuring radio interrupts from the RF core."]
pub struct RFC_DBELL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_DBELL {}
impl RFC_DBELL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_dbell::RegisterBlock {
        0x4004_1000 as *const _
    }
}
impl Deref for RFC_DBELL {
    type Target = rfc_dbell::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_DBELL::ptr() }
    }
}
#[doc = "RF core doorbell he doorbell module is the main user interface to the radio sub-system. It contains the registers used for both submitting commands to the radio, and for configuring radio interrupts from the RF core."]
pub mod rfc_dbell;
#[doc = "RF core power management his module contains clock control for all RF core sub-modules."]
pub struct RFC_PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_PWR {}
impl RFC_PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_pwr::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for RFC_PWR {
    type Target = rfc_pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_PWR::ptr() }
    }
}
#[doc = "RF core power management his module contains clock control for all RF core sub-modules."]
pub mod rfc_pwr;
#[doc = "RF core radio timer"]
pub struct RFC_RAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_RAT {}
impl RFC_RAT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_rat::RegisterBlock {
        0x4004_3000 as *const _
    }
}
impl Deref for RFC_RAT {
    type Target = rfc_rat::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_RAT::ptr() }
    }
}
#[doc = "RF core radio timer"]
pub mod rfc_rat;
#[doc = "Command and packet engine RAM (CPERAM) in the RF core"]
pub struct RFC_ULLRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_ULLRAM {}
impl RFC_ULLRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_ullram::RegisterBlock {
        0x2100_4000 as *const _
    }
}
impl Deref for RFC_ULLRAM {
    type Target = rfc_ullram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_ULLRAM::ptr() }
    }
}
#[doc = "Command and packet engine RAM (CPERAM) in the RF core"]
pub mod rfc_ullram;
#[doc = "General Purpose RAM"]
pub struct SRAM_MMR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRAM_MMR {}
impl SRAM_MMR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sram_mmr::RegisterBlock {
        0x4003_5000 as *const _
    }
}
impl Deref for SRAM_MMR {
    type Target = sram_mmr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SRAM_MMR::ptr() }
    }
}
#[doc = "General Purpose RAM"]
pub mod sram_mmr;
#[doc = "DMA Crypto Core is a low power low gate count crypto core with DMA capability and local key storage."]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "DMA Crypto Core is a low power low gate count crypto core with DMA capability and local key storage."]
pub mod crypto;
#[doc = "Event Fabric Component Definition"]
pub struct EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVENT {}
impl EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const event::RegisterBlock {
        0x4008_3000 as *const _
    }
}
impl Deref for EVENT {
    type Target = event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVENT::ptr() }
    }
}
#[doc = "Event Fabric Component Definition"]
pub mod event;
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub mod gpio;
#[doc = "I2CMaster/Slave Serial Controler"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2CMaster/Slave Serial Controler"]
pub mod i2c0;
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub mod i2s0;
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
impl IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ioc::RegisterBlock {
        0x4008_1000 as *const _
    }
}
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub mod ioc;
#[doc = "Power, Reset and Clock Management"]
pub struct PRCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCM {}
impl PRCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prcm::RegisterBlock {
        0x4008_2000 as *const _
    }
}
impl Deref for PRCM {
    type Target = prcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PRCM::ptr() }
    }
}
#[doc = "Power, Reset and Clock Management"]
pub mod prcm;
#[doc = "MCU Semaphore Module his module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub struct SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMPH {}
impl SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smph::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for SMPH {
    type Target = smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMPH::ptr() }
    }
}
#[doc = "MCU Semaphore Module his module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub mod smph;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi0;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub mod uart1;
#[doc = "ARM Micro Direct Memory Access Controller"]
pub struct UDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA0 {}
impl UDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for UDMA0 {
    type Target = udma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA0::ptr() }
    }
}
#[doc = "ARM Micro Direct Memory Access Controller"]
pub mod udma0;
#[doc = "Versatile Instruction Memory System ontrols memory access to the Flash and encapsulates the following instruction memories: Boot ROM Cache / GPRAM"]
pub struct VIMS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIMS {}
impl VIMS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vims::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for VIMS {
    type Target = vims::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VIMS::ptr() }
    }
}
#[doc = "Versatile Instruction Memory System ontrols memory access to the Flash and encapsulates the following instruction memories: Boot ROM Cache / GPRAM"]
pub mod vims;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADI2"]
    pub ADI2: ADI2,
    #[doc = "ADI3"]
    pub ADI3: ADI3,
    #[doc = "AON_BATMON"]
    pub AON_BATMON: AON_BATMON,
    #[doc = "AON_EVENT"]
    pub AON_EVENT: AON_EVENT,
    #[doc = "AON_IOC"]
    pub AON_IOC: AON_IOC,
    #[doc = "AON_PMCTL"]
    pub AON_PMCTL: AON_PMCTL,
    #[doc = "AON_RTC"]
    pub AON_RTC: AON_RTC,
    #[doc = "AUX_ADI4"]
    pub AUX_ADI4: AUX_ADI4,
    #[doc = "AUX_AIODIO0"]
    pub AUX_AIODIO0: AUX_AIODIO0,
    #[doc = "AUX_AIODIO1"]
    pub AUX_AIODIO1: AUX_AIODIO1,
    #[doc = "AUX_AIODIO2"]
    pub AUX_AIODIO2: AUX_AIODIO2,
    #[doc = "AUX_AIODIO3"]
    pub AUX_AIODIO3: AUX_AIODIO3,
    #[doc = "AUX_ANAIF"]
    pub AUX_ANAIF: AUX_ANAIF,
    #[doc = "AUX_DDI0_OSC"]
    pub AUX_DDI0_OSC: AUX_DDI0_OSC,
    #[doc = "AUX_EVCTL"]
    pub AUX_EVCTL: AUX_EVCTL,
    #[doc = "AUX_MAC"]
    pub AUX_MAC: AUX_MAC,
    #[doc = "AUX_SCE"]
    pub AUX_SCE: AUX_SCE,
    #[doc = "AUX_SMPH"]
    pub AUX_SMPH: AUX_SMPH,
    #[doc = "AUX_SPIM"]
    pub AUX_SPIM: AUX_SPIM,
    #[doc = "AUX_SYSIF"]
    pub AUX_SYSIF: AUX_SYSIF,
    #[doc = "AUX_TDC"]
    pub AUX_TDC: AUX_TDC,
    #[doc = "AUX_TIMER01"]
    pub AUX_TIMER01: AUX_TIMER01,
    #[doc = "AUX_TIMER2"]
    pub AUX_TIMER2: AUX_TIMER2,
    #[doc = "CCFG"]
    pub CCFG: CCFG,
    #[doc = "CPU_DWT"]
    pub CPU_DWT: CPU_DWT,
    #[doc = "CPU_FPB"]
    pub CPU_FPB: CPU_FPB,
    #[doc = "CPU_ITM"]
    pub CPU_ITM: CPU_ITM,
    #[doc = "CPU_ROM_TABLE"]
    pub CPU_ROM_TABLE: CPU_ROM_TABLE,
    #[doc = "CPU_SCS"]
    pub CPU_SCS: CPU_SCS,
    #[doc = "CPU_TIPROP"]
    pub CPU_TIPROP: CPU_TIPROP,
    #[doc = "CPU_TPIU"]
    pub CPU_TPIU: CPU_TPIU,
    #[doc = "FCFG1"]
    pub FCFG1: FCFG1,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "GPT0"]
    pub GPT0: GPT0,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "GPT3"]
    pub GPT3: GPT3,
    #[doc = "PKA"]
    pub PKA: PKA,
    #[doc = "PKA_INT"]
    pub PKA_INT: PKA_INT,
    #[doc = "PKA_RAM"]
    pub PKA_RAM: PKA_RAM,
    #[doc = "RFC_DBELL"]
    pub RFC_DBELL: RFC_DBELL,
    #[doc = "RFC_PWR"]
    pub RFC_PWR: RFC_PWR,
    #[doc = "RFC_RAT"]
    pub RFC_RAT: RFC_RAT,
    #[doc = "RFC_ULLRAM"]
    pub RFC_ULLRAM: RFC_ULLRAM,
    #[doc = "SRAM_MMR"]
    pub SRAM_MMR: SRAM_MMR,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "EVENT"]
    pub EVENT: EVENT,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "IOC"]
    pub IOC: IOC,
    #[doc = "PRCM"]
    pub PRCM: PRCM,
    #[doc = "SMPH"]
    pub SMPH: SMPH,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UDMA0"]
    pub UDMA0: UDMA0,
    #[doc = "VIMS"]
    pub VIMS: VIMS,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADI2: ADI2 {
                _marker: PhantomData,
            },
            ADI3: ADI3 {
                _marker: PhantomData,
            },
            AON_BATMON: AON_BATMON {
                _marker: PhantomData,
            },
            AON_EVENT: AON_EVENT {
                _marker: PhantomData,
            },
            AON_IOC: AON_IOC {
                _marker: PhantomData,
            },
            AON_PMCTL: AON_PMCTL {
                _marker: PhantomData,
            },
            AON_RTC: AON_RTC {
                _marker: PhantomData,
            },
            AUX_ADI4: AUX_ADI4 {
                _marker: PhantomData,
            },
            AUX_AIODIO0: AUX_AIODIO0 {
                _marker: PhantomData,
            },
            AUX_AIODIO1: AUX_AIODIO1 {
                _marker: PhantomData,
            },
            AUX_AIODIO2: AUX_AIODIO2 {
                _marker: PhantomData,
            },
            AUX_AIODIO3: AUX_AIODIO3 {
                _marker: PhantomData,
            },
            AUX_ANAIF: AUX_ANAIF {
                _marker: PhantomData,
            },
            AUX_DDI0_OSC: AUX_DDI0_OSC {
                _marker: PhantomData,
            },
            AUX_EVCTL: AUX_EVCTL {
                _marker: PhantomData,
            },
            AUX_MAC: AUX_MAC {
                _marker: PhantomData,
            },
            AUX_SCE: AUX_SCE {
                _marker: PhantomData,
            },
            AUX_SMPH: AUX_SMPH {
                _marker: PhantomData,
            },
            AUX_SPIM: AUX_SPIM {
                _marker: PhantomData,
            },
            AUX_SYSIF: AUX_SYSIF {
                _marker: PhantomData,
            },
            AUX_TDC: AUX_TDC {
                _marker: PhantomData,
            },
            AUX_TIMER01: AUX_TIMER01 {
                _marker: PhantomData,
            },
            AUX_TIMER2: AUX_TIMER2 {
                _marker: PhantomData,
            },
            CCFG: CCFG {
                _marker: PhantomData,
            },
            CPU_DWT: CPU_DWT {
                _marker: PhantomData,
            },
            CPU_FPB: CPU_FPB {
                _marker: PhantomData,
            },
            CPU_ITM: CPU_ITM {
                _marker: PhantomData,
            },
            CPU_ROM_TABLE: CPU_ROM_TABLE {
                _marker: PhantomData,
            },
            CPU_SCS: CPU_SCS {
                _marker: PhantomData,
            },
            CPU_TIPROP: CPU_TIPROP {
                _marker: PhantomData,
            },
            CPU_TPIU: CPU_TPIU {
                _marker: PhantomData,
            },
            FCFG1: FCFG1 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            GPT0: GPT0 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            GPT3: GPT3 {
                _marker: PhantomData,
            },
            PKA: PKA {
                _marker: PhantomData,
            },
            PKA_INT: PKA_INT {
                _marker: PhantomData,
            },
            PKA_RAM: PKA_RAM {
                _marker: PhantomData,
            },
            RFC_DBELL: RFC_DBELL {
                _marker: PhantomData,
            },
            RFC_PWR: RFC_PWR {
                _marker: PhantomData,
            },
            RFC_RAT: RFC_RAT {
                _marker: PhantomData,
            },
            RFC_ULLRAM: RFC_ULLRAM {
                _marker: PhantomData,
            },
            SRAM_MMR: SRAM_MMR {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            EVENT: EVENT {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            IOC: IOC {
                _marker: PhantomData,
            },
            PRCM: PRCM {
                _marker: PhantomData,
            },
            SMPH: SMPH {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UDMA0: UDMA0 {
                _marker: PhantomData,
            },
            VIMS: VIMS {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
