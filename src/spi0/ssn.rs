#[doc = "Register `SSN` reader"]
pub struct R(crate::R<SSN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSN` writer"]
pub struct W(crate::W<SSN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSN_SPEC>;
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
impl From<crate::W<SSN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSN` reader - desc SSN"]
pub type SSN_R = crate::BitReader<bool>;
#[doc = "Field `SSN` writer - desc SSN"]
pub type SSN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSN_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SSN"]
    #[inline(always)]
    pub fn ssn(&self) -> SSN_R {
        SSN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSN"]
    #[inline(always)]
    pub fn ssn(&mut self) -> SSN_W<0> {
        SSN_W::new(self)
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
#[doc = "desc SSN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssn](index.html) module"]
pub struct SSN_SPEC;
impl crate::RegisterSpec for SSN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssn::R](R) reader structure"]
impl crate::Readable for SSN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssn::W](W) writer structure"]
impl crate::Writable for SSN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSN to value 0x01"]
impl crate::Resettable for SSN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
