#[doc = "Register `SYSCTRL2` writer"]
pub struct W(crate::W<SYSCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL2_SPEC>;
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
impl From<crate::W<SYSCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTRL2` writer - desc SYSCTRL2"]
pub type SYSCTRL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTRL2_SPEC, u16, u16, 16, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTRL2_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:15 - desc SYSCTRL2"]
    #[inline(always)]
    pub fn sysctrl2(&mut self) -> SYSCTRL2_W<0> {
        SYSCTRL2_W::new(self)
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
#[doc = "desc SYSCTRL2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl2](index.html) module"]
pub struct SYSCTRL2_SPEC;
impl crate::RegisterSpec for SYSCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sysctrl2::W](W) writer structure"]
impl crate::Writable for SYSCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCTRL2 to value 0"]
impl crate::Resettable for SYSCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
