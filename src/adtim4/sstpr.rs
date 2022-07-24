#[doc = "Register `SSTPR` reader"]
pub struct R(crate::R<SSTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTPR` writer"]
pub struct W(crate::W<SSTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTPR_SPEC>;
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
impl From<crate::W<SSTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTP0` reader - desc SSTP0"]
pub type SSTP0_R = crate::BitReader<bool>;
#[doc = "Field `SSTP0` writer - desc SSTP0"]
pub type SSTP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTPR_SPEC, bool, O>;
#[doc = "Field `SSTP1` reader - desc SSTP1"]
pub type SSTP1_R = crate::BitReader<bool>;
#[doc = "Field `SSTP1` writer - desc SSTP1"]
pub type SSTP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTPR_SPEC, bool, O>;
#[doc = "Field `SSTP2` reader - desc SSTP2"]
pub type SSTP2_R = crate::BitReader<bool>;
#[doc = "Field `SSTP2` writer - desc SSTP2"]
pub type SSTP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTPR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSTPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc SSTP0"]
    #[inline(always)]
    pub fn sstp0(&self) -> SSTP0_R {
        SSTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SSTP1"]
    #[inline(always)]
    pub fn sstp1(&self) -> SSTP1_R {
        SSTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSTP2"]
    #[inline(always)]
    pub fn sstp2(&self) -> SSTP2_R {
        SSTP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SSTP0"]
    #[inline(always)]
    pub fn sstp0(&mut self) -> SSTP0_W<0> {
        SSTP0_W::new(self)
    }
    #[doc = "Bit 1 - desc SSTP1"]
    #[inline(always)]
    pub fn sstp1(&mut self) -> SSTP1_W<1> {
        SSTP1_W::new(self)
    }
    #[doc = "Bit 2 - desc SSTP2"]
    #[inline(always)]
    pub fn sstp2(&mut self) -> SSTP2_W<2> {
        SSTP2_W::new(self)
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
#[doc = "desc SSTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstpr](index.html) module"]
pub struct SSTPR_SPEC;
impl crate::RegisterSpec for SSTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstpr::R](R) reader structure"]
impl crate::Readable for SSTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstpr::W](W) writer structure"]
impl crate::Writable for SSTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTPR to value 0"]
impl crate::Resettable for SSTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
