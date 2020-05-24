#[doc = "Reader of register HASHIOBUFCTRL"]
pub type R = crate::R<u32, super::HASHIOBUFCTRL>;
#[doc = "Writer for register HASHIOBUFCTRL"]
pub type W = crate::W<u32, super::HASHIOBUFCTRL>;
#[doc = "Register HASHIOBUFCTRL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::HASHIOBUFCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `PAD_DMA_MESSAGE`"]
pub type PAD_DMA_MESSAGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD_DMA_MESSAGE`"]
pub struct PAD_DMA_MESSAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DMA_MESSAGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GET_DIGEST`"]
pub type GET_DIGEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GET_DIGEST`"]
pub struct GET_DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> GET_DIGEST_W<'a> {
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
#[doc = "Reader of field `PAD_MESSAGE`"]
pub type PAD_MESSAGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD_MESSAGE`"]
pub struct PAD_MESSAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_MESSAGE_W<'a> {
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
#[doc = "Reader of field `RFD_IN`"]
pub type RFD_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFD_IN`"]
pub struct RFD_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD_IN_W<'a> {
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
#[doc = "Reader of field `DATA_IN_AV`"]
pub type DATA_IN_AV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_IN_AV`"]
pub struct DATA_IN_AV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_AV_W<'a> {
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
#[doc = "Reader of field `OUTPUT_FULL`"]
pub type OUTPUT_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPUT_FULL`"]
pub struct OUTPUT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_FULL_W<'a> {
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
    #[doc = "Bit 7 - PAD_DMA_MESSAGE"]
    #[inline(always)]
    pub fn pad_dma_message(&self) -> PAD_DMA_MESSAGE_R {
        PAD_DMA_MESSAGE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GET_DIGEST"]
    #[inline(always)]
    pub fn get_digest(&self) -> GET_DIGEST_R {
        GET_DIGEST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PAD_MESSAGE"]
    #[inline(always)]
    pub fn pad_message(&self) -> PAD_MESSAGE_R {
        PAD_MESSAGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RFD_IN"]
    #[inline(always)]
    pub fn rfd_in(&self) -> RFD_IN_R {
        RFD_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DATA_IN_AV"]
    #[inline(always)]
    pub fn data_in_av(&self) -> DATA_IN_AV_R {
        DATA_IN_AV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OUTPUT_FULL"]
    #[inline(always)]
    pub fn output_full(&self) -> OUTPUT_FULL_R {
        OUTPUT_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - PAD_DMA_MESSAGE"]
    #[inline(always)]
    pub fn pad_dma_message(&mut self) -> PAD_DMA_MESSAGE_W {
        PAD_DMA_MESSAGE_W { w: self }
    }
    #[doc = "Bit 6 - GET_DIGEST"]
    #[inline(always)]
    pub fn get_digest(&mut self) -> GET_DIGEST_W {
        GET_DIGEST_W { w: self }
    }
    #[doc = "Bit 5 - PAD_MESSAGE"]
    #[inline(always)]
    pub fn pad_message(&mut self) -> PAD_MESSAGE_W {
        PAD_MESSAGE_W { w: self }
    }
    #[doc = "Bit 2 - RFD_IN"]
    #[inline(always)]
    pub fn rfd_in(&mut self) -> RFD_IN_W {
        RFD_IN_W { w: self }
    }
    #[doc = "Bit 1 - DATA_IN_AV"]
    #[inline(always)]
    pub fn data_in_av(&mut self) -> DATA_IN_AV_W {
        DATA_IN_AV_W { w: self }
    }
    #[doc = "Bit 0 - OUTPUT_FULL"]
    #[inline(always)]
    pub fn output_full(&mut self) -> OUTPUT_FULL_W {
        OUTPUT_FULL_W { w: self }
    }
}
