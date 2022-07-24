#[doc = "Register `CCR1B` reader"]
pub struct R(crate::R<CCR1B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR1B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR1B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR1B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR1B` writer"]
pub struct W(crate::W<CCR1B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR1B_SPEC>;
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
impl From<crate::W<CCR1B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR1B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1B` reader - desc CCR1B"]
pub type CCR1B_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR1B` writer - desc CCR1B"]
pub type CCR1B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR1B_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR1B_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    pub fn ccr1b(&self) -> CCR1B_R {
        CCR1B_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    pub fn ccr1b(&mut self) -> CCR1B_W<0> {
        CCR1B_W::new(self)
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
#[doc = "desc CCR1B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1b](index.html) module"]
pub struct CCR1B_SPEC;
impl crate::RegisterSpec for CCR1B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr1b::R](R) reader structure"]
impl crate::Readable for CCR1B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr1b::W](W) writer structure"]
impl crate::Writable for CCR1B_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR1B to value 0"]
impl crate::Resettable for CCR1B_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
