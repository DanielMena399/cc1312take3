#[doc = "Reader of register SCS"]
pub type R = crate::R<u32, super::SCS>;
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCS"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
