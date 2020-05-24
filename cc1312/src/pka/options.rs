#[doc = "Reader of register OPTIONS"]
pub type R = crate::R<u32, super::OPTIONS>;
#[doc = "Reader of field `INT_MASKING`"]
pub type INT_MASKING_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROTECTION_OPTION`"]
pub type PROTECTION_OPTION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PROGRAM_RAM`"]
pub type PROGRAM_RAM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEQUENCER_CONFIGURATION`"]
pub type SEQUENCER_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKCP_CONFIGURATION`"]
pub type PKCP_CONFIGURATION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 11 - INT_MASKING"]
    #[inline(always)]
    pub fn int_masking(&self) -> INT_MASKING_R {
        INT_MASKING_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - PROTECTION_OPTION"]
    #[inline(always)]
    pub fn protection_option(&self) -> PROTECTION_OPTION_R {
        PROTECTION_OPTION_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - PROGRAM_RAM"]
    #[inline(always)]
    pub fn program_ram(&self) -> PROGRAM_RAM_R {
        PROGRAM_RAM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - SEQUENCER_CONFIGURATION"]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATION_R {
        SEQUENCER_CONFIGURATION_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - PKCP_CONFIGURATION"]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATION_R {
        PKCP_CONFIGURATION_R::new((self.bits & 0x03) as u8)
    }
}
