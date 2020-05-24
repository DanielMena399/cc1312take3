#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `NOCYCCNT`"]
pub type NOCYCCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCYCCNT`"]
pub struct NOCYCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCYCCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `NOPRFCNT`"]
pub type NOPRFCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOPRFCNT`"]
pub struct NOPRFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOPRFCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CYCEVTENA`"]
pub type CYCEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCEVTENA`"]
pub struct CYCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FOLDEVTENA`"]
pub type FOLDEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOLDEVTENA`"]
pub struct FOLDEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FOLDEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LSUEVTENA`"]
pub type LSUEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSUEVTENA`"]
pub struct LSUEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSUEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLEEPEVTENA`"]
pub type SLEEPEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPEVTENA`"]
pub struct SLEEPEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EXCEVTENA`"]
pub type EXCEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCEVTENA`"]
pub struct EXCEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CPIEVTENA`"]
pub type CPIEVTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPIEVTENA`"]
pub struct CPIEVTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIEVTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EXCTRCENA`"]
pub type EXCTRCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCTRCENA`"]
pub struct EXCTRCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCTRCENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCSAMPLEENA`"]
pub type PCSAMPLEENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSAMPLEENA`"]
pub struct PCSAMPLEENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSAMPLEENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SYNCTAP`"]
pub type SYNCTAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCTAP`"]
pub struct SYNCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCTAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CYCTAP`"]
pub type CYCTAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCTAP`"]
pub struct CYCTAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCTAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `POSTCNT`"]
pub type POSTCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTCNT`"]
pub struct POSTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `POSTPRESET`"]
pub type POSTPRESET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTPRESET`"]
pub struct POSTPRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTPRESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `CYCCNTENA`"]
pub type CYCCNTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CYCCNTENA`"]
pub struct CYCCNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCCNTENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - NOCYCCNT"]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NOCYCCNT_R {
        NOCYCCNT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NOPRFCNT"]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NOPRFCNT_R {
        NOPRFCNT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CYCEVTENA"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CYCEVTENA_R {
        CYCEVTENA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FOLDEVTENA"]
    #[inline(always)]
    pub fn foldevtena(&self) -> FOLDEVTENA_R {
        FOLDEVTENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LSUEVTENA"]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LSUEVTENA_R {
        LSUEVTENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SLEEPEVTENA"]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SLEEPEVTENA_R {
        SLEEPEVTENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - EXCEVTENA"]
    #[inline(always)]
    pub fn excevtena(&self) -> EXCEVTENA_R {
        EXCEVTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPIEVTENA"]
    #[inline(always)]
    pub fn cpievtena(&self) -> CPIEVTENA_R {
        CPIEVTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EXCTRCENA"]
    #[inline(always)]
    pub fn exctrcena(&self) -> EXCTRCENA_R {
        EXCTRCENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PCSAMPLEENA"]
    #[inline(always)]
    pub fn pcsampleena(&self) -> PCSAMPLEENA_R {
        PCSAMPLEENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - SYNCTAP"]
    #[inline(always)]
    pub fn synctap(&self) -> SYNCTAP_R {
        SYNCTAP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - CYCTAP"]
    #[inline(always)]
    pub fn cyctap(&self) -> CYCTAP_R {
        CYCTAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - POSTCNT"]
    #[inline(always)]
    pub fn postcnt(&self) -> POSTCNT_R {
        POSTCNT_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - POSTPRESET"]
    #[inline(always)]
    pub fn postpreset(&self) -> POSTPRESET_R {
        POSTPRESET_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - CYCCNTENA"]
    #[inline(always)]
    pub fn cyccntena(&self) -> CYCCNTENA_R {
        CYCCNTENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - NOCYCCNT"]
    #[inline(always)]
    pub fn nocyccnt(&mut self) -> NOCYCCNT_W {
        NOCYCCNT_W { w: self }
    }
    #[doc = "Bit 24 - NOPRFCNT"]
    #[inline(always)]
    pub fn noprfcnt(&mut self) -> NOPRFCNT_W {
        NOPRFCNT_W { w: self }
    }
    #[doc = "Bit 22 - CYCEVTENA"]
    #[inline(always)]
    pub fn cycevtena(&mut self) -> CYCEVTENA_W {
        CYCEVTENA_W { w: self }
    }
    #[doc = "Bit 21 - FOLDEVTENA"]
    #[inline(always)]
    pub fn foldevtena(&mut self) -> FOLDEVTENA_W {
        FOLDEVTENA_W { w: self }
    }
    #[doc = "Bit 20 - LSUEVTENA"]
    #[inline(always)]
    pub fn lsuevtena(&mut self) -> LSUEVTENA_W {
        LSUEVTENA_W { w: self }
    }
    #[doc = "Bit 19 - SLEEPEVTENA"]
    #[inline(always)]
    pub fn sleepevtena(&mut self) -> SLEEPEVTENA_W {
        SLEEPEVTENA_W { w: self }
    }
    #[doc = "Bit 18 - EXCEVTENA"]
    #[inline(always)]
    pub fn excevtena(&mut self) -> EXCEVTENA_W {
        EXCEVTENA_W { w: self }
    }
    #[doc = "Bit 17 - CPIEVTENA"]
    #[inline(always)]
    pub fn cpievtena(&mut self) -> CPIEVTENA_W {
        CPIEVTENA_W { w: self }
    }
    #[doc = "Bit 16 - EXCTRCENA"]
    #[inline(always)]
    pub fn exctrcena(&mut self) -> EXCTRCENA_W {
        EXCTRCENA_W { w: self }
    }
    #[doc = "Bit 12 - PCSAMPLEENA"]
    #[inline(always)]
    pub fn pcsampleena(&mut self) -> PCSAMPLEENA_W {
        PCSAMPLEENA_W { w: self }
    }
    #[doc = "Bits 10:11 - SYNCTAP"]
    #[inline(always)]
    pub fn synctap(&mut self) -> SYNCTAP_W {
        SYNCTAP_W { w: self }
    }
    #[doc = "Bit 9 - CYCTAP"]
    #[inline(always)]
    pub fn cyctap(&mut self) -> CYCTAP_W {
        CYCTAP_W { w: self }
    }
    #[doc = "Bits 5:8 - POSTCNT"]
    #[inline(always)]
    pub fn postcnt(&mut self) -> POSTCNT_W {
        POSTCNT_W { w: self }
    }
    #[doc = "Bits 1:4 - POSTPRESET"]
    #[inline(always)]
    pub fn postpreset(&mut self) -> POSTPRESET_W {
        POSTPRESET_W { w: self }
    }
    #[doc = "Bit 0 - CYCCNTENA"]
    #[inline(always)]
    pub fn cyccntena(&mut self) -> CYCCNTENA_W {
        CYCCNTENA_W { w: self }
    }
}
