#[doc = "Writer for register HASHINLENH"]
pub type W = crate::W<u32, super::HASHINLENH>;
#[doc = "Register HASHINLENH `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHINLENH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LENGTH_IN`"]
pub struct LENGTH_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - LENGTH_IN"]
    #[inline(always)]
    pub fn length_in(&mut self) -> LENGTH_IN_W {
        LENGTH_IN_W { w: self }
    }
}
