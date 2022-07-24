#[doc = "Register `TPGS` reader"]
pub struct R(crate::R<TPGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPGS` writer"]
pub struct W(crate::W<TPGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPGS_SPEC>;
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
impl From<crate::W<TPGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPGS` reader - desc TPGS"]
pub type TPGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPGS` writer - desc TPGS"]
pub type TPGS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPGS_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPGS_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - desc TPGS"]
    #[inline(always)]
    pub fn tpgs(&self) -> TPGS_R {
        TPGS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc TPGS"]
    #[inline(always)]
    pub fn tpgs(&mut self) -> TPGS_W<0> {
        TPGS_W::new(self)
    }
    #[doc = "Bits 8:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<8> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TPGS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpgs](index.html) module"]
pub struct TPGS_SPEC;
impl crate::RegisterSpec for TPGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpgs::R](R) reader structure"]
impl crate::Readable for TPGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpgs::W](W) writer structure"]
impl crate::Writable for TPGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPGS to value 0x17"]
impl crate::Resettable for TPGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
