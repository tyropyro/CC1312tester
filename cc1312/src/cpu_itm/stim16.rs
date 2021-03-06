#[doc = "Reader of register STIM16"]
pub type R = crate::R<u32, super::STIM16>;
#[doc = "Writer for register STIM16"]
pub type W = crate::W<u32, super::STIM16>;
#[doc = "Register STIM16 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM16`"]
pub type STIM16_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM16`"]
pub struct STIM16_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM16"]
    #[inline(always)]
    pub fn stim16(&self) -> STIM16_R {
        STIM16_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM16"]
    #[inline(always)]
    pub fn stim16(&mut self) -> STIM16_W {
        STIM16_W { w: self }
    }
}
