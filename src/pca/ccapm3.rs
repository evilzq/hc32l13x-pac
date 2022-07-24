#[doc = "Register `CCAPM3` reader"]
pub struct R(crate::R<CCAPM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCAPM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCAPM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCAPM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCAPM3` writer"]
pub struct W(crate::W<CCAPM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCAPM3_SPEC>;
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
impl From<crate::W<CCAPM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCAPM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCIE` reader - desc CCIE"]
pub type CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CCIE` writer - desc CCIE"]
pub type CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `PWM` reader - desc PWM"]
pub type PWM_R = crate::BitReader<bool>;
#[doc = "Field `PWM` writer - desc PWM"]
pub type PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `TOG` reader - desc TOG"]
pub type TOG_R = crate::BitReader<bool>;
#[doc = "Field `TOG` writer - desc TOG"]
pub type TOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `MAT` reader - desc MAT"]
pub type MAT_R = crate::BitReader<bool>;
#[doc = "Field `MAT` writer - desc MAT"]
pub type MAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `CAPN` reader - desc CAPN"]
pub type CAPN_R = crate::BitReader<bool>;
#[doc = "Field `CAPN` writer - desc CAPN"]
pub type CAPN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `CAPP` reader - desc CAPP"]
pub type CAPP_R = crate::BitReader<bool>;
#[doc = "Field `CAPP` writer - desc CAPP"]
pub type CAPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `ECOM` reader - desc ECOM"]
pub type ECOM_R = crate::BitReader<bool>;
#[doc = "Field `ECOM` writer - desc ECOM"]
pub type ECOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCAPM3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CCIE"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PWM"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TOG"]
    #[inline(always)]
    pub fn tog(&self) -> TOG_R {
        TOG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc MAT"]
    #[inline(always)]
    pub fn mat(&self) -> MAT_R {
        MAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CAPN"]
    #[inline(always)]
    pub fn capn(&self) -> CAPN_R {
        CAPN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CAPP"]
    #[inline(always)]
    pub fn capp(&self) -> CAPP_R {
        CAPP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ECOM"]
    #[inline(always)]
    pub fn ecom(&self) -> ECOM_R {
        ECOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCIE"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W<0> {
        CCIE_W::new(self)
    }
    #[doc = "Bit 1 - desc PWM"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W<1> {
        PWM_W::new(self)
    }
    #[doc = "Bit 2 - desc TOG"]
    #[inline(always)]
    pub fn tog(&mut self) -> TOG_W<2> {
        TOG_W::new(self)
    }
    #[doc = "Bit 3 - desc MAT"]
    #[inline(always)]
    pub fn mat(&mut self) -> MAT_W<3> {
        MAT_W::new(self)
    }
    #[doc = "Bit 4 - desc CAPN"]
    #[inline(always)]
    pub fn capn(&mut self) -> CAPN_W<4> {
        CAPN_W::new(self)
    }
    #[doc = "Bit 5 - desc CAPP"]
    #[inline(always)]
    pub fn capp(&mut self) -> CAPP_W<5> {
        CAPP_W::new(self)
    }
    #[doc = "Bit 6 - desc ECOM"]
    #[inline(always)]
    pub fn ecom(&mut self) -> ECOM_W<6> {
        ECOM_W::new(self)
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
#[doc = "desc CCAPM3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccapm3](index.html) module"]
pub struct CCAPM3_SPEC;
impl crate::RegisterSpec for CCAPM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccapm3::R](R) reader structure"]
impl crate::Readable for CCAPM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccapm3::W](W) writer structure"]
impl crate::Writable for CCAPM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCAPM3 to value 0"]
impl crate::Resettable for CCAPM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
