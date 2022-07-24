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
#[doc = "Field `UF` writer - desc UF"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `OV` writer - desc OV"]
pub type OV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TO` writer - desc TO"]
pub type TO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FE` writer - desc FE"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BB` writer - desc BB"]
pub type BB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `S0E` writer - desc S0E"]
pub type S0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `S1E` writer - desc S1E"]
pub type S1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc UF"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W<0> {
        UF_W::new(self)
    }
    #[doc = "Bit 1 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OV_W<1> {
        OV_W::new(self)
    }
    #[doc = "Bit 2 - desc TO"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<2> {
        TO_W::new(self)
    }
    #[doc = "Bit 3 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<3> {
        DIR_W::new(self)
    }
    #[doc = "Bit 4 - desc FE"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W<4> {
        FE_W::new(self)
    }
    #[doc = "Bit 5 - desc BB"]
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W<5> {
        BB_W::new(self)
    }
    #[doc = "Bit 6 - desc S0E"]
    #[inline(always)]
    pub fn s0e(&mut self) -> S0E_W<6> {
        S0E_W::new(self)
    }
    #[doc = "Bit 7 - desc S1E"]
    #[inline(always)]
    pub fn s1e(&mut self) -> S1E_W<7> {
        S1E_W::new(self)
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
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
