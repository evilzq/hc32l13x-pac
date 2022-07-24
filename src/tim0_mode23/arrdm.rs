#[doc = "Register `ARRDM` reader"]
pub struct R(crate::R<ARRDM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARRDM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARRDM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARRDM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARRDM` writer"]
pub struct W(crate::W<ARRDM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARRDM_SPEC>;
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
impl From<crate::W<ARRDM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARRDM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARRDM` reader - desc ARRDM"]
pub type ARRDM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARRDM` writer - desc ARRDM"]
pub type ARRDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARRDM_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARRDM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc ARRDM"]
    #[inline(always)]
    pub fn arrdm(&self) -> ARRDM_R {
        ARRDM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc ARRDM"]
    #[inline(always)]
    pub fn arrdm(&mut self) -> ARRDM_W<0> {
        ARRDM_W::new(self)
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
#[doc = "desc ARRDM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arrdm](index.html) module"]
pub struct ARRDM_SPEC;
impl crate::RegisterSpec for ARRDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arrdm::R](R) reader structure"]
impl crate::Readable for ARRDM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arrdm::W](W) writer structure"]
impl crate::Writable for ARRDM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARRDM to value 0"]
impl crate::Resettable for ARRDM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
