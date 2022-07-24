#[doc = "Register `EPWM` reader"]
pub struct R(crate::R<EPWM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPWM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPWM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPWM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPWM` writer"]
pub struct W(crate::W<EPWM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPWM_SPEC>;
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
impl From<crate::W<EPWM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPWM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPWM` reader - desc EPWM"]
pub type EPWM_R = crate::BitReader<bool>;
#[doc = "Field `EPWM` writer - desc EPWM"]
pub type EPWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPWM_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EPWM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc EPWM"]
    #[inline(always)]
    pub fn epwm(&self) -> EPWM_R {
        EPWM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EPWM"]
    #[inline(always)]
    pub fn epwm(&mut self) -> EPWM_W<0> {
        EPWM_W::new(self)
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
#[doc = "desc EPWM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epwm](index.html) module"]
pub struct EPWM_SPEC;
impl crate::RegisterSpec for EPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epwm::R](R) reader structure"]
impl crate::Readable for EPWM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epwm::W](W) writer structure"]
impl crate::Writable for EPWM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPWM to value 0"]
impl crate::Resettable for EPWM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
