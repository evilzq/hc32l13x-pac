#[doc = "Register `ADTR` reader"]
pub struct R(crate::R<ADTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADTR` writer"]
pub struct W(crate::W<ADTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADTR_SPEC>;
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
impl From<crate::W<ADTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEVE` reader - desc UEVE"]
pub type UEVE_R = crate::BitReader<bool>;
#[doc = "Field `UEVE` writer - desc UEVE"]
pub type UEVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTR_SPEC, bool, O>;
#[doc = "Field `CMA0E` reader - desc CMA0E"]
pub type CMA0E_R = crate::BitReader<bool>;
#[doc = "Field `CMA0E` writer - desc CMA0E"]
pub type CMA0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTR_SPEC, bool, O>;
#[doc = "Field `CMB0E` reader - desc CMB0E"]
pub type CMB0E_R = crate::BitReader<bool>;
#[doc = "Field `CMB0E` writer - desc CMB0E"]
pub type CMB0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTR_SPEC, bool, O>;
#[doc = "Field `ADTE` reader - desc ADTE"]
pub type ADTE_R = crate::BitReader<bool>;
#[doc = "Field `ADTE` writer - desc ADTE"]
pub type ADTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    pub fn ueve(&self) -> UEVE_R {
        UEVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMA0E"]
    #[inline(always)]
    pub fn cma0e(&self) -> CMA0E_R {
        CMA0E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CMB0E"]
    #[inline(always)]
    pub fn cmb0e(&self) -> CMB0E_R {
        CMB0E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    pub fn adte(&self) -> ADTE_R {
        ADTE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UEVE"]
    #[inline(always)]
    pub fn ueve(&mut self) -> UEVE_W<0> {
        UEVE_W::new(self)
    }
    #[doc = "Bit 1 - desc CMA0E"]
    #[inline(always)]
    pub fn cma0e(&mut self) -> CMA0E_W<1> {
        CMA0E_W::new(self)
    }
    #[doc = "Bit 4 - desc CMB0E"]
    #[inline(always)]
    pub fn cmb0e(&mut self) -> CMB0E_W<4> {
        CMB0E_W::new(self)
    }
    #[doc = "Bit 7 - desc ADTE"]
    #[inline(always)]
    pub fn adte(&mut self) -> ADTE_W<7> {
        ADTE_W::new(self)
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
#[doc = "desc ADTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adtr](index.html) module"]
pub struct ADTR_SPEC;
impl crate::RegisterSpec for ADTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adtr::R](R) reader structure"]
impl crate::Readable for ADTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adtr::W](W) writer structure"]
impl crate::Writable for ADTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADTR to value 0"]
impl crate::Resettable for ADTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
