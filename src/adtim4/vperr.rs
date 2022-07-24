#[doc = "Register `VPERR` reader"]
pub struct R(crate::R<VPERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VPERR` writer"]
pub struct W(crate::W<VPERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VPERR_SPEC>;
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
impl From<crate::W<VPERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VPERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEPERIA` reader - desc GEPERIA"]
pub type GEPERIA_R = crate::BitReader<bool>;
#[doc = "Field `GEPERIA` writer - desc GEPERIA"]
pub type GEPERIA_W<'a, const O: u8> = crate::BitWriter<'a, u32, VPERR_SPEC, bool, O>;
#[doc = "Field `GEPERIB` reader - desc GEPERIB"]
pub type GEPERIB_R = crate::BitReader<bool>;
#[doc = "Field `GEPERIB` writer - desc GEPERIB"]
pub type GEPERIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, VPERR_SPEC, bool, O>;
#[doc = "Field `GEPERIC` reader - desc GEPERIC"]
pub type GEPERIC_R = crate::BitReader<bool>;
#[doc = "Field `GEPERIC` writer - desc GEPERIC"]
pub type GEPERIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, VPERR_SPEC, bool, O>;
#[doc = "Field `GEPERID` reader - desc GEPERID"]
pub type GEPERID_R = crate::BitReader<bool>;
#[doc = "Field `GEPERID` writer - desc GEPERID"]
pub type GEPERID_W<'a, const O: u8> = crate::BitWriter<'a, u32, VPERR_SPEC, bool, O>;
#[doc = "Field `PCNTE` reader - desc PCNTE"]
pub type PCNTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCNTE` writer - desc PCNTE"]
pub type PCNTE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VPERR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCNTS` reader - desc PCNTS"]
pub type PCNTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCNTS` writer - desc PCNTS"]
pub type PCNTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VPERR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VPERR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc GEPERIA"]
    #[inline(always)]
    pub fn geperia(&self) -> GEPERIA_R {
        GEPERIA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc GEPERIB"]
    #[inline(always)]
    pub fn geperib(&self) -> GEPERIB_R {
        GEPERIB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc GEPERIC"]
    #[inline(always)]
    pub fn geperic(&self) -> GEPERIC_R {
        GEPERIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc GEPERID"]
    #[inline(always)]
    pub fn geperid(&self) -> GEPERID_R {
        GEPERID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc PCNTE"]
    #[inline(always)]
    pub fn pcnte(&self) -> PCNTE_R {
        PCNTE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - desc PCNTS"]
    #[inline(always)]
    pub fn pcnts(&self) -> PCNTS_R {
        PCNTS_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc GEPERIA"]
    #[inline(always)]
    pub fn geperia(&mut self) -> GEPERIA_W<0> {
        GEPERIA_W::new(self)
    }
    #[doc = "Bit 1 - desc GEPERIB"]
    #[inline(always)]
    pub fn geperib(&mut self) -> GEPERIB_W<1> {
        GEPERIB_W::new(self)
    }
    #[doc = "Bit 2 - desc GEPERIC"]
    #[inline(always)]
    pub fn geperic(&mut self) -> GEPERIC_W<2> {
        GEPERIC_W::new(self)
    }
    #[doc = "Bit 3 - desc GEPERID"]
    #[inline(always)]
    pub fn geperid(&mut self) -> GEPERID_W<3> {
        GEPERID_W::new(self)
    }
    #[doc = "Bits 16:17 - desc PCNTE"]
    #[inline(always)]
    pub fn pcnte(&mut self) -> PCNTE_W<16> {
        PCNTE_W::new(self)
    }
    #[doc = "Bits 18:20 - desc PCNTS"]
    #[inline(always)]
    pub fn pcnts(&mut self) -> PCNTS_W<18> {
        PCNTS_W::new(self)
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
#[doc = "desc VPERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vperr](index.html) module"]
pub struct VPERR_SPEC;
impl crate::RegisterSpec for VPERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vperr::R](R) reader structure"]
impl crate::Readable for VPERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vperr::W](W) writer structure"]
impl crate::Writable for VPERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VPERR to value 0"]
impl crate::Resettable for VPERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
