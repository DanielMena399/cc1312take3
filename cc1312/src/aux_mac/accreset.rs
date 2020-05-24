#[doc = "Writer for register ACCRESET"]
pub type W = crate::W<u32, super::ACCRESET>;
#[doc = "Register ACCRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::ACCRESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRG`"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - TRG"]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
}
