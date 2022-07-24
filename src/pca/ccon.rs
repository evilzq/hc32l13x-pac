#[doc = "Register `CCON` reader"]
pub struct R(crate::R<CCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCON` writer"]
pub struct W(crate::W<CCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCON_SPEC>;
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
impl From<crate::W<CCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCF0` reader - desc CCF0"]
pub type CCF0_R = crate::BitReader<bool>;
#[doc = "Field `CCF0` writer - desc CCF0"]
pub type CCF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CCF1` reader - desc CCF1"]
pub type CCF1_R = crate::BitReader<bool>;
#[doc = "Field `CCF1` writer - desc CCF1"]
pub type CCF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CCF2` reader - desc CCF2"]
pub type CCF2_R = crate::BitReader<bool>;
#[doc = "Field `CCF2` writer - desc CCF2"]
pub type CCF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CCF3` reader - desc CCF3"]
pub type CCF3_R = crate::BitReader<bool>;
#[doc = "Field `CCF3` writer - desc CCF3"]
pub type CCF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CCF4` reader - desc CCF4"]
pub type CCF4_R = crate::BitReader<bool>;
#[doc = "Field `CCF4` writer - desc CCF4"]
pub type CCF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CR` reader - desc CR"]
pub type CR_R = crate::BitReader<bool>;
#[doc = "Field `CR` writer - desc CR"]
pub type CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `CF` reader - desc CF"]
pub type CF_R = crate::BitReader<bool>;
#[doc = "Field `CF` writer - desc CF"]
pub type CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CCF0"]
    #[inline(always)]
    pub fn ccf0(&self) -> CCF0_R {
        CCF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CCF1"]
    #[inline(always)]
    pub fn ccf1(&self) -> CCF1_R {
        CCF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CCF2"]
    #[inline(always)]
    pub fn ccf2(&self) -> CCF2_R {
        CCF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CCF3"]
    #[inline(always)]
    pub fn ccf3(&self) -> CCF3_R {
        CCF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CCF4"]
    #[inline(always)]
    pub fn ccf4(&self) -> CCF4_R {
        CCF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CF"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCF0"]
    #[inline(always)]
    pub fn ccf0(&mut self) -> CCF0_W<0> {
        CCF0_W::new(self)
    }
    #[doc = "Bit 1 - desc CCF1"]
    #[inline(always)]
    pub fn ccf1(&mut self) -> CCF1_W<1> {
        CCF1_W::new(self)
    }
    #[doc = "Bit 2 - desc CCF2"]
    #[inline(always)]
    pub fn ccf2(&mut self) -> CCF2_W<2> {
        CCF2_W::new(self)
    }
    #[doc = "Bit 3 - desc CCF3"]
    #[inline(always)]
    pub fn ccf3(&mut self) -> CCF3_W<3> {
        CCF3_W::new(self)
    }
    #[doc = "Bit 4 - desc CCF4"]
    #[inline(always)]
    pub fn ccf4(&mut self) -> CCF4_W<4> {
        CCF4_W::new(self)
    }
    #[doc = "Bit 6 - desc CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<6> {
        CR_W::new(self)
    }
    #[doc = "Bit 7 - desc CF"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W<7> {
        CF_W::new(self)
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
#[doc = "desc CCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccon](index.html) module"]
pub struct CCON_SPEC;
impl crate::RegisterSpec for CCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccon::R](R) reader structure"]
impl crate::Readable for CCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccon::W](W) writer structure"]
impl crate::Writable for CCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCON to value 0"]
impl crate::Resettable for CCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
