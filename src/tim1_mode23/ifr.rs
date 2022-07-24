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
#[doc = "Field `CB0F` reader - desc CB0F"]
pub type CB0F_R = crate::BitReader<bool>;
#[doc = "Field `CA0E` reader - desc CA0E"]
pub type CA0E_R = crate::BitReader<bool>;
#[doc = "Field `CB0E` reader - desc CB0E"]
pub type CB0E_R = crate::BitReader<bool>;
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
    #[doc = "Bit 5 - desc CB0F"]
    #[inline(always)]
    pub fn cb0f(&self) -> CB0F_R {
        CB0F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CA0E"]
    #[inline(always)]
    pub fn ca0e(&self) -> CA0E_R {
        CA0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CB0E"]
    #[inline(always)]
    pub fn cb0e(&self) -> CB0E_R {
        CB0E_R::new(((self.bits >> 11) & 1) != 0)
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
