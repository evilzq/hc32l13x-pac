#[doc = "Register `TMRUN` reader"]
pub struct R(crate::R<TMRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMRUN` writer"]
pub struct W(crate::W<TMRUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMRUN_SPEC>;
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
impl From<crate::W<TMRUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMRUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TME` reader - desc TME"]
pub type TME_R = crate::BitReader<bool>;
#[doc = "Field `TME` writer - desc TME"]
pub type TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMRUN_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMRUN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TME"]
    #[inline(always)]
    pub fn tme(&mut self) -> TME_W<0> {
        TME_W::new(self)
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
#[doc = "desc TMRUN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmrun](index.html) module"]
pub struct TMRUN_SPEC;
impl crate::RegisterSpec for TMRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmrun::R](R) reader structure"]
impl crate::Readable for TMRUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmrun::W](W) writer structure"]
impl crate::Writable for TMRUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMRUN to value 0"]
impl crate::Resettable for TMRUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
