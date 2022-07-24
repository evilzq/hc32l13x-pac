#[doc = "Register `DTR` reader"]
pub struct R(crate::R<DTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTR` writer"]
pub struct W(crate::W<DTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR_SPEC>;
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
impl From<crate::W<DTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOE` reader - desc MOE"]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - desc MOE"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - desc MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<12> {
        MOE_W::new(self)
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
#[doc = "desc DTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr](index.html) module"]
pub struct DTR_SPEC;
impl crate::RegisterSpec for DTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtr::R](R) reader structure"]
impl crate::Readable for DTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtr::W](W) writer structure"]
impl crate::Writable for DTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
