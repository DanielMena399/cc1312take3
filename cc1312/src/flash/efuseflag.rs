#[doc = "Reader of register EFUSEFLAG"]
pub type R = crate::R<u32, super::EFUSEFLAG>;
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - KEY"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0x01) != 0)
    }
}
