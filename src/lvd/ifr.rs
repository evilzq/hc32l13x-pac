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
#[doc = "Field `INTF` reader - desc INTF"]
pub type INTF_R = crate::BitReader<bool>;
#[doc = "Field `INTF` writer - desc INTF"]
pub type INTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `FILTER` reader - desc FILTER"]
pub type FILTER_R = crate::BitReader<bool>;
#[doc = "Field `FILTER` writer - desc FILTER"]
pub type FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FILTER"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    pub fn intf(&mut self) -> INTF_W<0> {
        INTF_W::new(self)
    }
    #[doc = "Bit 1 - desc FILTER"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W<1> {
        FILTER_W::new(self)
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
