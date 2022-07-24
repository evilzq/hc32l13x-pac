#[doc = "Register `GCMDR` reader"]
pub struct R(crate::R<GCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMDR` writer"]
pub struct W(crate::W<GCMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMDR_SPEC>;
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
impl From<crate::W<GCMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCMD` reader - desc GCMD"]
pub type GCMD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCMD` writer - desc GCMD"]
pub type GCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCMDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCMDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc GCMD"]
    #[inline(always)]
    pub fn gcmd(&self) -> GCMD_R {
        GCMD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc GCMD"]
    #[inline(always)]
    pub fn gcmd(&mut self) -> GCMD_W<0> {
        GCMD_W::new(self)
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
#[doc = "desc GCMDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmdr](index.html) module"]
pub struct GCMDR_SPEC;
impl crate::RegisterSpec for GCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmdr::R](R) reader structure"]
impl crate::Readable for GCMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmdr::W](W) writer structure"]
impl crate::Writable for GCMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCMDR to value 0xffff"]
impl crate::Resettable for GCMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
