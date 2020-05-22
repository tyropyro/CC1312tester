#[doc = "Reader of register STIM18"]
pub type R = crate::R<u32, super::STIM18>;
#[doc = "Writer for register STIM18"]
pub type W = crate::W<u32, super::STIM18>;
#[doc = "Register STIM18 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM18`"]
pub type STIM18_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM18`"]
pub struct STIM18_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM18"]
    #[inline(always)]
    pub fn stim18(&self) -> STIM18_R {
        STIM18_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM18"]
    #[inline(always)]
    pub fn stim18(&mut self) -> STIM18_W {
        STIM18_W { w: self }
    }
}