#[doc = "Writer for register AESKEY3%s"]
pub type W = crate::W<u32, super::AESKEY3>;
#[doc = "Register AESKEY3%s `reset()`'s with value 0"]
impl crate::ResetValue for super::AESKEY3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AES_KEY3`"]
pub struct AES_KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AES_KEY3"]
    #[inline(always)]
    pub fn aes_key3(&mut self) -> AES_KEY3_W {
        AES_KEY3_W { w: self }
    }
}
