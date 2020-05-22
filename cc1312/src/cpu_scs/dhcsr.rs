#[doc = "Reader of register DHCSR"]
pub type R = crate::R<u32, super::DHCSR>;
#[doc = "Writer for register DHCSR"]
pub type W = crate::W<u32, super::DHCSR>;
#[doc = "Register DHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S_RESET_ST`"]
pub type S_RESET_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_RESET_ST`"]
pub struct S_RESET_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RESET_ST_W<'a> {
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
#[doc = "Reader of field `S_RETIRE_ST`"]
pub type S_RETIRE_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_RETIRE_ST`"]
pub struct S_RETIRE_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> S_RETIRE_ST_W<'a> {
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
#[doc = "Reader of field `S_LOCKUP`"]
pub type S_LOCKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_LOCKUP`"]
pub struct S_LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_LOCKUP_W<'a> {
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
#[doc = "Reader of field `S_SLEEP`"]
pub type S_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_SLEEP`"]
pub struct S_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> S_SLEEP_W<'a> {
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
#[doc = "Reader of field `S_HALT`"]
pub type S_HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_HALT`"]
pub struct S_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> S_HALT_W<'a> {
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
#[doc = "Reader of field `S_REGRDY`"]
pub type S_REGRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_REGRDY`"]
pub struct S_REGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> S_REGRDY_W<'a> {
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
#[doc = "Reader of field `C_SNAPSTALL`"]
pub type C_SNAPSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_SNAPSTALL`"]
pub struct C_SNAPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> C_SNAPSTALL_W<'a> {
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
#[doc = "Reader of field `C_MASKINTS`"]
pub type C_MASKINTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_MASKINTS`"]
pub struct C_MASKINTS_W<'a> {
    w: &'a mut W,
}
impl<'a> C_MASKINTS_W<'a> {
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
#[doc = "Reader of field `C_STEP`"]
pub type C_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_STEP`"]
pub struct C_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> C_STEP_W<'a> {
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
#[doc = "Reader of field `C_HALT`"]
pub type C_HALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_HALT`"]
pub struct C_HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> C_HALT_W<'a> {
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
#[doc = "Reader of field `C_DEBUGEN`"]
pub type C_DEBUGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_DEBUGEN`"]
pub struct C_DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DEBUGEN_W<'a> {
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
    #[doc = "Bit 25 - S_RESET_ST"]
    #[inline(always)]
    pub fn s_reset_st(&self) -> S_RESET_ST_R {
        S_RESET_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - S_RETIRE_ST"]
    #[inline(always)]
    pub fn s_retire_st(&self) -> S_RETIRE_ST_R {
        S_RETIRE_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - S_LOCKUP"]
    #[inline(always)]
    pub fn s_lockup(&self) -> S_LOCKUP_R {
        S_LOCKUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - S_SLEEP"]
    #[inline(always)]
    pub fn s_sleep(&self) -> S_SLEEP_R {
        S_SLEEP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - S_HALT"]
    #[inline(always)]
    pub fn s_halt(&self) -> S_HALT_R {
        S_HALT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S_REGRDY"]
    #[inline(always)]
    pub fn s_regrdy(&self) -> S_REGRDY_R {
        S_REGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - C_SNAPSTALL"]
    #[inline(always)]
    pub fn c_snapstall(&self) -> C_SNAPSTALL_R {
        C_SNAPSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - C_MASKINTS"]
    #[inline(always)]
    pub fn c_maskints(&self) -> C_MASKINTS_R {
        C_MASKINTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - C_STEP"]
    #[inline(always)]
    pub fn c_step(&self) -> C_STEP_R {
        C_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - C_HALT"]
    #[inline(always)]
    pub fn c_halt(&self) -> C_HALT_R {
        C_HALT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - C_DEBUGEN"]
    #[inline(always)]
    pub fn c_debugen(&self) -> C_DEBUGEN_R {
        C_DEBUGEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - S_RESET_ST"]
    #[inline(always)]
    pub fn s_reset_st(&mut self) -> S_RESET_ST_W {
        S_RESET_ST_W { w: self }
    }
    #[doc = "Bit 24 - S_RETIRE_ST"]
    #[inline(always)]
    pub fn s_retire_st(&mut self) -> S_RETIRE_ST_W {
        S_RETIRE_ST_W { w: self }
    }
    #[doc = "Bit 19 - S_LOCKUP"]
    #[inline(always)]
    pub fn s_lockup(&mut self) -> S_LOCKUP_W {
        S_LOCKUP_W { w: self }
    }
    #[doc = "Bit 18 - S_SLEEP"]
    #[inline(always)]
    pub fn s_sleep(&mut self) -> S_SLEEP_W {
        S_SLEEP_W { w: self }
    }
    #[doc = "Bit 17 - S_HALT"]
    #[inline(always)]
    pub fn s_halt(&mut self) -> S_HALT_W {
        S_HALT_W { w: self }
    }
    #[doc = "Bit 16 - S_REGRDY"]
    #[inline(always)]
    pub fn s_regrdy(&mut self) -> S_REGRDY_W {
        S_REGRDY_W { w: self }
    }
    #[doc = "Bit 5 - C_SNAPSTALL"]
    #[inline(always)]
    pub fn c_snapstall(&mut self) -> C_SNAPSTALL_W {
        C_SNAPSTALL_W { w: self }
    }
    #[doc = "Bit 3 - C_MASKINTS"]
    #[inline(always)]
    pub fn c_maskints(&mut self) -> C_MASKINTS_W {
        C_MASKINTS_W { w: self }
    }
    #[doc = "Bit 2 - C_STEP"]
    #[inline(always)]
    pub fn c_step(&mut self) -> C_STEP_W {
        C_STEP_W { w: self }
    }
    #[doc = "Bit 1 - C_HALT"]
    #[inline(always)]
    pub fn c_halt(&mut self) -> C_HALT_W {
        C_HALT_W { w: self }
    }
    #[doc = "Bit 0 - C_DEBUGEN"]
    #[inline(always)]
    pub fn c_debugen(&mut self) -> C_DEBUGEN_W {
        C_DEBUGEN_W { w: self }
    }
}
