#[doc = "Reader of register CLAIMTAG"]
pub type R = crate::R<u32, super::CLAIMTAG>;
#[doc = "Reader of field `CLAIMTAG`"]
pub type CLAIMTAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CLAIMTAG"]
    #[inline(always)]
    pub fn claimtag(&self) -> CLAIMTAG_R {
        CLAIMTAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
