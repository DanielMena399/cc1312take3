#[doc = "Reader of register EFUSEPINS"]
pub type R = crate::R<u32, super::EFUSEPINS>;
#[doc = "Reader of field `EFC_SELF_TEST_DONE`"]
pub type EFC_SELF_TEST_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_SELF_TEST_ERROR`"]
pub type EFC_SELF_TEST_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_ECC_SELF_TEST_EN`"]
pub type SYS_ECC_SELF_TEST_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_INSTRUCTION_INFO`"]
pub type EFC_INSTRUCTION_INFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_INSTRUCTION_ERROR`"]
pub type EFC_INSTRUCTION_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_AUTOLOAD_ERROR`"]
pub type EFC_AUTOLOAD_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_ECC_OVERRIDE_EN`"]
pub type SYS_ECC_OVERRIDE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_READY`"]
pub type EFC_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFC_FCLRZ`"]
pub type EFC_FCLRZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_DIEID_AUTOLOAD_EN`"]
pub type SYS_DIEID_AUTOLOAD_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYS_REPAIR_EN`"]
pub type SYS_REPAIR_EN_R = crate::R<u8, u8>;
#[doc = "Reader of field `SYS_WS_READ_STATES`"]
pub type SYS_WS_READ_STATES_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 15 - EFC_SELF_TEST_DONE"]
    #[inline(always)]
    pub fn efc_self_test_done(&self) -> EFC_SELF_TEST_DONE_R {
        EFC_SELF_TEST_DONE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EFC_SELF_TEST_ERROR"]
    #[inline(always)]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERROR_R {
        EFC_SELF_TEST_ERROR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SYS_ECC_SELF_TEST_EN"]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_EN_R {
        SYS_ECC_SELF_TEST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EFC_INSTRUCTION_INFO"]
    #[inline(always)]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFO_R {
        EFC_INSTRUCTION_INFO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EFC_INSTRUCTION_ERROR"]
    #[inline(always)]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERROR_R {
        EFC_INSTRUCTION_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EFC_AUTOLOAD_ERROR"]
    #[inline(always)]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERROR_R {
        EFC_AUTOLOAD_ERROR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SYS_ECC_OVERRIDE_EN"]
    #[inline(always)]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_EN_R {
        SYS_ECC_OVERRIDE_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EFC_READY"]
    #[inline(always)]
    pub fn efc_ready(&self) -> EFC_READY_R {
        EFC_READY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EFC_FCLRZ"]
    #[inline(always)]
    pub fn efc_fclrz(&self) -> EFC_FCLRZ_R {
        EFC_FCLRZ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SYS_DIEID_AUTOLOAD_EN"]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_EN_R {
        SYS_DIEID_AUTOLOAD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SYS_REPAIR_EN"]
    #[inline(always)]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_EN_R {
        SYS_REPAIR_EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - SYS_WS_READ_STATES"]
    #[inline(always)]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATES_R {
        SYS_WS_READ_STATES_R::new((self.bits & 0x0f) as u8)
    }
}
