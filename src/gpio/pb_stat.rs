#[doc = "Register `PB_STAT` reader"]
pub struct R(crate::R<PB_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PB00` reader - desc PB00"]
pub type PB00_R = crate::BitReader<bool>;
#[doc = "Field `PB01` reader - desc PB01"]
pub type PB01_R = crate::BitReader<bool>;
#[doc = "Field `PB02` reader - desc PB02"]
pub type PB02_R = crate::BitReader<bool>;
#[doc = "Field `PB03` reader - desc PB03"]
pub type PB03_R = crate::BitReader<bool>;
#[doc = "Field `PB04` reader - desc PB04"]
pub type PB04_R = crate::BitReader<bool>;
#[doc = "Field `PB05` reader - desc PB05"]
pub type PB05_R = crate::BitReader<bool>;
#[doc = "Field `PB06` reader - desc PB06"]
pub type PB06_R = crate::BitReader<bool>;
#[doc = "Field `PB07` reader - desc PB07"]
pub type PB07_R = crate::BitReader<bool>;
#[doc = "Field `PB08` reader - desc PB08"]
pub type PB08_R = crate::BitReader<bool>;
#[doc = "Field `PB09` reader - desc PB09"]
pub type PB09_R = crate::BitReader<bool>;
#[doc = "Field `PB10` reader - desc PB10"]
pub type PB10_R = crate::BitReader<bool>;
#[doc = "Field `PB11` reader - desc PB11"]
pub type PB11_R = crate::BitReader<bool>;
#[doc = "Field `PB12` reader - desc PB12"]
pub type PB12_R = crate::BitReader<bool>;
#[doc = "Field `PB13` reader - desc PB13"]
pub type PB13_R = crate::BitReader<bool>;
#[doc = "Field `PB14` reader - desc PB14"]
pub type PB14_R = crate::BitReader<bool>;
#[doc = "Field `PB15` reader - desc PB15"]
pub type PB15_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc PB00"]
    #[inline(always)]
    pub fn pb00(&self) -> PB00_R {
        PB00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PB01"]
    #[inline(always)]
    pub fn pb01(&self) -> PB01_R {
        PB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PB02"]
    #[inline(always)]
    pub fn pb02(&self) -> PB02_R {
        PB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PB03"]
    #[inline(always)]
    pub fn pb03(&self) -> PB03_R {
        PB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PB04"]
    #[inline(always)]
    pub fn pb04(&self) -> PB04_R {
        PB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PB05"]
    #[inline(always)]
    pub fn pb05(&self) -> PB05_R {
        PB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PB06"]
    #[inline(always)]
    pub fn pb06(&self) -> PB06_R {
        PB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PB07"]
    #[inline(always)]
    pub fn pb07(&self) -> PB07_R {
        PB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PB08"]
    #[inline(always)]
    pub fn pb08(&self) -> PB08_R {
        PB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PB09"]
    #[inline(always)]
    pub fn pb09(&self) -> PB09_R {
        PB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PB10"]
    #[inline(always)]
    pub fn pb10(&self) -> PB10_R {
        PB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PB11"]
    #[inline(always)]
    pub fn pb11(&self) -> PB11_R {
        PB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PB12"]
    #[inline(always)]
    pub fn pb12(&self) -> PB12_R {
        PB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PB13"]
    #[inline(always)]
    pub fn pb13(&self) -> PB13_R {
        PB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PB14"]
    #[inline(always)]
    pub fn pb14(&self) -> PB14_R {
        PB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PB15"]
    #[inline(always)]
    pub fn pb15(&self) -> PB15_R {
        PB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc PB_STAT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_stat](index.html) module"]
pub struct PB_STAT_SPEC;
impl crate::RegisterSpec for PB_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_stat::R](R) reader structure"]
impl crate::Readable for PB_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PB_STAT to value 0"]
impl crate::Resettable for PB_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
