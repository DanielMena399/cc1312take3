#[doc = "Reader of register REMAP"]
pub type R = crate::R<u32, super::REMAP>;
#[doc = "Writer for register REMAP"]
pub type W = crate::W<u32, super::REMAP>;
#[doc = "Register REMAP `reset()`'s with value 0x2000_0000"]
impl crate::ResetValue for super::REMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
#[doc = "Reader of field `REMAP`"]
pub type REMAP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REMAP`"]
pub struct REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 5)) | (((value as u32) & 0x00ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:28 - REMAP"]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 5) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:28 - REMAP"]
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W {
        REMAP_W { w: self }
    }
}
