#[doc = "Reader of register RFCMODEHWOPT"]
pub type R = crate::R<u32, super::RFCMODEHWOPT>;
#[doc = "Reader of field `AVAIL`"]
pub type AVAIL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - AVAIL"]
    #[inline(always)]
    pub fn avail(&self) -> AVAIL_R {
        AVAIL_R::new((self.bits & 0xff) as u8)
    }
}
