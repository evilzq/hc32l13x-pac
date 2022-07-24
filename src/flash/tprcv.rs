#[doc = "Register `TPRCV` reader"]
pub struct R(crate::R<TPRCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPRCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPRCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPRCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPRCV` writer"]
pub struct W(crate::W<TPRCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPRCV_SPEC>;
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
impl From<crate::W<TPRCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPRCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPRCV` reader - desc TPRCV"]
pub type TPRCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPRCV` writer - desc TPRCV"]
pub type TPRCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPRCV_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPRCV_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - desc TPRCV"]
    #[inline(always)]
    pub fn tprcv(&self) -> TPRCV_R {
        TPRCV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc TPRCV"]
    #[inline(always)]
    pub fn tprcv(&mut self) -> TPRCV_W<0> {
        TPRCV_W::new(self)
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
#[doc = "desc TPRCV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tprcv](index.html) module"]
pub struct TPRCV_SPEC;
impl crate::RegisterSpec for TPRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tprcv::R](R) reader structure"]
impl crate::Readable for TPRCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tprcv::W](W) writer structure"]
impl crate::Writable for TPRCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPRCV to value 0x18"]
impl crate::Resettable for TPRCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
