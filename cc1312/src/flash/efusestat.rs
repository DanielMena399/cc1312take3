#[doc = "Reader of register EFUSESTAT"]
pub type R = crate::R<u32, super::EFUSESTAT>;
#[doc = "Reader of field `RESETDONE`"]
pub type RESETDONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RESETDONE"]
    #[inline(always)]
    pub fn resetdone(&self) -> RESETDONE_R {
        RESETDONE_R::new((self.bits & 0x01) != 0)
    }
}
