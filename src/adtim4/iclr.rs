#[doc = "Register `ICLR` writer"]
pub struct W(crate::W<ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICLR_SPEC>;
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
impl From<crate::W<ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMAC` writer - desc CMAC"]
pub type CMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CMBC` writer - desc CMBC"]
pub type CMBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CMCC` writer - desc CMCC"]
pub type CMCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `CMDC` writer - desc CMDC"]
pub type CMDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `OVFC` writer - desc OVFC"]
pub type OVFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `UDFC` writer - desc UDFC"]
pub type UDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `DTEC` writer - desc DTEC"]
pub type DTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `SAMLC` writer - desc SAMLC"]
pub type SAMLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `SAMHC` writer - desc SAMHC"]
pub type SAMHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - desc CMAC"]
    #[inline(always)]
    pub fn cmac(&mut self) -> CMAC_W<0> {
        CMAC_W::new(self)
    }
    #[doc = "Bit 1 - desc CMBC"]
    #[inline(always)]
    pub fn cmbc(&mut self) -> CMBC_W<1> {
        CMBC_W::new(self)
    }
    #[doc = "Bit 2 - desc CMCC"]
    #[inline(always)]
    pub fn cmcc(&mut self) -> CMCC_W<2> {
        CMCC_W::new(self)
    }
    #[doc = "Bit 3 - desc CMDC"]
    #[inline(always)]
    pub fn cmdc(&mut self) -> CMDC_W<3> {
        CMDC_W::new(self)
    }
    #[doc = "Bit 6 - desc OVFC"]
    #[inline(always)]
    pub fn ovfc(&mut self) -> OVFC_W<6> {
        OVFC_W::new(self)
    }
    #[doc = "Bit 7 - desc UDFC"]
    #[inline(always)]
    pub fn udfc(&mut self) -> UDFC_W<7> {
        UDFC_W::new(self)
    }
    #[doc = "Bit 8 - desc DTEC"]
    #[inline(always)]
    pub fn dtec(&mut self) -> DTEC_W<8> {
        DTEC_W::new(self)
    }
    #[doc = "Bit 14 - desc SAMLC"]
    #[inline(always)]
    pub fn samlc(&mut self) -> SAMLC_W<14> {
        SAMLC_W::new(self)
    }
    #[doc = "Bit 15 - desc SAMHC"]
    #[inline(always)]
    pub fn samhc(&mut self) -> SAMHC_W<15> {
        SAMHC_W::new(self)
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
#[doc = "desc ICLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iclr](index.html) module"]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iclr::W](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
