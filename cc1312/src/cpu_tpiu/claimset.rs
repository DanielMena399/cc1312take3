#[doc = "Writer for register CLAIMSET"]
pub type W = crate::W<u32, super::CLAIMSET>;
#[doc = "Register CLAIMSET `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::CLAIMSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Write proxy for field `CLAIMSET`"]
pub struct CLAIMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CLAIMSET"]
    #[inline(always)]
    pub fn claimset(&mut self) -> CLAIMSET_W {
        CLAIMSET_W { w: self }
    }
}
