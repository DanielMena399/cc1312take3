#[doc = "Writer for register HASHMODE"]
pub type W = crate::W<u32, super::HASHMODE>;
#[doc = "Register HASHMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::HASHMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SHA384_MODE`"]
pub struct SHA384_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA384_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `SHA512_MODE`"]
pub struct SHA512_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA512_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `SHA224_MODE`"]
pub struct SHA224_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA224_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `SHA256_MODE`"]
pub struct SHA256_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA256_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `NEW_HASH`"]
pub struct NEW_HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> NEW_HASH_W<'a> {
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
impl W {
    #[doc = "Bit 6 - SHA384_MODE"]
    #[inline(always)]
    pub fn sha384_mode(&mut self) -> SHA384_MODE_W {
        SHA384_MODE_W { w: self }
    }
    #[doc = "Bit 5 - SHA512_MODE"]
    #[inline(always)]
    pub fn sha512_mode(&mut self) -> SHA512_MODE_W {
        SHA512_MODE_W { w: self }
    }
    #[doc = "Bit 4 - SHA224_MODE"]
    #[inline(always)]
    pub fn sha224_mode(&mut self) -> SHA224_MODE_W {
        SHA224_MODE_W { w: self }
    }
    #[doc = "Bit 3 - SHA256_MODE"]
    #[inline(always)]
    pub fn sha256_mode(&mut self) -> SHA256_MODE_W {
        SHA256_MODE_W { w: self }
    }
    #[doc = "Bit 0 - NEW_HASH"]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NEW_HASH_W {
        NEW_HASH_W { w: self }
    }
}
