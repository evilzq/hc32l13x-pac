#[doc = "Register `CALCON` reader"]
pub struct R(crate::R<CALCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCON` writer"]
pub struct W(crate::W<CALCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCON_SPEC>;
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
impl From<crate::W<CALCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCNTVAL` reader - desc CCNTVAL"]
pub type CCNTVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCNTVAL` writer - desc CCNTVAL"]
pub type CCNTVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALCON_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc CCNTVAL"]
    #[inline(always)]
    pub fn ccntval(&self) -> CCNTVAL_R {
        CCNTVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc CCNTVAL"]
    #[inline(always)]
    pub fn ccntval(&mut self) -> CCNTVAL_W<0> {
        CCNTVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CALCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calcon](index.html) module"]
pub struct CALCON_SPEC;
impl crate::RegisterSpec for CALCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calcon::R](R) reader structure"]
impl crate::Readable for CALCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calcon::W](W) writer structure"]
impl crate::Writable for CALCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALCON to value 0xffff_ffff"]
impl crate::Resettable for CALCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
