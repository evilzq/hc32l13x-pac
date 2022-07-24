#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGLIC` writer - desc SGLIC"]
pub type SGLIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `LTIC` writer - desc LTIC"]
pub type LTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `HTIC` writer - desc HTIC"]
pub type HTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `REGIC` writer - desc REGIC"]
pub type REGIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `SQRIC` writer - desc SQRIC"]
pub type SQRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `JQRIC` writer - desc JQRIC"]
pub type JQRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc SGLIC"]
    #[inline(always)]
    pub fn sglic(&mut self) -> SGLIC_W<0> {
        SGLIC_W::new(self)
    }
    #[doc = "Bit 1 - desc LTIC"]
    #[inline(always)]
    pub fn ltic(&mut self) -> LTIC_W<1> {
        LTIC_W::new(self)
    }
    #[doc = "Bit 2 - desc HTIC"]
    #[inline(always)]
    pub fn htic(&mut self) -> HTIC_W<2> {
        HTIC_W::new(self)
    }
    #[doc = "Bit 3 - desc REGIC"]
    #[inline(always)]
    pub fn regic(&mut self) -> REGIC_W<3> {
        REGIC_W::new(self)
    }
    #[doc = "Bit 4 - desc SQRIC"]
    #[inline(always)]
    pub fn sqric(&mut self) -> SQRIC_W<4> {
        SQRIC_W::new(self)
    }
    #[doc = "Bit 5 - desc JQRIC"]
    #[inline(always)]
    pub fn jqric(&mut self) -> JQRIC_W<5> {
        JQRIC_W::new(self)
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
#[doc = "desc ICR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0x3f"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
