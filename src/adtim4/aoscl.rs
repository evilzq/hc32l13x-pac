#[doc = "Register `AOSCL` writer"]
pub struct W(crate::W<AOSCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AOSCL_SPEC>;
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
impl From<crate::W<AOSCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AOSCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBRAKE` writer - desc FBRAKE"]
pub type FBRAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSCL_SPEC, bool, O>;
#[doc = "Field `FSAME` writer - desc FSAME"]
pub type FSAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSCL_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOSCL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc FBRAKE"]
    #[inline(always)]
    pub fn fbrake(&mut self) -> FBRAKE_W<0> {
        FBRAKE_W::new(self)
    }
    #[doc = "Bit 1 - desc FSAME"]
    #[inline(always)]
    pub fn fsame(&mut self) -> FSAME_W<1> {
        FSAME_W::new(self)
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
#[doc = "desc AOSCL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aoscl](index.html) module"]
pub struct AOSCL_SPEC;
impl crate::RegisterSpec for AOSCL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aoscl::W](W) writer structure"]
impl crate::Writable for AOSCL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AOSCL to value 0"]
impl crate::Resettable for AOSCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
