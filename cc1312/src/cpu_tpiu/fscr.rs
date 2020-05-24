#[doc = "Reader of register FSCR"]
pub type R = crate::R<u32, super::FSCR>;
#[doc = "Reader of field `FSCR`"]
pub type FSCR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FSCR"]
    #[inline(always)]
    pub fn fscr(&self) -> FSCR_R {
        FSCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
