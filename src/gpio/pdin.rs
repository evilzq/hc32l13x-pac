#[doc = "Register `PDIN` reader"]
pub struct R(crate::R<PDIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PD00` reader - desc PD00"]
pub type PD00_R = crate::BitReader<bool>;
#[doc = "Field `PD01` reader - desc PD01"]
pub type PD01_R = crate::BitReader<bool>;
#[doc = "Field `PD02` reader - desc PD02"]
pub type PD02_R = crate::BitReader<bool>;
#[doc = "Field `PD03` reader - desc PD03"]
pub type PD03_R = crate::BitReader<bool>;
#[doc = "Field `PD04` reader - desc PD04"]
pub type PD04_R = crate::BitReader<bool>;
#[doc = "Field `PD05` reader - desc PD05"]
pub type PD05_R = crate::BitReader<bool>;
#[doc = "Field `PD06` reader - desc PD06"]
pub type PD06_R = crate::BitReader<bool>;
#[doc = "Field `PD07` reader - desc PD07"]
pub type PD07_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc PD00"]
    #[inline(always)]
    pub fn pd00(&self) -> PD00_R {
        PD00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PD01"]
    #[inline(always)]
    pub fn pd01(&self) -> PD01_R {
        PD01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PD02"]
    #[inline(always)]
    pub fn pd02(&self) -> PD02_R {
        PD02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&self) -> PD03_R {
        PD03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PD04"]
    #[inline(always)]
    pub fn pd04(&self) -> PD04_R {
        PD04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PD05"]
    #[inline(always)]
    pub fn pd05(&self) -> PD05_R {
        PD05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PD06"]
    #[inline(always)]
    pub fn pd06(&self) -> PD06_R {
        PD06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&self) -> PD07_R {
        PD07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc PDIN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdin](index.html) module"]
pub struct PDIN_SPEC;
impl crate::RegisterSpec for PDIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdin::R](R) reader structure"]
impl crate::Readable for PDIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDIN to value 0"]
impl crate::Resettable for PDIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
