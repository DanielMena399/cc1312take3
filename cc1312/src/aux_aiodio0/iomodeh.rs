#[doc = "Reader of register IOMODEH"]
pub type R = crate::R<u32, super::IOMODEH>;
#[doc = "Writer for register IOMODEH"]
pub type W = crate::W<u32, super::IOMODEH>;
#[doc = "Register IOMODEH `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMODEH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO7`"]
pub type IO7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO7`"]
pub struct IO7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IO6`"]
pub type IO6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO6`"]
pub struct IO6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `IO5`"]
pub type IO5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO5`"]
pub struct IO5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IO4`"]
pub type IO4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO4`"]
pub struct IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - IO7"]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - IO6"]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - IO5"]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - IO4"]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - IO7"]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W {
        IO7_W { w: self }
    }
    #[doc = "Bits 4:5 - IO6"]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W {
        IO6_W { w: self }
    }
    #[doc = "Bits 2:3 - IO5"]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W {
        IO5_W { w: self }
    }
    #[doc = "Bits 0:1 - IO4"]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W {
        IO4_W { w: self }
    }
}
