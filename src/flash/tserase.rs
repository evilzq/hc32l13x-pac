#[doc = "Register `TSERASE` reader"]
pub struct R(crate::R<TSERASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSERASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSERASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSERASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSERASE` writer"]
pub struct W(crate::W<TSERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSERASE_SPEC>;
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
impl From<crate::W<TSERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSERASE` reader - desc TSERASE"]
pub type TSERASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSERASE` writer - desc TSERASE"]
pub type TSERASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSERASE_SPEC, u32, u32, 18, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSERASE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:17 - desc TSERASE"]
    #[inline(always)]
    pub fn tserase(&self) -> TSERASE_R {
        TSERASE_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - desc TSERASE"]
    #[inline(always)]
    pub fn tserase(&mut self) -> TSERASE_W<0> {
        TSERASE_W::new(self)
    }
    #[doc = "Bits 18:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<18> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TSERASE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tserase](index.html) module"]
pub struct TSERASE_SPEC;
impl crate::RegisterSpec for TSERASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tserase::R](R) reader structure"]
impl crate::Readable for TSERASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tserase::W](W) writer structure"]
impl crate::Writable for TSERASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSERASE to value 0x4650"]
impl crate::Resettable for TSERASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4650
    }
}
