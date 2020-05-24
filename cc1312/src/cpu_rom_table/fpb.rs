#[doc = "Reader of register FPB"]
pub type R = crate::R<u32, super::FPB>;
#[doc = "Reader of field `FPB`"]
pub type FPB_R = crate::R<u32, u32>;
#[doc = "Reader of field `FPB_PRESENT`"]
pub type FPB_PRESENT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 1:31 - FPB"]
    #[inline(always)]
    pub fn fpb(&self) -> FPB_R {
        FPB_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - FPB_PRESENT"]
    #[inline(always)]
    pub fn fpb_present(&self) -> FPB_PRESENT_R {
        FPB_PRESENT_R::new((self.bits & 0x01) != 0)
    }
}
