#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCF0` writer - desc CCF0"]
pub type CCF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CCF1` writer - desc CCF1"]
pub type CCF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CCF2` writer - desc CCF2"]
pub type CCF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CCF3` writer - desc CCF3"]
pub type CCF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CCF4` writer - desc CCF4"]
pub type CCF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CF` writer - desc CF"]
pub type CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc CCF0"]
    #[inline(always)]
    pub fn ccf0(&mut self) -> CCF0_W<0> {
        CCF0_W::new(self)
    }
    #[doc = "Bit 1 - desc CCF1"]
    #[inline(always)]
    pub fn ccf1(&mut self) -> CCF1_W<1> {
        CCF1_W::new(self)
    }
    #[doc = "Bit 2 - desc CCF2"]
    #[inline(always)]
    pub fn ccf2(&mut self) -> CCF2_W<2> {
        CCF2_W::new(self)
    }
    #[doc = "Bit 3 - desc CCF3"]
    #[inline(always)]
    pub fn ccf3(&mut self) -> CCF3_W<3> {
        CCF3_W::new(self)
    }
    #[doc = "Bit 4 - desc CCF4"]
    #[inline(always)]
    pub fn ccf4(&mut self) -> CCF4_W<4> {
        CCF4_W::new(self)
    }
    #[doc = "Bit 7 - desc CF"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W<7> {
        CF_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ICLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLR to value 0x9f"]
impl crate::Resettable for ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9f
    }
}
