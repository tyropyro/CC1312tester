#[doc = "Reader of register PEEK10"]
pub type R = crate::R<u32, super::PEEK10>;
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0x01) != 0)
    }
}
