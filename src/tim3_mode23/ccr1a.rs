#[doc = "Register `CCR1A` reader"]
pub struct R(crate::R<CCR1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR1A` writer"]
pub struct W(crate::W<CCR1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR1A_SPEC>;
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
impl From<crate::W<CCR1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1A` reader - desc CCR1A"]
pub type CCR1A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR1A` writer - desc CCR1A"]
pub type CCR1A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR1A_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1A_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    pub fn ccr1a(&self) -> CCR1A_R {
        CCR1A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    pub fn ccr1a(&mut self) -> CCR1A_W<0> {
        CCR1A_W::new(self)
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
#[doc = "desc CCR1A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1a](index.html) module"]
pub struct CCR1A_SPEC;
impl crate::RegisterSpec for CCR1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr1a::R](R) reader structure"]
impl crate::Readable for CCR1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr1a::W](W) writer structure"]
impl crate::Writable for CCR1A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR1A to value 0"]
impl crate::Resettable for CCR1A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
