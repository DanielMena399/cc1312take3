#[doc = "Reader of register DEVID"]
pub type R = crate::R<u32, super::DEVID>;
#[doc = "Reader of field `DEVID`"]
pub type DEVID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DEVID"]
    #[inline(always)]
    pub fn devid(&self) -> DEVID_R {
        DEVID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
