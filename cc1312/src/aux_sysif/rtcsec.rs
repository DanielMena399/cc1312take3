#[doc = "Reader of register RTCSEC"]
pub type R = crate::R<u32, super::RTCSEC>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SEC"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
    }
}
