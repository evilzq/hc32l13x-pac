#[doc = "Register `PCAS` reader"]
pub struct R(crate::R<PCAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCAS` writer"]
pub struct W(crate::W<PCAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCAS_SPEC>;
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
impl From<crate::W<PCAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCA_CH0` reader - desc PCA_CH0"]
pub type PCA_CH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCA_CH0` writer - desc PCA_CH0"]
pub type PCA_CH0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCAS_SPEC, u8, u8, 3, O>;
#[doc = "Field `PCA_ECI` reader - desc PCA_ECI"]
pub type PCA_ECI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCA_ECI` writer - desc PCA_ECI"]
pub type PCA_ECI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCAS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCAS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc PCA_CH0"]
    #[inline(always)]
    pub fn pca_ch0(&self) -> PCA_CH0_R {
        PCA_CH0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc PCA_ECI"]
    #[inline(always)]
    pub fn pca_eci(&self) -> PCA_ECI_R {
        PCA_ECI_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PCA_CH0"]
    #[inline(always)]
    pub fn pca_ch0(&mut self) -> PCA_CH0_W<0> {
        PCA_CH0_W::new(self)
    }
    #[doc = "Bits 3:5 - desc PCA_ECI"]
    #[inline(always)]
    pub fn pca_eci(&mut self) -> PCA_ECI_W<3> {
        PCA_ECI_W::new(self)
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
#[doc = "desc PCAS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcas](index.html) module"]
pub struct PCAS_SPEC;
impl crate::RegisterSpec for PCAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcas::R](R) reader structure"]
impl crate::Readable for PCAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcas::W](W) writer structure"]
impl crate::Writable for PCAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCAS to value 0"]
impl crate::Resettable for PCAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
