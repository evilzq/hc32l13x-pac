#[doc = "Register `CON` reader"]
pub struct R(crate::R<CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CON` writer"]
pub struct W(crate::W<CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON_SPEC>;
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
impl From<crate::W<CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WOV` reader - desc WOV"]
pub type WOV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WOV` writer - desc WOV"]
pub type WOV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CON_SPEC, u8, u8, 4, O>;
#[doc = "Field `R` reader - desc R"]
pub type R_R = crate::BitReader<bool>;
#[doc = "Field `WINT_EN` reader - desc WINT_EN"]
pub type WINT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WINT_EN` writer - desc WINT_EN"]
pub type WINT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CON_SPEC, bool, O>;
#[doc = "Field `INT` reader - desc INT"]
pub type INT_R = crate::BitReader<bool>;
#[doc = "Field `WCNTL` reader - desc WCNTL"]
pub type WCNTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc WOV"]
    #[inline(always)]
    pub fn wov(&self) -> WOV_R {
        WOV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - desc R"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WINT_EN"]
    #[inline(always)]
    pub fn wint_en(&self) -> WINT_EN_R {
        WINT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc INT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc WCNTL"]
    #[inline(always)]
    pub fn wcntl(&self) -> WCNTL_R {
        WCNTL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc WOV"]
    #[inline(always)]
    pub fn wov(&mut self) -> WOV_W<0> {
        WOV_W::new(self)
    }
    #[doc = "Bit 5 - desc WINT_EN"]
    #[inline(always)]
    pub fn wint_en(&mut self) -> WINT_EN_W<5> {
        WINT_EN_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](index.html) module"]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [con::R](R) reader structure"]
impl crate::Readable for CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [con::W](W) writer structure"]
impl crate::Writable for CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CON to value 0x0f"]
impl crate::Resettable for CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
