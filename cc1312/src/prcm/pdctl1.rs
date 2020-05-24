#[doc = "Reader of register PDCTL1"]
pub type R = crate::R<u32, super::PDCTL1>;
#[doc = "Writer for register PDCTL1"]
pub type W = crate::W<u32, super::PDCTL1>;
#[doc = "Register PDCTL1 `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::PDCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "VIMS_MODE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VIMS_MODE_A {
    #[doc = "0: VIMS power domain is only powered when CPU power domain is powered."]
    CPU_DEP = 0,
    #[doc = "1: VIMS power domain is powered whenever the BUS power domain is powered."]
    BUS_DEP = 1,
    #[doc = "2: Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    BLOCKED = 2,
}
impl From<VIMS_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: VIMS_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VIMS_MODE`"]
pub type VIMS_MODE_R = crate::R<u8, VIMS_MODE_A>;
impl VIMS_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VIMS_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VIMS_MODE_A::CPU_DEP),
            1 => Val(VIMS_MODE_A::BUS_DEP),
            2 => Val(VIMS_MODE_A::BLOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPU_DEP`"]
    #[inline(always)]
    pub fn is_cpu_dep(&self) -> bool {
        *self == VIMS_MODE_A::CPU_DEP
    }
    #[doc = "Checks if the value of the field is `BUS_DEP`"]
    #[inline(always)]
    pub fn is_bus_dep(&self) -> bool {
        *self == VIMS_MODE_A::BUS_DEP
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == VIMS_MODE_A::BLOCKED
    }
}
#[doc = "Write proxy for field `VIMS_MODE`"]
pub struct VIMS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIMS_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VIMS power domain is only powered when CPU power domain is powered."]
    #[inline(always)]
    pub fn cpu_dep(self) -> &'a mut W {
        self.variant(VIMS_MODE_A::CPU_DEP)
    }
    #[doc = "VIMS power domain is powered whenever the BUS power domain is powered."]
    #[inline(always)]
    pub fn bus_dep(self) -> &'a mut W {
        self.variant(VIMS_MODE_A::BUS_DEP)
    }
    #[doc = "Block power up of VIMS power domain at next wake up. This mode only has effect when VIMS power domain is not powered. Used for Autonomous RF Core."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(VIMS_MODE_A::BLOCKED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `RFC_ON`"]
pub type RFC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_ON`"]
pub struct RFC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CPU_ON`"]
pub type CPU_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_ON`"]
pub struct CPU_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4 - VIMS_MODE"]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU_ON"]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - VIMS_MODE"]
    #[inline(always)]
    pub fn vims_mode(&mut self) -> VIMS_MODE_W {
        VIMS_MODE_W { w: self }
    }
    #[doc = "Bit 2 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&mut self) -> RFC_ON_W {
        RFC_ON_W { w: self }
    }
    #[doc = "Bit 1 - CPU_ON"]
    #[inline(always)]
    pub fn cpu_on(&mut self) -> CPU_ON_W {
        CPU_ON_W { w: self }
    }
}
