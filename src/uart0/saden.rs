#[doc = "Register `SADEN` reader"]
pub struct R(crate::R<SADEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADEN` writer"]
pub struct W(crate::W<SADEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADEN_SPEC>;
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
impl From<crate::W<SADEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADEN` reader - desc SADEN"]
pub type SADEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SADEN` writer - desc SADEN"]
pub type SADEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADEN_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SADEN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc SADEN"]
    #[inline(always)]
    pub fn saden(&self) -> SADEN_R {
        SADEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc SADEN"]
    #[inline(always)]
    pub fn saden(&mut self) -> SADEN_W<0> {
        SADEN_W::new(self)
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
#[doc = "desc SADEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saden](index.html) module"]
pub struct SADEN_SPEC;
impl crate::RegisterSpec for SADEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saden::R](R) reader structure"]
impl crate::Readable for SADEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saden::W](W) writer structure"]
impl crate::Writable for SADEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADEN to value 0"]
impl crate::Resettable for SADEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
