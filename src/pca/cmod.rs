#[doc = "Register `CMOD` reader"]
pub struct R(crate::R<CMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMOD` writer"]
pub struct W(crate::W<CMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMOD_SPEC>;
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
impl From<crate::W<CMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFIE` reader - desc CFIE"]
pub type CFIE_R = crate::BitReader<bool>;
#[doc = "Field `CFIE` writer - desc CFIE"]
pub type CFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMOD_SPEC, bool, O>;
#[doc = "Field `CPS` reader - desc CPS"]
pub type CPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPS` writer - desc CPS"]
pub type CPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMOD_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDTE` reader - desc WDTE"]
pub type WDTE_R = crate::BitReader<bool>;
#[doc = "Field `WDTE` writer - desc WDTE"]
pub type WDTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMOD_SPEC, bool, O>;
#[doc = "Field `CIDL` reader - desc CIDL"]
pub type CIDL_R = crate::BitReader<bool>;
#[doc = "Field `CIDL` writer - desc CIDL"]
pub type CIDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMOD_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMOD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc CPS"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - desc WDTE"]
    #[inline(always)]
    pub fn wdte(&self) -> WDTE_R {
        WDTE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CIDL"]
    #[inline(always)]
    pub fn cidl(&self) -> CIDL_R {
        CIDL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CFIE"]
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W<0> {
        CFIE_W::new(self)
    }
    #[doc = "Bits 1:3 - desc CPS"]
    #[inline(always)]
    pub fn cps(&mut self) -> CPS_W<1> {
        CPS_W::new(self)
    }
    #[doc = "Bit 6 - desc WDTE"]
    #[inline(always)]
    pub fn wdte(&mut self) -> WDTE_W<6> {
        WDTE_W::new(self)
    }
    #[doc = "Bit 7 - desc CIDL"]
    #[inline(always)]
    pub fn cidl(&mut self) -> CIDL_W<7> {
        CIDL_W::new(self)
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
#[doc = "desc CMOD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmod](index.html) module"]
pub struct CMOD_SPEC;
impl crate::RegisterSpec for CMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmod::R](R) reader structure"]
impl crate::Readable for CMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmod::W](W) writer structure"]
impl crate::Writable for CMOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMOD to value 0"]
impl crate::Resettable for CMOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
