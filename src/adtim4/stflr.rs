#[doc = "Register `STFLR` reader"]
pub struct R(crate::R<STFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STFLR` writer"]
pub struct W(crate::W<STFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STFLR_SPEC>;
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
impl From<crate::W<STFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMAF` reader - desc CMAF"]
pub type CMAF_R = crate::BitReader<bool>;
#[doc = "Field `CMAF` writer - desc CMAF"]
pub type CMAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMBF` reader - desc CMBF"]
pub type CMBF_R = crate::BitReader<bool>;
#[doc = "Field `CMBF` writer - desc CMBF"]
pub type CMBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMCF` reader - desc CMCF"]
pub type CMCF_R = crate::BitReader<bool>;
#[doc = "Field `CMCF` writer - desc CMCF"]
pub type CMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMDF` reader - desc CMDF"]
pub type CMDF_R = crate::BitReader<bool>;
#[doc = "Field `CMDF` writer - desc CMDF"]
pub type CMDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `OVFF` reader - desc OVFF"]
pub type OVFF_R = crate::BitReader<bool>;
#[doc = "Field `OVFF` writer - desc OVFF"]
pub type OVFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `UDFF` reader - desc UDFF"]
pub type UDFF_R = crate::BitReader<bool>;
#[doc = "Field `UDFF` writer - desc UDFF"]
pub type UDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `DTEF` reader - desc DTEF"]
pub type DTEF_R = crate::BitReader<bool>;
#[doc = "Field `DTEF` writer - desc DTEF"]
pub type DTEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMSAUF` reader - desc CMSAUF"]
pub type CMSAUF_R = crate::BitReader<bool>;
#[doc = "Field `CMSAUF` writer - desc CMSAUF"]
pub type CMSAUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMSADF` reader - desc CMSADF"]
pub type CMSADF_R = crate::BitReader<bool>;
#[doc = "Field `CMSADF` writer - desc CMSADF"]
pub type CMSADF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMSBUF` reader - desc CMSBUF"]
pub type CMSBUF_R = crate::BitReader<bool>;
#[doc = "Field `CMSBUF` writer - desc CMSBUF"]
pub type CMSBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `CMSBDF` reader - desc CMSBDF"]
pub type CMSBDF_R = crate::BitReader<bool>;
#[doc = "Field `CMSBDF` writer - desc CMSBDF"]
pub type CMSBDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STFLR_SPEC, bool, O>;
#[doc = "Field `VPERNUM` reader - desc VPERNUM"]
pub type VPERNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIRF` reader - desc DIRF"]
pub type DIRF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc CMAF"]
    #[inline(always)]
    pub fn cmaf(&self) -> CMAF_R {
        CMAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMBF"]
    #[inline(always)]
    pub fn cmbf(&self) -> CMBF_R {
        CMBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMCF"]
    #[inline(always)]
    pub fn cmcf(&self) -> CMCF_R {
        CMCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMDF"]
    #[inline(always)]
    pub fn cmdf(&self) -> CMDF_R {
        CMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&self) -> UDFF_R {
        UDFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DTEF"]
    #[inline(always)]
    pub fn dtef(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CMSAUF"]
    #[inline(always)]
    pub fn cmsauf(&self) -> CMSAUF_R {
        CMSAUF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CMSADF"]
    #[inline(always)]
    pub fn cmsadf(&self) -> CMSADF_R {
        CMSADF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CMSBUF"]
    #[inline(always)]
    pub fn cmsbuf(&self) -> CMSBUF_R {
        CMSBUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CMSBDF"]
    #[inline(always)]
    pub fn cmsbdf(&self) -> CMSBDF_R {
        CMSBDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 21:23 - desc VPERNUM"]
    #[inline(always)]
    pub fn vpernum(&self) -> VPERNUM_R {
        VPERNUM_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 31 - desc DIRF"]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMAF"]
    #[inline(always)]
    pub fn cmaf(&mut self) -> CMAF_W<0> {
        CMAF_W::new(self)
    }
    #[doc = "Bit 1 - desc CMBF"]
    #[inline(always)]
    pub fn cmbf(&mut self) -> CMBF_W<1> {
        CMBF_W::new(self)
    }
    #[doc = "Bit 2 - desc CMCF"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<2> {
        CMCF_W::new(self)
    }
    #[doc = "Bit 3 - desc CMDF"]
    #[inline(always)]
    pub fn cmdf(&mut self) -> CMDF_W<3> {
        CMDF_W::new(self)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&mut self) -> OVFF_W<6> {
        OVFF_W::new(self)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&mut self) -> UDFF_W<7> {
        UDFF_W::new(self)
    }
    #[doc = "Bit 8 - desc DTEF"]
    #[inline(always)]
    pub fn dtef(&mut self) -> DTEF_W<8> {
        DTEF_W::new(self)
    }
    #[doc = "Bit 9 - desc CMSAUF"]
    #[inline(always)]
    pub fn cmsauf(&mut self) -> CMSAUF_W<9> {
        CMSAUF_W::new(self)
    }
    #[doc = "Bit 10 - desc CMSADF"]
    #[inline(always)]
    pub fn cmsadf(&mut self) -> CMSADF_W<10> {
        CMSADF_W::new(self)
    }
    #[doc = "Bit 11 - desc CMSBUF"]
    #[inline(always)]
    pub fn cmsbuf(&mut self) -> CMSBUF_W<11> {
        CMSBUF_W::new(self)
    }
    #[doc = "Bit 12 - desc CMSBDF"]
    #[inline(always)]
    pub fn cmsbdf(&mut self) -> CMSBDF_W<12> {
        CMSBDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc STFLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stflr](index.html) module"]
pub struct STFLR_SPEC;
impl crate::RegisterSpec for STFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stflr::R](R) reader structure"]
impl crate::Readable for STFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stflr::W](W) writer structure"]
impl crate::Writable for STFLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STFLR to value 0"]
impl crate::Resettable for STFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
