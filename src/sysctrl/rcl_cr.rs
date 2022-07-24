#[doc = "Register `RCL_CR` reader"]
pub struct R(crate::R<RCL_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCL_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCL_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCL_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCL_CR` writer"]
pub struct W(crate::W<RCL_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCL_CR_SPEC>;
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
impl From<crate::W<RCL_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCL_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCL_CR_SPEC, u16, u16, 10, O>;
#[doc = "Field `STARTUP` reader - desc STARTUP"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - desc STARTUP"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCL_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Bits 10:11 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W<10> {
        STARTUP_W::new(self)
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
#[doc = "desc RCL_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcl_cr](index.html) module"]
pub struct RCL_CR_SPEC;
impl crate::RegisterSpec for RCL_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcl_cr::R](R) reader structure"]
impl crate::Readable for RCL_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcl_cr::W](W) writer structure"]
impl crate::Writable for RCL_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCL_CR to value 0x033f"]
impl crate::Resettable for RCL_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x033f
    }
}
