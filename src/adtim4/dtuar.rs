#[doc = "Register `DTUAR` reader"]
pub struct R(crate::R<DTUAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTUAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTUAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTUAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTUAR` writer"]
pub struct W(crate::W<DTUAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTUAR_SPEC>;
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
impl From<crate::W<DTUAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTUAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTUA` reader - desc DTUA"]
pub type DTUA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTUA` writer - desc DTUA"]
pub type DTUA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTUAR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTUAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc DTUA"]
    #[inline(always)]
    pub fn dtua(&self) -> DTUA_R {
        DTUA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTUA"]
    #[inline(always)]
    pub fn dtua(&mut self) -> DTUA_W<0> {
        DTUA_W::new(self)
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
#[doc = "desc DTUAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtuar](index.html) module"]
pub struct DTUAR_SPEC;
impl crate::RegisterSpec for DTUAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtuar::R](R) reader structure"]
impl crate::Readable for DTUAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtuar::W](W) writer structure"]
impl crate::Writable for DTUAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTUAR to value 0xffff"]
impl crate::Resettable for DTUAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
