#[doc = "Reader of register STIM14"]
pub type R = crate::R<u32, super::STIM14>;
#[doc = "Writer for register STIM14"]
pub type W = crate::W<u32, super::STIM14>;
#[doc = "Register STIM14 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM14`"]
pub type STIM14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM14`"]
pub struct STIM14_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM14"]
    #[inline(always)]
    pub fn stim14(&self) -> STIM14_R {
        STIM14_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM14"]
    #[inline(always)]
    pub fn stim14(&mut self) -> STIM14_W {
        STIM14_W { w: self }
    }
}
