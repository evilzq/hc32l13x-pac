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
#[doc = "Field `UF` reader - desc UF"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `OV` reader - desc OV"]
pub type OV_R = crate::BitReader<bool>;
#[doc = "Field `TO` reader - desc TO"]
pub type TO_R = crate::BitReader<bool>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - desc FE"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `BB` reader - desc BB"]
pub type BB_R = crate::BitReader<bool>;
#[doc = "Field `S0E` reader - desc S0E"]
pub type S0E_R = crate::BitReader<bool>;
#[doc = "Field `S1E` reader - desc S1E"]
pub type S1E_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc UF"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BB"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc S0E"]
    #[inline(always)]
    pub fn s0e(&self) -> S0E_R {
        S0E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc S1E"]
    #[inline(always)]
    pub fn s1e(&self) -> S1E_R {
        S1E_R::new(((self.bits >> 7) & 1) != 0)
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
