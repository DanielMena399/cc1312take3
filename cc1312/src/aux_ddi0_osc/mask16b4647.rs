#[doc = "Writer for register MASK16B4647"]
pub type W = crate::W<u32, super::MASK16B4647>;
#[doc = "Register MASK16B4647 `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK16B4647 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `D`"]
pub struct D_W<'a> {
    w: &'a mut W,
}
impl<'a> D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 16:31 - M"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bits 0:15 - D"]
    #[inline(always)]
    pub fn d(&mut self) -> D_W {
        D_W { w: self }
    }
}
