#[doc = "Register `PDBSETCLR` reader"]
pub struct R(crate::R<PDBSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDBSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDBSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDBSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDBSETCLR` writer"]
pub struct W(crate::W<PDBSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDBSETCLR_SPEC>;
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
impl From<crate::W<PDBSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDBSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDBCLR` reader - desc PDBCLR"]
pub type PDBCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDBCLR` writer - desc PDBCLR"]
pub type PDBCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDBSETCLR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PDBSET` reader - desc PDBSET"]
pub type PDBSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDBSET` writer - desc PDBSET"]
pub type PDBSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDBSETCLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc PDBCLR"]
    #[inline(always)]
    pub fn pdbclr(&self) -> PDBCLR_R {
        PDBCLR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc PDBSET"]
    #[inline(always)]
    pub fn pdbset(&self) -> PDBSET_R {
        PDBSET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc PDBCLR"]
    #[inline(always)]
    pub fn pdbclr(&mut self) -> PDBCLR_W<0> {
        PDBCLR_W::new(self)
    }
    #[doc = "Bits 16:23 - desc PDBSET"]
    #[inline(always)]
    pub fn pdbset(&mut self) -> PDBSET_W<16> {
        PDBSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PDBSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdbsetclr](index.html) module"]
pub struct PDBSETCLR_SPEC;
impl crate::RegisterSpec for PDBSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdbsetclr::R](R) reader structure"]
impl crate::Readable for PDBSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdbsetclr::W](W) writer structure"]
impl crate::Writable for PDBSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDBSETCLR to value 0"]
impl crate::Resettable for PDBSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
