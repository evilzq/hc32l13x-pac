#[doc = "Register `HSTPR` reader"]
pub struct R(crate::R<HSTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTPR` writer"]
pub struct W(crate::W<HSTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPR_SPEC>;
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
impl From<crate::W<HSTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTP0` reader - desc HSTP0"]
pub type HSTP0_R = crate::BitReader<bool>;
#[doc = "Field `HSTP0` writer - desc HSTP0"]
pub type HSTP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP1` reader - desc HSTP1"]
pub type HSTP1_R = crate::BitReader<bool>;
#[doc = "Field `HSTP1` writer - desc HSTP1"]
pub type HSTP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP2` reader - desc HSTP2"]
pub type HSTP2_R = crate::BitReader<bool>;
#[doc = "Field `HSTP2` writer - desc HSTP2"]
pub type HSTP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP3` reader - desc HSTP3"]
pub type HSTP3_R = crate::BitReader<bool>;
#[doc = "Field `HSTP3` writer - desc HSTP3"]
pub type HSTP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP4` reader - desc HSTP4"]
pub type HSTP4_R = crate::BitReader<bool>;
#[doc = "Field `HSTP4` writer - desc HSTP4"]
pub type HSTP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP5` reader - desc HSTP5"]
pub type HSTP5_R = crate::BitReader<bool>;
#[doc = "Field `HSTP5` writer - desc HSTP5"]
pub type HSTP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP6` reader - desc HSTP6"]
pub type HSTP6_R = crate::BitReader<bool>;
#[doc = "Field `HSTP6` writer - desc HSTP6"]
pub type HSTP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP7` reader - desc HSTP7"]
pub type HSTP7_R = crate::BitReader<bool>;
#[doc = "Field `HSTP7` writer - desc HSTP7"]
pub type HSTP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP8` reader - desc HSTP8"]
pub type HSTP8_R = crate::BitReader<bool>;
#[doc = "Field `HSTP8` writer - desc HSTP8"]
pub type HSTP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP9` reader - desc HSTP9"]
pub type HSTP9_R = crate::BitReader<bool>;
#[doc = "Field `HSTP9` writer - desc HSTP9"]
pub type HSTP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP10` reader - desc HSTP10"]
pub type HSTP10_R = crate::BitReader<bool>;
#[doc = "Field `HSTP10` writer - desc HSTP10"]
pub type HSTP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP11` reader - desc HSTP11"]
pub type HSTP11_R = crate::BitReader<bool>;
#[doc = "Field `HSTP11` writer - desc HSTP11"]
pub type HSTP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP12` reader - desc HSTP12"]
pub type HSTP12_R = crate::BitReader<bool>;
#[doc = "Field `HSTP12` writer - desc HSTP12"]
pub type HSTP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP13` reader - desc HSTP13"]
pub type HSTP13_R = crate::BitReader<bool>;
#[doc = "Field `HSTP13` writer - desc HSTP13"]
pub type HSTP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP14` reader - desc HSTP14"]
pub type HSTP14_R = crate::BitReader<bool>;
#[doc = "Field `HSTP14` writer - desc HSTP14"]
pub type HSTP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `HSTP15` reader - desc HSTP15"]
pub type HSTP15_R = crate::BitReader<bool>;
#[doc = "Field `HSTP15` writer - desc HSTP15"]
pub type HSTP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
#[doc = "Field `STOPS` reader - desc STOPS"]
pub type STOPS_R = crate::BitReader<bool>;
#[doc = "Field `STOPS` writer - desc STOPS"]
pub type STOPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&self) -> HSTP0_R {
        HSTP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&self) -> HSTP1_R {
        HSTP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&self) -> HSTP2_R {
        HSTP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HSTP3"]
    #[inline(always)]
    pub fn hstp3(&self) -> HSTP3_R {
        HSTP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HSTP4"]
    #[inline(always)]
    pub fn hstp4(&self) -> HSTP4_R {
        HSTP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSTP5"]
    #[inline(always)]
    pub fn hstp5(&self) -> HSTP5_R {
        HSTP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSTP6"]
    #[inline(always)]
    pub fn hstp6(&self) -> HSTP6_R {
        HSTP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSTP7"]
    #[inline(always)]
    pub fn hstp7(&self) -> HSTP7_R {
        HSTP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&self) -> HSTP8_R {
        HSTP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&self) -> HSTP9_R {
        HSTP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HSTP10"]
    #[inline(always)]
    pub fn hstp10(&self) -> HSTP10_R {
        HSTP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSTP11"]
    #[inline(always)]
    pub fn hstp11(&self) -> HSTP11_R {
        HSTP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HSTP12"]
    #[inline(always)]
    pub fn hstp12(&self) -> HSTP12_R {
        HSTP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HSTP13"]
    #[inline(always)]
    pub fn hstp13(&self) -> HSTP13_R {
        HSTP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HSTP14"]
    #[inline(always)]
    pub fn hstp14(&self) -> HSTP14_R {
        HSTP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HSTP15"]
    #[inline(always)]
    pub fn hstp15(&self) -> HSTP15_R {
        HSTP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STOPS"]
    #[inline(always)]
    pub fn stops(&self) -> STOPS_R {
        STOPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&mut self) -> HSTP0_W<0> {
        HSTP0_W::new(self)
    }
    #[doc = "Bit 1 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&mut self) -> HSTP1_W<1> {
        HSTP1_W::new(self)
    }
    #[doc = "Bit 2 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&mut self) -> HSTP2_W<2> {
        HSTP2_W::new(self)
    }
    #[doc = "Bit 3 - desc HSTP3"]
    #[inline(always)]
    pub fn hstp3(&mut self) -> HSTP3_W<3> {
        HSTP3_W::new(self)
    }
    #[doc = "Bit 4 - desc HSTP4"]
    #[inline(always)]
    pub fn hstp4(&mut self) -> HSTP4_W<4> {
        HSTP4_W::new(self)
    }
    #[doc = "Bit 5 - desc HSTP5"]
    #[inline(always)]
    pub fn hstp5(&mut self) -> HSTP5_W<5> {
        HSTP5_W::new(self)
    }
    #[doc = "Bit 6 - desc HSTP6"]
    #[inline(always)]
    pub fn hstp6(&mut self) -> HSTP6_W<6> {
        HSTP6_W::new(self)
    }
    #[doc = "Bit 7 - desc HSTP7"]
    #[inline(always)]
    pub fn hstp7(&mut self) -> HSTP7_W<7> {
        HSTP7_W::new(self)
    }
    #[doc = "Bit 8 - desc HSTP8"]
    #[inline(always)]
    pub fn hstp8(&mut self) -> HSTP8_W<8> {
        HSTP8_W::new(self)
    }
    #[doc = "Bit 9 - desc HSTP9"]
    #[inline(always)]
    pub fn hstp9(&mut self) -> HSTP9_W<9> {
        HSTP9_W::new(self)
    }
    #[doc = "Bit 10 - desc HSTP10"]
    #[inline(always)]
    pub fn hstp10(&mut self) -> HSTP10_W<10> {
        HSTP10_W::new(self)
    }
    #[doc = "Bit 11 - desc HSTP11"]
    #[inline(always)]
    pub fn hstp11(&mut self) -> HSTP11_W<11> {
        HSTP11_W::new(self)
    }
    #[doc = "Bit 12 - desc HSTP12"]
    #[inline(always)]
    pub fn hstp12(&mut self) -> HSTP12_W<12> {
        HSTP12_W::new(self)
    }
    #[doc = "Bit 13 - desc HSTP13"]
    #[inline(always)]
    pub fn hstp13(&mut self) -> HSTP13_W<13> {
        HSTP13_W::new(self)
    }
    #[doc = "Bit 14 - desc HSTP14"]
    #[inline(always)]
    pub fn hstp14(&mut self) -> HSTP14_W<14> {
        HSTP14_W::new(self)
    }
    #[doc = "Bit 15 - desc HSTP15"]
    #[inline(always)]
    pub fn hstp15(&mut self) -> HSTP15_W<15> {
        HSTP15_W::new(self)
    }
    #[doc = "Bit 31 - desc STOPS"]
    #[inline(always)]
    pub fn stops(&mut self) -> STOPS_W<31> {
        STOPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc HSTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpr](index.html) module"]
pub struct HSTPR_SPEC;
impl crate::RegisterSpec for HSTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpr::R](R) reader structure"]
impl crate::Readable for HSTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstpr::W](W) writer structure"]
impl crate::Writable for HSTPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTPR to value 0"]
impl crate::Resettable for HSTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
