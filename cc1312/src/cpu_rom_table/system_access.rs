#[doc = "Reader of register SYSTEM_ACCESS"]
pub type R = crate::R<u32, super::SYSTEM_ACCESS>;
#[doc = "Reader of field `SYSTEM_ACCESS`"]
pub type SYSTEM_ACCESS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SYSTEM_ACCESS"]
    #[inline(always)]
    pub fn system_access(&self) -> SYSTEM_ACCESS_R {
        SYSTEM_ACCESS_R::new((self.bits & 0x01) != 0)
    }
}
