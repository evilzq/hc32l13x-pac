#[doc = "Register `RST` writer"]
pub struct W(crate::W<RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_SPEC>;
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
impl From<crate::W<RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` writer - desc RST"]
pub type RST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RST_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - desc RST"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<0> {
        RST_W::new(self)
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
#[doc = "desc RST\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](index.html) module"]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rst::W](W) writer structure"]
impl crate::Writable for RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
