#[doc = "Register `RCH_CR` reader"]
pub struct R(crate::R<RCH_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCH_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCH_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCH_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCH_CR` writer"]
pub struct W(crate::W<RCH_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCH_CR_SPEC>;
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
impl From<crate::W<RCH_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCH_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCH_CR_SPEC, u16, u16, 11, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCH_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
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
#[doc = "desc RCH_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rch_cr](index.html) module"]
pub struct RCH_CR_SPEC;
impl crate::RegisterSpec for RCH_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rch_cr::R](R) reader structure"]
impl crate::Readable for RCH_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rch_cr::W](W) writer structure"]
impl crate::Writable for RCH_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCH_CR to value 0x0126"]
impl crate::Resettable for RCH_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0126
    }
}
