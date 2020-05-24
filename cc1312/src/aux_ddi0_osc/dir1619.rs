#[doc = "Reader of register DIR1619"]
pub type R = crate::R<u32, super::DIR1619>;
#[doc = "Writer for register DIR1619"]
pub type W = crate::W<u32, super::DIR1619>;
#[doc = "Register DIR1619 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR1619 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B3`"]
pub type B3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B3`"]
pub struct B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `B2`"]
pub type B2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B2`"]
pub struct B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `B1`"]
pub type B1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B1`"]
pub struct B1_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `B0`"]
pub type B0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B0`"]
pub struct B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - B3"]
    #[inline(always)]
    pub fn b3(&self) -> B3_R {
        B3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - B2"]
    #[inline(always)]
    pub fn b2(&self) -> B2_R {
        B2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - B1"]
    #[inline(always)]
    pub fn b1(&self) -> B1_R {
        B1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - B0"]
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - B3"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W {
        B3_W { w: self }
    }
    #[doc = "Bits 16:23 - B2"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W {
        B2_W { w: self }
    }
    #[doc = "Bits 8:15 - B1"]
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W {
        B1_W { w: self }
    }
    #[doc = "Bits 0:7 - B0"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W {
        B0_W { w: self }
    }
}
