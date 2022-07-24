#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MDF` reader - desc MDF"]
pub type MDF_R = crate::BitReader<bool>;
#[doc = "Field `SSERR` reader - desc SSERR"]
pub type SSERR_R = crate::BitReader<bool>;
#[doc = "Field `SPIF` reader - desc SPIF"]
pub type SPIF_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc MDF"]
    #[inline(always)]
    pub fn mdf(&self) -> MDF_R {
        MDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SSERR"]
    #[inline(always)]
    pub fn sserr(&self) -> SSERR_R {
        SSERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SPIF"]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc STAT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
