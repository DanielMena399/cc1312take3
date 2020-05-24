#[doc = "Reader of register PCSR"]
pub type R = crate::R<u32, super::PCSR>;
#[doc = "Reader of field `EIASAMPLE`"]
pub type EIASAMPLE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EIASAMPLE"]
    #[inline(always)]
    pub fn eiasample(&self) -> EIASAMPLE_R {
        EIASAMPLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
