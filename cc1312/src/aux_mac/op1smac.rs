#[doc = "Writer for register OP1SMAC"]
pub type W = crate::W<u32, super::OP1SMAC>;
#[doc = "Register OP1SMAC `reset()`'s with value 0"]
impl crate::ResetValue for super::OP1SMAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OP1_VALUE`"]
pub struct OP1_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - OP1_VALUE"]
    #[inline(always)]
    pub fn op1_value(&mut self) -> OP1_VALUE_W {
        OP1_VALUE_W { w: self }
    }
}
