#[doc = "Reader of register IOMODEL"]
pub type R = crate::R<u32, super::IOMODEL>;
#[doc = "Writer for register IOMODEL"]
pub type W = crate::W<u32, super::IOMODEL>;
#[doc = "Register IOMODEL `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMODEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO3`"]
pub type IO3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO3`"]
pub struct IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IO2`"]
pub type IO2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO2`"]
pub struct IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `IO1`"]
pub type IO1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO1`"]
pub struct IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IO0`"]
pub type IO0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO0`"]
pub struct IO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - IO3"]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - IO2"]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - IO1"]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - IO0"]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - IO3"]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W {
        IO3_W { w: self }
    }
    #[doc = "Bits 4:5 - IO2"]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W {
        IO2_W { w: self }
    }
    #[doc = "Bits 2:3 - IO1"]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W {
        IO1_W { w: self }
    }
    #[doc = "Bits 0:1 - IO0"]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W {
        IO0_W { w: self }
    }
}
