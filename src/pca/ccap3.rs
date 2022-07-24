#[doc = "Register `CCAP3` reader"]
pub struct R(crate::R<CCAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAP3` writer"]
pub struct W(crate::W<CCAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAP3_SPEC>;
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
impl From<crate::W<CCAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCAP3` reader - desc CCAP3"]
pub type CCAP3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCAP3` writer - desc CCAP3"]
pub type CCAP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCAP3_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAP3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc CCAP3"]
    #[inline(always)]
    pub fn ccap3(&self) -> CCAP3_R {
        CCAP3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCAP3"]
    #[inline(always)]
    pub fn ccap3(&mut self) -> CCAP3_W<0> {
        CCAP3_W::new(self)
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
#[doc = "desc CCAP3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccap3](index.html) module"]
pub struct CCAP3_SPEC;
impl crate::RegisterSpec for CCAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccap3::R](R) reader structure"]
impl crate::Readable for CCAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccap3::W](W) writer structure"]
impl crate::Writable for CCAP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCAP3 to value 0"]
impl crate::Resettable for CCAP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
