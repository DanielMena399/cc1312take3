#[doc = "Reader of register MCUWUSEL1"]
pub type R = crate::R<u32, super::MCUWUSEL1>;
#[doc = "Writer for register MCUWUSEL1"]
pub type W = crate::W<u32, super::MCUWUSEL1>;
#[doc = "Register MCUWUSEL1 `reset()`'s with value 0x3f3f_3f3f"]
impl crate::ResetValue for super::MCUWUSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f3f_3f3f
    }
}
#[doc = "Reader of field `WU7_EV`"]
pub type WU7_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU7_EV`"]
pub struct WU7_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU7_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `WU6_EV`"]
pub type WU6_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU6_EV`"]
pub struct WU6_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU6_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WU5_EV`"]
pub type WU5_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU5_EV`"]
pub struct WU5_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU5_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WU4_EV`"]
pub type WU4_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU4_EV`"]
pub struct WU4_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU4_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - WU7_EV"]
    #[inline(always)]
    pub fn wu7_ev(&self) -> WU7_EV_R {
        WU7_EV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WU6_EV"]
    #[inline(always)]
    pub fn wu6_ev(&self) -> WU6_EV_R {
        WU6_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - WU5_EV"]
    #[inline(always)]
    pub fn wu5_ev(&self) -> WU5_EV_R {
        WU5_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - WU4_EV"]
    #[inline(always)]
    pub fn wu4_ev(&self) -> WU4_EV_R {
        WU4_EV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - WU7_EV"]
    #[inline(always)]
    pub fn wu7_ev(&mut self) -> WU7_EV_W {
        WU7_EV_W { w: self }
    }
    #[doc = "Bits 16:21 - WU6_EV"]
    #[inline(always)]
    pub fn wu6_ev(&mut self) -> WU6_EV_W {
        WU6_EV_W { w: self }
    }
    #[doc = "Bits 8:13 - WU5_EV"]
    #[inline(always)]
    pub fn wu5_ev(&mut self) -> WU5_EV_W {
        WU5_EV_W { w: self }
    }
    #[doc = "Bits 0:5 - WU4_EV"]
    #[inline(always)]
    pub fn wu4_ev(&mut self) -> WU4_EV_W {
        WU4_EV_W { w: self }
    }
}
