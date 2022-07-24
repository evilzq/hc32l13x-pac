#[doc = "Register `TNVS` reader"]
pub struct R(crate::R<TNVS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TNVS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TNVS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TNVS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TNVS` writer"]
pub struct W(crate::W<TNVS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TNVS_SPEC>;
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
impl From<crate::W<TNVS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TNVS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TNVS` reader - desc TNVS"]
pub type TNVS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TNVS` writer - desc TNVS"]
pub type TNVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TNVS_SPEC, u16, u16, 9, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TNVS_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 0:8 - desc TNVS"]
    #[inline(always)]
    pub fn tnvs(&self) -> TNVS_R {
        TNVS_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TNVS"]
    #[inline(always)]
    pub fn tnvs(&mut self) -> TNVS_W<0> {
        TNVS_W::new(self)
    }
    #[doc = "Bits 9:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<9> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TNVS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tnvs](index.html) module"]
pub struct TNVS_SPEC;
impl crate::RegisterSpec for TNVS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tnvs::R](R) reader structure"]
impl crate::Readable for TNVS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tnvs::W](W) writer structure"]
impl crate::Writable for TNVS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TNVS to value 0x20"]
impl crate::Resettable for TNVS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
