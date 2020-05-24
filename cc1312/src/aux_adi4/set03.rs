#[doc = "Writer for register SET03"]
pub type W = crate::W<u32, super::SET03>;
#[doc = "Register SET03 `reset()`'s with value 0"]
impl crate::ResetValue for super::SET03 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `S3`"]
pub struct S3_W<'a> {
    w: &'a mut W,
}
impl<'a> S3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `S2`"]
pub struct S2_W<'a> {
    w: &'a mut W,
}
impl<'a> S2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `S1`"]
pub struct S1_W<'a> {
    w: &'a mut W,
}
impl<'a> S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `S0`"]
pub struct S0_W<'a> {
    w: &'a mut W,
}
impl<'a> S0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 24:31 - S3"]
    #[inline(always)]
    pub fn s3(&mut self) -> S3_W {
        S3_W { w: self }
    }
    #[doc = "Bits 16:23 - S2"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2_W {
        S2_W { w: self }
    }
    #[doc = "Bits 8:15 - S1"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1_W {
        S1_W { w: self }
    }
    #[doc = "Bits 0:7 - S0"]
    #[inline(always)]
    pub fn s0(&mut self) -> S0_W {
        S0_W { w: self }
    }
}
