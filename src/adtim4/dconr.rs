#[doc = "Register `DCONR` reader"]
pub struct R(crate::R<DCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCONR` writer"]
pub struct W(crate::W<DCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCONR_SPEC>;
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
impl From<crate::W<DCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCEN` reader - desc DTCEN"]
pub type DTCEN_R = crate::BitReader<bool>;
#[doc = "Field `DTCEN` writer - desc DTCEN"]
pub type DTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCONR_SPEC, bool, O>;
#[doc = "Field `SEPA` reader - desc SEPA"]
pub type SEPA_R = crate::BitReader<bool>;
#[doc = "Field `SEPA` writer - desc SEPA"]
pub type SEPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCONR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&self) -> DTCEN_R {
        DTCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&self) -> SEPA_R {
        SEPA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DTCEN"]
    #[inline(always)]
    pub fn dtcen(&mut self) -> DTCEN_W<0> {
        DTCEN_W::new(self)
    }
    #[doc = "Bit 8 - desc SEPA"]
    #[inline(always)]
    pub fn sepa(&mut self) -> SEPA_W<8> {
        SEPA_W::new(self)
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
#[doc = "desc DCONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dconr](index.html) module"]
pub struct DCONR_SPEC;
impl crate::RegisterSpec for DCONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dconr::R](R) reader structure"]
impl crate::Readable for DCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dconr::W](W) writer structure"]
impl crate::Writable for DCONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCONR to value 0"]
impl crate::Resettable for DCONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
