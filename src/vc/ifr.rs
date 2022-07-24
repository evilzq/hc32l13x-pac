#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFR` writer"]
pub struct W(crate::W<IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFR_SPEC>;
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
impl From<crate::W<IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VC0_INTF` reader - desc VC0_INTF"]
pub type VC0_INTF_R = crate::BitReader<bool>;
#[doc = "Field `VC0_INTF` writer - desc VC0_INTF"]
pub type VC0_INTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `VC1_INTF` reader - desc VC1_INTF"]
pub type VC1_INTF_R = crate::BitReader<bool>;
#[doc = "Field `VC1_INTF` writer - desc VC1_INTF"]
pub type VC1_INTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `VC0_FILTER` reader - desc VC0_FILTER"]
pub type VC0_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `VC1_FILTER` reader - desc VC1_FILTER"]
pub type VC1_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc VC0_INTF"]
    #[inline(always)]
    pub fn vc0_intf(&self) -> VC0_INTF_R {
        VC0_INTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc VC1_INTF"]
    #[inline(always)]
    pub fn vc1_intf(&self) -> VC1_INTF_R {
        VC1_INTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc VC0_FILTER"]
    #[inline(always)]
    pub fn vc0_filter(&self) -> VC0_FILTER_R {
        VC0_FILTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc VC1_FILTER"]
    #[inline(always)]
    pub fn vc1_filter(&self) -> VC1_FILTER_R {
        VC1_FILTER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc VC0_INTF"]
    #[inline(always)]
    pub fn vc0_intf(&mut self) -> VC0_INTF_W<0> {
        VC0_INTF_W::new(self)
    }
    #[doc = "Bit 1 - desc VC1_INTF"]
    #[inline(always)]
    pub fn vc1_intf(&mut self) -> VC1_INTF_W<1> {
        VC1_INTF_W::new(self)
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
#[doc = "desc IFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifr::W](W) writer structure"]
impl crate::Writable for IFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
