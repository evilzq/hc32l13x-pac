#[doc = "Register `TSRCV` reader"]
pub struct R(crate::R<TSRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSRCV` writer"]
pub struct W(crate::W<TSRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSRCV_SPEC>;
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
impl From<crate::W<TSRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSRCV` reader - desc TSRCV"]
pub type TSRCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSRCV` writer - desc TSRCV"]
pub type TSRCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSRCV_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSRCV_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - desc TSRCV"]
    #[inline(always)]
    pub fn tsrcv(&self) -> TSRCV_R {
        TSRCV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TSRCV"]
    #[inline(always)]
    pub fn tsrcv(&mut self) -> TSRCV_W<0> {
        TSRCV_W::new(self)
    }
    #[doc = "Bits 12:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<12> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TSRCV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsrcv](index.html) module"]
pub struct TSRCV_SPEC;
impl crate::RegisterSpec for TSRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsrcv::R](R) reader structure"]
impl crate::Readable for TSRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsrcv::W](W) writer structure"]
impl crate::Writable for TSRCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSRCV to value 0xf0"]
impl crate::Resettable for TSRCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
