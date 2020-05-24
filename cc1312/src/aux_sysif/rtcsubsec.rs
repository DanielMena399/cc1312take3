#[doc = "Reader of register RTCSUBSEC"]
pub type R = crate::R<u32, super::RTCSUBSEC>;
#[doc = "Reader of field `SUBSEC`"]
pub type SUBSEC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SUBSEC"]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
    }
}
