#[doc = "Register `ALMWEEK` reader"]
pub struct R(crate::R<ALMWEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMWEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMWEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMWEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMWEEK` writer"]
pub struct W(crate::W<ALMWEEK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMWEEK_SPEC>;
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
impl From<crate::W<ALMWEEK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMWEEK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMWEEK` reader - desc ALMWEEK"]
pub type ALMWEEK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMWEEK` writer - desc ALMWEEK"]
pub type ALMWEEK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMWEEK_SPEC, u8, u8, 7, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALMWEEK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - desc ALMWEEK"]
    #[inline(always)]
    pub fn almweek(&self) -> ALMWEEK_R {
        ALMWEEK_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc ALMWEEK"]
    #[inline(always)]
    pub fn almweek(&mut self) -> ALMWEEK_W<0> {
        ALMWEEK_W::new(self)
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
#[doc = "desc ALMWEEK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almweek](index.html) module"]
pub struct ALMWEEK_SPEC;
impl crate::RegisterSpec for ALMWEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almweek::R](R) reader structure"]
impl crate::Readable for ALMWEEK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almweek::W](W) writer structure"]
impl crate::Writable for ALMWEEK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMWEEK to value 0"]
impl crate::Resettable for ALMWEEK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
