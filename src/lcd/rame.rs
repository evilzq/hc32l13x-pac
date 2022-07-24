#[doc = "Register `RAME` reader"]
pub struct R(crate::R<RAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAME` writer"]
pub struct W(crate::W<RAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAME_SPEC>;
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
impl From<crate::W<RAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D0` reader - desc D0"]
pub type D0_R = crate::BitReader<bool>;
#[doc = "Field `D0` writer - desc D0"]
pub type D0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D1` reader - desc D1"]
pub type D1_R = crate::BitReader<bool>;
#[doc = "Field `D1` writer - desc D1"]
pub type D1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D2` reader - desc D2"]
pub type D2_R = crate::BitReader<bool>;
#[doc = "Field `D2` writer - desc D2"]
pub type D2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D3` reader - desc D3"]
pub type D3_R = crate::BitReader<bool>;
#[doc = "Field `D3` writer - desc D3"]
pub type D3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D4` reader - desc D4"]
pub type D4_R = crate::BitReader<bool>;
#[doc = "Field `D4` writer - desc D4"]
pub type D4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D5` reader - desc D5"]
pub type D5_R = crate::BitReader<bool>;
#[doc = "Field `D5` writer - desc D5"]
pub type D5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D6` reader - desc D6"]
pub type D6_R = crate::BitReader<bool>;
#[doc = "Field `D6` writer - desc D6"]
pub type D6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
#[doc = "Field `D7` reader - desc D7"]
pub type D7_R = crate::BitReader<bool>;
#[doc = "Field `D7` writer - desc D7"]
pub type D7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAME_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc D0"]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc D1"]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc D2"]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc D3"]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc D4"]
    #[inline(always)]
    pub fn d4(&self) -> D4_R {
        D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc D5"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc D6"]
    #[inline(always)]
    pub fn d6(&self) -> D6_R {
        D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc D7"]
    #[inline(always)]
    pub fn d7(&self) -> D7_R {
        D7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc D0"]
    #[inline(always)]
    pub fn d0(&mut self) -> D0_W<0> {
        D0_W::new(self)
    }
    #[doc = "Bit 1 - desc D1"]
    #[inline(always)]
    pub fn d1(&mut self) -> D1_W<1> {
        D1_W::new(self)
    }
    #[doc = "Bit 2 - desc D2"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W<2> {
        D2_W::new(self)
    }
    #[doc = "Bit 3 - desc D3"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W<3> {
        D3_W::new(self)
    }
    #[doc = "Bit 4 - desc D4"]
    #[inline(always)]
    pub fn d4(&mut self) -> D4_W<4> {
        D4_W::new(self)
    }
    #[doc = "Bit 5 - desc D5"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W<5> {
        D5_W::new(self)
    }
    #[doc = "Bit 6 - desc D6"]
    #[inline(always)]
    pub fn d6(&mut self) -> D6_W<6> {
        D6_W::new(self)
    }
    #[doc = "Bit 7 - desc D7"]
    #[inline(always)]
    pub fn d7(&mut self) -> D7_W<7> {
        D7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RAME\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rame](index.html) module"]
pub struct RAME_SPEC;
impl crate::RegisterSpec for RAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rame::R](R) reader structure"]
impl crate::Readable for RAME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rame::W](W) writer structure"]
impl crate::Writable for RAME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAME to value 0"]
impl crate::Resettable for RAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
