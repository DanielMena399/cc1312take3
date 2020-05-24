#[doc = "Writer for register MASK4B3839"]
pub type W = crate::W<u32, super::MASK4B3839>;
#[doc = "Register MASK4B3839 `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK4B3839 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `M1H`"]
pub struct M1H_W<'a> {
    w: &'a mut W,
}
impl<'a> M1H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `D1H`"]
pub struct D1H_W<'a> {
    w: &'a mut W,
}
impl<'a> D1H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `M1L`"]
pub struct M1L_W<'a> {
    w: &'a mut W,
}
impl<'a> M1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `D1L`"]
pub struct D1L_W<'a> {
    w: &'a mut W,
}
impl<'a> D1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `M0H`"]
pub struct M0H_W<'a> {
    w: &'a mut W,
}
impl<'a> M0H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `D0H`"]
pub struct D0H_W<'a> {
    w: &'a mut W,
}
impl<'a> D0H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `M0L`"]
pub struct M0L_W<'a> {
    w: &'a mut W,
}
impl<'a> M0L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `D0L`"]
pub struct D0L_W<'a> {
    w: &'a mut W,
}
impl<'a> D0L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 28:31 - M1H"]
    #[inline(always)]
    pub fn m1h(&mut self) -> M1H_W {
        M1H_W { w: self }
    }
    #[doc = "Bits 24:27 - D1H"]
    #[inline(always)]
    pub fn d1h(&mut self) -> D1H_W {
        D1H_W { w: self }
    }
    #[doc = "Bits 20:23 - M1L"]
    #[inline(always)]
    pub fn m1l(&mut self) -> M1L_W {
        M1L_W { w: self }
    }
    #[doc = "Bits 16:19 - D1L"]
    #[inline(always)]
    pub fn d1l(&mut self) -> D1L_W {
        D1L_W { w: self }
    }
    #[doc = "Bits 12:15 - M0H"]
    #[inline(always)]
    pub fn m0h(&mut self) -> M0H_W {
        M0H_W { w: self }
    }
    #[doc = "Bits 8:11 - D0H"]
    #[inline(always)]
    pub fn d0h(&mut self) -> D0H_W {
        D0H_W { w: self }
    }
    #[doc = "Bits 4:7 - M0L"]
    #[inline(always)]
    pub fn m0l(&mut self) -> M0L_W {
        M0L_W { w: self }
    }
    #[doc = "Bits 0:3 - D0L"]
    #[inline(always)]
    pub fn d0l(&mut self) -> D0L_W {
        D0L_W { w: self }
    }
}
