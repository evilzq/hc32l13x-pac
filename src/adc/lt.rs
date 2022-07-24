#[doc = "Register `LT` reader"]
pub struct R(crate::R<LT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LT` writer"]
pub struct W(crate::W<LT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LT_SPEC>;
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
impl From<crate::W<LT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` reader - desc LT"]
pub type LT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LT` writer - desc LT"]
pub type LT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LT_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - desc LT"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc LT"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
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
#[doc = "desc LT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lt](index.html) module"]
pub struct LT_SPEC;
impl crate::RegisterSpec for LT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lt::R](R) reader structure"]
impl crate::Readable for LT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lt::W](W) writer structure"]
impl crate::Writable for LT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LT to value 0"]
impl crate::Resettable for LT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
