#[doc = "Register `CCAPO` reader"]
pub struct R(crate::R<CCAPO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAPO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAPO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAPO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAPO` writer"]
pub struct W(crate::W<CCAPO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAPO_SPEC>;
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
impl From<crate::W<CCAPO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAPO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCAPO0` reader - desc CCAPO0"]
pub type CCAPO0_R = crate::BitReader<bool>;
#[doc = "Field `CCAPO0` writer - desc CCAPO0"]
pub type CCAPO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
#[doc = "Field `CCAPO1` reader - desc CCAPO1"]
pub type CCAPO1_R = crate::BitReader<bool>;
#[doc = "Field `CCAPO1` writer - desc CCAPO1"]
pub type CCAPO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
#[doc = "Field `CCAPO2` reader - desc CCAPO2"]
pub type CCAPO2_R = crate::BitReader<bool>;
#[doc = "Field `CCAPO2` writer - desc CCAPO2"]
pub type CCAPO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
#[doc = "Field `CCAPO3` reader - desc CCAPO3"]
pub type CCAPO3_R = crate::BitReader<bool>;
#[doc = "Field `CCAPO3` writer - desc CCAPO3"]
pub type CCAPO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
#[doc = "Field `CCAPO4` reader - desc CCAPO4"]
pub type CCAPO4_R = crate::BitReader<bool>;
#[doc = "Field `CCAPO4` writer - desc CCAPO4"]
pub type CCAPO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CCAPO0"]
    #[inline(always)]
    pub fn ccapo0(&self) -> CCAPO0_R {
        CCAPO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCAPO1"]
    #[inline(always)]
    pub fn ccapo1(&self) -> CCAPO1_R {
        CCAPO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CCAPO2"]
    #[inline(always)]
    pub fn ccapo2(&self) -> CCAPO2_R {
        CCAPO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CCAPO3"]
    #[inline(always)]
    pub fn ccapo3(&self) -> CCAPO3_R {
        CCAPO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CCAPO4"]
    #[inline(always)]
    pub fn ccapo4(&self) -> CCAPO4_R {
        CCAPO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCAPO0"]
    #[inline(always)]
    pub fn ccapo0(&mut self) -> CCAPO0_W<0> {
        CCAPO0_W::new(self)
    }
    #[doc = "Bit 1 - desc CCAPO1"]
    #[inline(always)]
    pub fn ccapo1(&mut self) -> CCAPO1_W<1> {
        CCAPO1_W::new(self)
    }
    #[doc = "Bit 2 - desc CCAPO2"]
    #[inline(always)]
    pub fn ccapo2(&mut self) -> CCAPO2_W<2> {
        CCAPO2_W::new(self)
    }
    #[doc = "Bit 3 - desc CCAPO3"]
    #[inline(always)]
    pub fn ccapo3(&mut self) -> CCAPO3_W<3> {
        CCAPO3_W::new(self)
    }
    #[doc = "Bit 4 - desc CCAPO4"]
    #[inline(always)]
    pub fn ccapo4(&mut self) -> CCAPO4_W<4> {
        CCAPO4_W::new(self)
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
#[doc = "desc CCAPO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccapo](index.html) module"]
pub struct CCAPO_SPEC;
impl crate::RegisterSpec for CCAPO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccapo::R](R) reader structure"]
impl crate::Readable for CCAPO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccapo::W](W) writer structure"]
impl crate::Writable for CCAPO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCAPO to value 0"]
impl crate::Resettable for CCAPO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
