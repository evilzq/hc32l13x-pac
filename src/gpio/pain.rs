#[doc = "Register `PAIN` reader"]
pub struct R(crate::R<PAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PA00` reader - desc PA00"]
pub type PA00_R = crate::BitReader<bool>;
#[doc = "Field `PA01` reader - desc PA01"]
pub type PA01_R = crate::BitReader<bool>;
#[doc = "Field `PA02` reader - desc PA02"]
pub type PA02_R = crate::BitReader<bool>;
#[doc = "Field `PA03` reader - desc PA03"]
pub type PA03_R = crate::BitReader<bool>;
#[doc = "Field `PA04` reader - desc PA04"]
pub type PA04_R = crate::BitReader<bool>;
#[doc = "Field `PA05` reader - desc PA05"]
pub type PA05_R = crate::BitReader<bool>;
#[doc = "Field `PA06` reader - desc PA06"]
pub type PA06_R = crate::BitReader<bool>;
#[doc = "Field `PA07` reader - desc PA07"]
pub type PA07_R = crate::BitReader<bool>;
#[doc = "Field `PA08` reader - desc PA08"]
pub type PA08_R = crate::BitReader<bool>;
#[doc = "Field `PA09` reader - desc PA09"]
pub type PA09_R = crate::BitReader<bool>;
#[doc = "Field `PA10` reader - desc PA10"]
pub type PA10_R = crate::BitReader<bool>;
#[doc = "Field `PA11` reader - desc PA11"]
pub type PA11_R = crate::BitReader<bool>;
#[doc = "Field `PA12` reader - desc PA12"]
pub type PA12_R = crate::BitReader<bool>;
#[doc = "Field `PA13` reader - desc PA13"]
pub type PA13_R = crate::BitReader<bool>;
#[doc = "Field `PA14` reader - desc PA14"]
pub type PA14_R = crate::BitReader<bool>;
#[doc = "Field `PA15` reader - desc PA15"]
pub type PA15_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc PA00"]
    #[inline(always)]
    pub fn pa00(&self) -> PA00_R {
        PA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PA01"]
    #[inline(always)]
    pub fn pa01(&self) -> PA01_R {
        PA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PA02"]
    #[inline(always)]
    pub fn pa02(&self) -> PA02_R {
        PA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PA03"]
    #[inline(always)]
    pub fn pa03(&self) -> PA03_R {
        PA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PA04"]
    #[inline(always)]
    pub fn pa04(&self) -> PA04_R {
        PA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PA05"]
    #[inline(always)]
    pub fn pa05(&self) -> PA05_R {
        PA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PA06"]
    #[inline(always)]
    pub fn pa06(&self) -> PA06_R {
        PA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PA07"]
    #[inline(always)]
    pub fn pa07(&self) -> PA07_R {
        PA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PA08"]
    #[inline(always)]
    pub fn pa08(&self) -> PA08_R {
        PA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PA09"]
    #[inline(always)]
    pub fn pa09(&self) -> PA09_R {
        PA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PA10"]
    #[inline(always)]
    pub fn pa10(&self) -> PA10_R {
        PA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PA11"]
    #[inline(always)]
    pub fn pa11(&self) -> PA11_R {
        PA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PA12"]
    #[inline(always)]
    pub fn pa12(&self) -> PA12_R {
        PA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PA13"]
    #[inline(always)]
    pub fn pa13(&self) -> PA13_R {
        PA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PA14"]
    #[inline(always)]
    pub fn pa14(&self) -> PA14_R {
        PA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PA15"]
    #[inline(always)]
    pub fn pa15(&self) -> PA15_R {
        PA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc PAIN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pain](index.html) module"]
pub struct PAIN_SPEC;
impl crate::RegisterSpec for PAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pain::R](R) reader structure"]
impl crate::Readable for PAIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAIN to value 0"]
impl crate::Resettable for PAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
