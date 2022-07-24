#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T2C` reader - desc T2C"]
pub type T2C_R = crate::BitReader<bool>;
#[doc = "Field `B2T` reader - desc B2T"]
pub type B2T_R = crate::BitReader<bool>;
#[doc = "Field `B2C` reader - desc B2C"]
pub type B2C_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc T2C"]
    #[inline(always)]
    pub fn t2c(&self) -> T2C_R {
        T2C_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc B2T"]
    #[inline(always)]
    pub fn b2t(&self) -> B2T_R {
        B2T_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc B2C"]
    #[inline(always)]
    pub fn b2c(&self) -> B2C_R {
        B2C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc SR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
