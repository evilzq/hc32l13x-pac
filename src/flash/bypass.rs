#[doc = "Register `BYPASS` writer"]
pub struct W(crate::W<BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPASS_SPEC>;
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
impl From<crate::W<BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYSEQ` writer - desc BYSEQ"]
pub type BYSEQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYPASS_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYPASS_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - desc BYSEQ"]
    #[inline(always)]
    pub fn byseq(&mut self) -> BYSEQ_W<0> {
        BYSEQ_W::new(self)
    }
    #[doc = "Bits 16:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<16> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc BYPASS\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypass](index.html) module"]
pub struct BYPASS_SPEC;
impl crate::RegisterSpec for BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bypass::W](W) writer structure"]
impl crate::Writable for BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYPASS to value 0"]
impl crate::Resettable for BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
