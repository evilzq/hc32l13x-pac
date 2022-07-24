#[doc = "Register `SSTAR` reader"]
pub struct R(crate::R<SSTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTAR` writer"]
pub struct W(crate::W<SSTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTAR_SPEC>;
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
impl From<crate::W<SSTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTA0` reader - desc SSTA0"]
pub type SSTA0_R = crate::BitReader<bool>;
#[doc = "Field `SSTA0` writer - desc SSTA0"]
pub type SSTA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAR_SPEC, bool, O>;
#[doc = "Field `SSTA1` reader - desc SSTA1"]
pub type SSTA1_R = crate::BitReader<bool>;
#[doc = "Field `SSTA1` writer - desc SSTA1"]
pub type SSTA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAR_SPEC, bool, O>;
#[doc = "Field `SSTA2` reader - desc SSTA2"]
pub type SSTA2_R = crate::BitReader<bool>;
#[doc = "Field `SSTA2` writer - desc SSTA2"]
pub type SSTA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SSTA0"]
    #[inline(always)]
    pub fn ssta0(&self) -> SSTA0_R {
        SSTA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SSTA1"]
    #[inline(always)]
    pub fn ssta1(&self) -> SSTA1_R {
        SSTA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSTA2"]
    #[inline(always)]
    pub fn ssta2(&self) -> SSTA2_R {
        SSTA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSTA0"]
    #[inline(always)]
    pub fn ssta0(&mut self) -> SSTA0_W<0> {
        SSTA0_W::new(self)
    }
    #[doc = "Bit 1 - desc SSTA1"]
    #[inline(always)]
    pub fn ssta1(&mut self) -> SSTA1_W<1> {
        SSTA1_W::new(self)
    }
    #[doc = "Bit 2 - desc SSTA2"]
    #[inline(always)]
    pub fn ssta2(&mut self) -> SSTA2_W<2> {
        SSTA2_W::new(self)
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
#[doc = "desc SSTAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstar](index.html) module"]
pub struct SSTAR_SPEC;
impl crate::RegisterSpec for SSTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstar::R](R) reader structure"]
impl crate::Readable for SSTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstar::W](W) writer structure"]
impl crate::Writable for SSTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTAR to value 0"]
impl crate::Resettable for SSTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
