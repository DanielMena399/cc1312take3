#[doc = "Writer for register AESDATALEN1"]
pub type W = crate::W<u32, super::AESDATALEN1>;
#[doc = "Register AESDATALEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESDATALEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `C_LENGTH`"]
pub struct C_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> C_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:28 - C_LENGTH"]
    #[inline(always)]
    pub fn c_length(&mut self) -> C_LENGTH_W {
        C_LENGTH_W { w: self }
    }
}
