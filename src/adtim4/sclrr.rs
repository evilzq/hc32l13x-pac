#[doc = "Register `SCLRR` reader"]
pub struct R(crate::R<SCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLRR` writer"]
pub struct W(crate::W<SCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLRR_SPEC>;
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
impl From<crate::W<SCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLR0` reader - desc SCLR0"]
pub type SCLR0_R = crate::BitReader<bool>;
#[doc = "Field `SCLR0` writer - desc SCLR0"]
pub type SCLR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCLRR_SPEC, bool, O>;
#[doc = "Field `SCLR1` reader - desc SCLR1"]
pub type SCLR1_R = crate::BitReader<bool>;
#[doc = "Field `SCLR1` writer - desc SCLR1"]
pub type SCLR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCLRR_SPEC, bool, O>;
#[doc = "Field `SCLR2` reader - desc SCLR2"]
pub type SCLR2_R = crate::BitReader<bool>;
#[doc = "Field `SCLR2` writer - desc SCLR2"]
pub type SCLR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCLRR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SCLR0"]
    #[inline(always)]
    pub fn sclr0(&self) -> SCLR0_R {
        SCLR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SCLR1"]
    #[inline(always)]
    pub fn sclr1(&self) -> SCLR1_R {
        SCLR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SCLR2"]
    #[inline(always)]
    pub fn sclr2(&self) -> SCLR2_R {
        SCLR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SCLR0"]
    #[inline(always)]
    pub fn sclr0(&mut self) -> SCLR0_W<0> {
        SCLR0_W::new(self)
    }
    #[doc = "Bit 1 - desc SCLR1"]
    #[inline(always)]
    pub fn sclr1(&mut self) -> SCLR1_W<1> {
        SCLR1_W::new(self)
    }
    #[doc = "Bit 2 - desc SCLR2"]
    #[inline(always)]
    pub fn sclr2(&mut self) -> SCLR2_W<2> {
        SCLR2_W::new(self)
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
#[doc = "desc SCLRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclrr](index.html) module"]
pub struct SCLRR_SPEC;
impl crate::RegisterSpec for SCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclrr::R](R) reader structure"]
impl crate::Readable for SCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclrr::W](W) writer structure"]
impl crate::Writable for SCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLRR to value 0"]
impl crate::Resettable for SCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
