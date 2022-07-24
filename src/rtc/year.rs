#[doc = "Register `YEAR` reader"]
pub struct R(crate::R<YEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `YEAR` writer"]
pub struct W(crate::W<YEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YEAR_SPEC>;
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
impl From<crate::W<YEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YEARL` reader - desc YEARL"]
pub type YEARL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEARL` writer - desc YEARL"]
pub type YEARL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YEAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `YEARH` reader - desc YEARH"]
pub type YEARH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEARH` writer - desc YEARH"]
pub type YEARH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YEAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, YEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc YEARL"]
    #[inline(always)]
    pub fn yearl(&self) -> YEARL_R {
        YEARL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc YEARH"]
    #[inline(always)]
    pub fn yearh(&self) -> YEARH_R {
        YEARH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc YEARL"]
    #[inline(always)]
    pub fn yearl(&mut self) -> YEARL_W<0> {
        YEARL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc YEARH"]
    #[inline(always)]
    pub fn yearh(&mut self) -> YEARH_W<4> {
        YEARH_W::new(self)
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
#[doc = "desc YEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [year](index.html) module"]
pub struct YEAR_SPEC;
impl crate::RegisterSpec for YEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [year::R](R) reader structure"]
impl crate::Readable for YEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [year::W](W) writer structure"]
impl crate::Writable for YEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets YEAR to value 0"]
impl crate::Resettable for YEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
