#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSN1_SEL` reader - desc SSN1_SEL"]
pub type SSN1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSN1_SEL` writer - desc SSN1_SEL"]
pub type SSN1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `AHB_SEL` reader - desc AHB_SEL"]
pub type AHB_SEL_R = crate::BitReader<bool>;
#[doc = "Field `AHB_SEL` writer - desc AHB_SEL"]
pub type AHB_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc SSN1_SEL"]
    #[inline(always)]
    pub fn ssn1_sel(&self) -> SSN1_SEL_R {
        SSN1_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc AHB_SEL"]
    #[inline(always)]
    pub fn ahb_sel(&self) -> AHB_SEL_R {
        AHB_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SSN1_SEL"]
    #[inline(always)]
    pub fn ssn1_sel(&mut self) -> SSN1_SEL_W<0> {
        SSN1_SEL_W::new(self)
    }
    #[doc = "Bit 15 - desc AHB_SEL"]
    #[inline(always)]
    pub fn ahb_sel(&mut self) -> AHB_SEL_W<15> {
        AHB_SEL_W::new(self)
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
#[doc = "desc CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
