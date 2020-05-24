#[doc = "Reader of register CLAIMMASK"]
pub type R = crate::R<u32, super::CLAIMMASK>;
#[doc = "Reader of field `CLAIMMASK`"]
pub type CLAIMMASK_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CLAIMMASK"]
    #[inline(always)]
    pub fn claimmask(&self) -> CLAIMMASK_R {
        CLAIMMASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
