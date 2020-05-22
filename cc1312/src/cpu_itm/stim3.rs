#[doc = "Reader of register STIM3"]
pub type R = crate::R<u32, super::STIM3>;
#[doc = "Writer for register STIM3"]
pub type W = crate::W<u32, super::STIM3>;
#[doc = "Register STIM3 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM3`"]
pub type STIM3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM3`"]
pub struct STIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM3"]
    #[inline(always)]
    pub fn stim3(&self) -> STIM3_R {
        STIM3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM3"]
    #[inline(always)]
    pub fn stim3(&mut self) -> STIM3_W {
        STIM3_W { w: self }
    }
}
