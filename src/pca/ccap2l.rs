#[doc = "Register `CCAP2L` reader"]
pub struct R(crate::R<CCAP2L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAP2L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAP2L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAP2L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAP2L` writer"]
pub struct W(crate::W<CCAP2L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAP2L_SPEC>;
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
impl From<crate::W<CCAP2L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAP2L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCAP2` reader - desc CCAP2"]
pub type CCAP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCAP2` writer - desc CCAP2"]
pub type CCAP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCAP2L_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAP2L_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc CCAP2"]
    #[inline(always)]
    pub fn ccap2(&self) -> CCAP2_R {
        CCAP2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc CCAP2"]
    #[inline(always)]
    pub fn ccap2(&mut self) -> CCAP2_W<0> {
        CCAP2_W::new(self)
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
#[doc = "desc CCAP2L\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccap2l](index.html) module"]
pub struct CCAP2L_SPEC;
impl crate::RegisterSpec for CCAP2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccap2l::R](R) reader structure"]
impl crate::Readable for CCAP2L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccap2l::W](W) writer structure"]
impl crate::Writable for CCAP2L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCAP2L to value 0"]
impl crate::Resettable for CCAP2L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
