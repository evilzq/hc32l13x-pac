#[doc = "Register `XTL_CR` reader"]
pub struct R(crate::R<XTL_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTL_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTL_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTL_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTL_CR` writer"]
pub struct W(crate::W<XTL_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTL_CR_SPEC>;
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
impl From<crate::W<XTL_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTL_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVER` reader - desc DRIVER"]
pub type DRIVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVER` writer - desc DRIVER"]
pub type DRIVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `AMP_SEL` reader - desc AMP_SEL"]
pub type AMP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP_SEL` writer - desc AMP_SEL"]
pub type AMP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STARTUP` reader - desc STARTUP"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - desc STARTUP"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTL_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, XTL_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&self) -> DRIVER_R {
        DRIVER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc AMP_SEL"]
    #[inline(always)]
    pub fn amp_sel(&self) -> AMP_SEL_R {
        AMP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DRIVER"]
    #[inline(always)]
    pub fn driver(&mut self) -> DRIVER_W<0> {
        DRIVER_W::new(self)
    }
    #[doc = "Bits 2:3 - desc AMP_SEL"]
    #[inline(always)]
    pub fn amp_sel(&mut self) -> AMP_SEL_W<2> {
        AMP_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc STARTUP"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W<4> {
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
#[doc = "desc XTL_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtl_cr](index.html) module"]
pub struct XTL_CR_SPEC;
impl crate::RegisterSpec for XTL_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtl_cr::R](R) reader structure"]
impl crate::Readable for XTL_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtl_cr::W](W) writer structure"]
impl crate::Writable for XTL_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTL_CR to value 0x21"]
impl crate::Resettable for XTL_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
