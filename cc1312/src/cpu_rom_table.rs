#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Space Component"]
    pub scs: SCS,
    #[doc = "0x04 - Data Watchpoint and Trace Component oints to the Data Watchpoint and Trace block at 0xE0001000. Value has bit \\[0\\]
set if DWT is present."]
    pub dwt: DWT,
    #[doc = "0x08 - Flash Patch and Breakpoint Component oints to the Flash Patch and Breakpoint block at 0xE0002000. Value has bit \\[0\\]
set to 1 if FPB is present."]
    pub fpb: FPB,
    #[doc = "0x0c - Instrumentation Trace Component oints to the Instrumentation Trace block at 0xE0000000. Value has bit \\[0\\]
set if ITM is present."]
    pub itm: ITM,
    #[doc = "0x10 - Trace Port Interface Component oints to the TPIU. Value has bit \\[0\\]
set to 1 if TPIU is present. TPIU is at 0xE0040000."]
    pub tpiu: TPIU,
    #[doc = "0x14 - Enhanced Trace Component oints to the ETM. Value has bit \\[0\\]
set to 1 if ETM is present. ETM is at 0xE0041000."]
    pub etm: ETM,
    #[doc = "0x18 - End Marker arks the end of the ROM table. If CoreSight components are added, they are added starting from this location and the End marker is moved to the next location after the additional components."]
    pub end: END,
    _reserved7: [u8; 4016usize],
    #[doc = "0xfcc - System Memory Map Access for DAP"]
    pub system_access: SYSTEM_ACCESS,
}
#[doc = "System Control Space Component\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs](scs) module"]
pub type SCS = crate::Reg<u32, _SCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS;
#[doc = "`read()` method returns [scs::R](scs::R) reader structure"]
impl crate::Readable for SCS {}
#[doc = "System Control Space Component"]
pub mod scs;
#[doc = "Data Watchpoint and Trace Component oints to the Data Watchpoint and Trace block at 0xE0001000. Value has bit \\[0\\]
set if DWT is present.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dwt](dwt) module"]
pub type DWT = crate::Reg<u32, _DWT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DWT;
#[doc = "`read()` method returns [dwt::R](dwt::R) reader structure"]
impl crate::Readable for DWT {}
#[doc = "Data Watchpoint and Trace Component oints to the Data Watchpoint and Trace block at 0xE0001000. Value has bit \\[0\\]
set if DWT is present."]
pub mod dwt;
#[doc = "Flash Patch and Breakpoint Component oints to the Flash Patch and Breakpoint block at 0xE0002000. Value has bit \\[0\\]
set to 1 if FPB is present.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpb](fpb) module"]
pub type FPB = crate::Reg<u32, _FPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPB;
#[doc = "`read()` method returns [fpb::R](fpb::R) reader structure"]
impl crate::Readable for FPB {}
#[doc = "Flash Patch and Breakpoint Component oints to the Flash Patch and Breakpoint block at 0xE0002000. Value has bit \\[0\\]
set to 1 if FPB is present."]
pub mod fpb;
#[doc = "Instrumentation Trace Component oints to the Instrumentation Trace block at 0xE0000000. Value has bit \\[0\\]
set if ITM is present.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itm](itm) module"]
pub type ITM = crate::Reg<u32, _ITM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITM;
#[doc = "`read()` method returns [itm::R](itm::R) reader structure"]
impl crate::Readable for ITM {}
#[doc = "Instrumentation Trace Component oints to the Instrumentation Trace block at 0xE0000000. Value has bit \\[0\\]
set if ITM is present."]
pub mod itm;
#[doc = "Trace Port Interface Component oints to the TPIU. Value has bit \\[0\\]
set to 1 if TPIU is present. TPIU is at 0xE0040000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiu](tpiu) module"]
pub type TPIU = crate::Reg<u32, _TPIU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIU;
#[doc = "`read()` method returns [tpiu::R](tpiu::R) reader structure"]
impl crate::Readable for TPIU {}
#[doc = "Trace Port Interface Component oints to the TPIU. Value has bit \\[0\\]
set to 1 if TPIU is present. TPIU is at 0xE0040000."]
pub mod tpiu;
#[doc = "Enhanced Trace Component oints to the ETM. Value has bit \\[0\\]
set to 1 if ETM is present. ETM is at 0xE0041000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm](etm) module"]
pub type ETM = crate::Reg<u32, _ETM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETM;
#[doc = "`read()` method returns [etm::R](etm::R) reader structure"]
impl crate::Readable for ETM {}
#[doc = "Enhanced Trace Component oints to the ETM. Value has bit \\[0\\]
set to 1 if ETM is present. ETM is at 0xE0041000."]
pub mod etm;
#[doc = "End Marker arks the end of the ROM table. If CoreSight components are added, they are added starting from this location and the End marker is moved to the next location after the additional components.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end](end) module"]
pub type END = crate::Reg<u32, _END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _END;
#[doc = "`read()` method returns [end::R](end::R) reader structure"]
impl crate::Readable for END {}
#[doc = "End Marker arks the end of the ROM table. If CoreSight components are added, they are added starting from this location and the End marker is moved to the next location after the additional components."]
pub mod end;
#[doc = "System Memory Map Access for DAP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [system_access](system_access) module"]
pub type SYSTEM_ACCESS = crate::Reg<u32, _SYSTEM_ACCESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEM_ACCESS;
#[doc = "`read()` method returns [system_access::R](system_access::R) reader structure"]
impl crate::Readable for SYSTEM_ACCESS {}
#[doc = "System Memory Map Access for DAP"]
pub mod system_access;
