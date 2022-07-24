#[doc = "Register `DTDAR` reader"]
pub struct R(crate::R<DTDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTDAR` writer"]
pub struct W(crate::W<DTDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTDAR_SPEC>;
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
impl From<crate::W<DTDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTDAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTDA` reader - desc DTDA"]
pub type DTDA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTDA` writer - desc DTDA"]
pub type DTDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTDAR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTDAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc DTDA"]
    #[inline(always)]
    pub fn dtda(&self) -> DTDA_R {
        DTDA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTDA"]
    #[inline(always)]
    pub fn dtda(&mut self) -> DTDA_W<0> {
        DTDA_W::new(self)
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
#[doc = "desc DTDAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtdar](index.html) module"]
pub struct DTDAR_SPEC;
impl crate::RegisterSpec for DTDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtdar::R](R) reader structure"]
impl crate::Readable for DTDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtdar::W](W) writer structure"]
impl crate::Writable for DTDAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTDAR to value 0xffff"]
impl crate::Resettable for DTDAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
