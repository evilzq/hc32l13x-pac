#[doc = "Register `CCR0A` reader"]
pub struct R(crate::R<CCR0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR0A` writer"]
pub struct W(crate::W<CCR0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR0A_SPEC>;
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
impl From<crate::W<CCR0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR0A` reader - desc CCR0A"]
pub type CCR0A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR0A` writer - desc CCR0A"]
pub type CCR0A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR0A_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR0A_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc CCR0A"]
    #[inline(always)]
    pub fn ccr0a(&self) -> CCR0A_R {
        CCR0A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR0A"]
    #[inline(always)]
    pub fn ccr0a(&mut self) -> CCR0A_W<0> {
        CCR0A_W::new(self)
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
#[doc = "desc CCR0A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr0a](index.html) module"]
pub struct CCR0A_SPEC;
impl crate::RegisterSpec for CCR0A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr0a::R](R) reader structure"]
impl crate::Readable for CCR0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr0a::W](W) writer structure"]
impl crate::Writable for CCR0A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR0A to value 0"]
impl crate::Resettable for CCR0A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
