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
#[doc = "Field `UIF` reader - desc UIF"]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `CA0F` reader - desc CA0F"]
pub type CA0F_R = crate::BitReader<bool>;
#[doc = "Field `CA1F` reader - desc CA1F"]
pub type CA1F_R = crate::BitReader<bool>;
#[doc = "Field `CA2F` reader - desc CA2F"]
pub type CA2F_R = crate::BitReader<bool>;
#[doc = "Field `CB0F` reader - desc CB0F"]
pub type CB0F_R = crate::BitReader<bool>;
#[doc = "Field `CB1F` reader - desc CB1F"]
pub type CB1F_R = crate::BitReader<bool>;
#[doc = "Field `CB2F` reader - desc CB2F"]
pub type CB2F_R = crate::BitReader<bool>;
#[doc = "Field `CA0E` reader - desc CA0E"]
pub type CA0E_R = crate::BitReader<bool>;
#[doc = "Field `CA1E` reader - desc CA1E"]
pub type CA1E_R = crate::BitReader<bool>;
#[doc = "Field `CA2E` reader - desc CA2E"]
pub type CA2E_R = crate::BitReader<bool>;
#[doc = "Field `CB0E` reader - desc CB0E"]
pub type CB0E_R = crate::BitReader<bool>;
#[doc = "Field `CB1E` reader - desc CB1E"]
pub type CB1E_R = crate::BitReader<bool>;
#[doc = "Field `CB2E` reader - desc CB2E"]
pub type CB2E_R = crate::BitReader<bool>;
#[doc = "Field `BIF` reader - desc BIF"]
pub type BIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` reader - desc TIF"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc CA0F"]
    #[inline(always)]
    pub fn ca0f(&self) -> CA0F_R {
        CA0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CA1F"]
    #[inline(always)]
    pub fn ca1f(&self) -> CA1F_R {
        CA1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CA2F"]
    #[inline(always)]
    pub fn ca2f(&self) -> CA2F_R {
        CA2F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CB0F"]
    #[inline(always)]
    pub fn cb0f(&self) -> CB0F_R {
        CB0F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CB1F"]
    #[inline(always)]
    pub fn cb1f(&self) -> CB1F_R {
        CB1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CB2F"]
    #[inline(always)]
    pub fn cb2f(&self) -> CB2F_R {
        CB2F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CA0E"]
    #[inline(always)]
    pub fn ca0e(&self) -> CA0E_R {
        CA0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CA1E"]
    #[inline(always)]
    pub fn ca1e(&self) -> CA1E_R {
        CA1E_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CA2E"]
    #[inline(always)]
    pub fn ca2e(&self) -> CA2E_R {
        CA2E_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CB0E"]
    #[inline(always)]
    pub fn cb0e(&self) -> CB0E_R {
        CB0E_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CB1E"]
    #[inline(always)]
    pub fn cb1e(&self) -> CB1E_R {
        CB1E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc CB2E"]
    #[inline(always)]
    pub fn cb2e(&self) -> CB2E_R {
        CB2E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 15) & 1) != 0)
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
