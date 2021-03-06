#[doc = "Reader of register NVIC_ICER1"]
pub type R = crate::R<u32, super::NVIC_ICER1>;
#[doc = "Writer for register NVIC_ICER1"]
pub type W = crate::W<u32, super::NVIC_ICER1>;
#[doc = "Register NVIC_ICER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLRENA37`"]
pub type CLRENA37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA37`"]
pub struct CLRENA37_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA37_W<'a> {
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
#[doc = "Reader of field `CLRENA36`"]
pub type CLRENA36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA36`"]
pub struct CLRENA36_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA36_W<'a> {
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
#[doc = "Reader of field `CLRENA35`"]
pub type CLRENA35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA35`"]
pub struct CLRENA35_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA35_W<'a> {
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
#[doc = "Reader of field `CLRENA34`"]
pub type CLRENA34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA34`"]
pub struct CLRENA34_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA34_W<'a> {
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
#[doc = "Reader of field `CLRENA33`"]
pub type CLRENA33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA33`"]
pub struct CLRENA33_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA33_W<'a> {
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
#[doc = "Reader of field `CLRENA32`"]
pub type CLRENA32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRENA32`"]
pub struct CLRENA32_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA32_W<'a> {
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
    #[doc = "Bit 5 - CLRENA37"]
    #[inline(always)]
    pub fn clrena37(&self) -> CLRENA37_R {
        CLRENA37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLRENA36"]
    #[inline(always)]
    pub fn clrena36(&self) -> CLRENA36_R {
        CLRENA36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLRENA35"]
    #[inline(always)]
    pub fn clrena35(&self) -> CLRENA35_R {
        CLRENA35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLRENA34"]
    #[inline(always)]
    pub fn clrena34(&self) -> CLRENA34_R {
        CLRENA34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLRENA33"]
    #[inline(always)]
    pub fn clrena33(&self) -> CLRENA33_R {
        CLRENA33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CLRENA32"]
    #[inline(always)]
    pub fn clrena32(&self) -> CLRENA32_R {
        CLRENA32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CLRENA37"]
    #[inline(always)]
    pub fn clrena37(&mut self) -> CLRENA37_W {
        CLRENA37_W { w: self }
    }
    #[doc = "Bit 4 - CLRENA36"]
    #[inline(always)]
    pub fn clrena36(&mut self) -> CLRENA36_W {
        CLRENA36_W { w: self }
    }
    #[doc = "Bit 3 - CLRENA35"]
    #[inline(always)]
    pub fn clrena35(&mut self) -> CLRENA35_W {
        CLRENA35_W { w: self }
    }
    #[doc = "Bit 2 - CLRENA34"]
    #[inline(always)]
    pub fn clrena34(&mut self) -> CLRENA34_W {
        CLRENA34_W { w: self }
    }
    #[doc = "Bit 1 - CLRENA33"]
    #[inline(always)]
    pub fn clrena33(&mut self) -> CLRENA33_W {
        CLRENA33_W { w: self }
    }
    #[doc = "Bit 0 - CLRENA32"]
    #[inline(always)]
    pub fn clrena32(&mut self) -> CLRENA32_W {
        CLRENA32_W { w: self }
    }
}
