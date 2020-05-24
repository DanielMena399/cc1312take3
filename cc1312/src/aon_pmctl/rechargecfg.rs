#[doc = "Reader of register RECHARGECFG"]
pub type R = crate::R<u32, super::RECHARGECFG>;
#[doc = "Writer for register RECHARGECFG"]
pub type W = crate::W<u32, super::RECHARGECFG>;
#[doc = "Register RECHARGECFG `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::RECHARGECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `C2`"]
pub type C2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C2`"]
pub struct C2_W<'a> {
    w: &'a mut W,
}
impl<'a> C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `C1`"]
pub type C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C1`"]
pub struct C1_W<'a> {
    w: &'a mut W,
}
impl<'a> C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_PER_M`"]
pub type MAX_PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_PER_M`"]
pub struct MAX_PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MAX_PER_E`"]
pub type MAX_PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_PER_E`"]
pub struct MAX_PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PER_M`"]
pub type PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_M`"]
pub struct PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PER_E`"]
pub type PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_E`"]
pub struct PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 20:23 - C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 11:15 - MAX_PER_M"]
    #[inline(always)]
    pub fn max_per_m(&self) -> MAX_PER_M_R {
        MAX_PER_M_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - MAX_PER_E"]
    #[inline(always)]
    pub fn max_per_e(&self) -> MAX_PER_E_R {
        MAX_PER_E_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - PER_M"]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - PER_E"]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 20:23 - C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W {
        C2_W { w: self }
    }
    #[doc = "Bits 16:19 - C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W {
        C1_W { w: self }
    }
    #[doc = "Bits 11:15 - MAX_PER_M"]
    #[inline(always)]
    pub fn max_per_m(&mut self) -> MAX_PER_M_W {
        MAX_PER_M_W { w: self }
    }
    #[doc = "Bits 8:10 - MAX_PER_E"]
    #[inline(always)]
    pub fn max_per_e(&mut self) -> MAX_PER_E_W {
        MAX_PER_E_W { w: self }
    }
    #[doc = "Bits 3:7 - PER_M"]
    #[inline(always)]
    pub fn per_m(&mut self) -> PER_M_W {
        PER_M_W { w: self }
    }
    #[doc = "Bits 0:2 - PER_E"]
    #[inline(always)]
    pub fn per_e(&mut self) -> PER_E_W {
        PER_E_W { w: self }
    }
}
