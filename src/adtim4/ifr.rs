#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMAF` reader - desc CMAF"]
pub type CMAF_R = crate::BitReader<bool>;
#[doc = "Field `CMBF` reader - desc CMBF"]
pub type CMBF_R = crate::BitReader<bool>;
#[doc = "Field `CMCF` reader - desc CMCF"]
pub type CMCF_R = crate::BitReader<bool>;
#[doc = "Field `CMDF` reader - desc CMDF"]
pub type CMDF_R = crate::BitReader<bool>;
#[doc = "Field `OVFF` reader - desc OVFF"]
pub type OVFF_R = crate::BitReader<bool>;
#[doc = "Field `UDFF` reader - desc UDFF"]
pub type UDFF_R = crate::BitReader<bool>;
#[doc = "Field `DTEF` reader - desc DTEF"]
pub type DTEF_R = crate::BitReader<bool>;
#[doc = "Field `SAMLF` reader - desc SAMLF"]
pub type SAMLF_R = crate::BitReader<bool>;
#[doc = "Field `SAMHF` reader - desc SAMHF"]
pub type SAMHF_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
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
    #[doc = "Bit 14 - desc SAMLF"]
    #[inline(always)]
    pub fn samlf(&self) -> SAMLF_R {
        SAMLF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SAMHF"]
    #[inline(always)]
    pub fn samhf(&self) -> SAMHF_R {
        SAMHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc IFR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
