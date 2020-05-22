#[doc = "Reader of register SETBURST"]
pub type R = crate::R<u32, super::SETBURST>;
#[doc = "Writer for register SETBURST"]
pub type W = crate::W<u32, super::SETBURST>;
#[doc = "Register SETBURST `reset()`'s with value 0"]
impl crate::ResetValue for super::SETBURST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHNLS`"]
pub type CHNLS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHNLS`"]
pub struct CHNLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CHNLS"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHNLS"]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
}
