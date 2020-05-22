#[doc = "Reader of register STIM6"]
pub type R = crate::R<u32, super::STIM6>;
#[doc = "Writer for register STIM6"]
pub type W = crate::W<u32, super::STIM6>;
#[doc = "Register STIM6 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM6`"]
pub type STIM6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM6`"]
pub struct STIM6_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM6"]
    #[inline(always)]
    pub fn stim6(&self) -> STIM6_R {
        STIM6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM6"]
    #[inline(always)]
    pub fn stim6(&mut self) -> STIM6_W {
        STIM6_W { w: self }
    }
}