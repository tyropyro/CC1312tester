#[doc = "Reader of register STIM13"]
pub type R = crate::R<u32, super::STIM13>;
#[doc = "Writer for register STIM13"]
pub type W = crate::W<u32, super::STIM13>;
#[doc = "Register STIM13 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM13`"]
pub type STIM13_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM13`"]
pub struct STIM13_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM13"]
    #[inline(always)]
    pub fn stim13(&self) -> STIM13_R {
        STIM13_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM13"]
    #[inline(always)]
    pub fn stim13(&mut self) -> STIM13_W {
        STIM13_W { w: self }
    }
}
