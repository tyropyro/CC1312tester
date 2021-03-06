#[doc = "Reader of register STIM2"]
pub type R = crate::R<u32, super::STIM2>;
#[doc = "Writer for register STIM2"]
pub type W = crate::W<u32, super::STIM2>;
#[doc = "Register STIM2 `reset()`'s with value 0"]
impl crate::ResetValue for super::STIM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STIM2`"]
pub type STIM2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STIM2`"]
pub struct STIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - STIM2"]
    #[inline(always)]
    pub fn stim2(&self) -> STIM2_R {
        STIM2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - STIM2"]
    #[inline(always)]
    pub fn stim2(&mut self) -> STIM2_W {
        STIM2_W { w: self }
    }
}
