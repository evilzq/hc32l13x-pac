#[doc = "Register `BCONR` reader"]
pub struct R(crate::R<BCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCONR` writer"]
pub struct W(crate::W<BCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCONR_SPEC>;
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
impl From<crate::W<BCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BENA` reader - desc BENA"]
pub type BENA_R = crate::BitReader<bool>;
#[doc = "Field `BENA` writer - desc BENA"]
pub type BENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCONR_SPEC, bool, O>;
#[doc = "Field `BENB` reader - desc BENB"]
pub type BENB_R = crate::BitReader<bool>;
#[doc = "Field `BENB` writer - desc BENB"]
pub type BENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCONR_SPEC, bool, O>;
#[doc = "Field `BENP` reader - desc BENP"]
pub type BENP_R = crate::BitReader<bool>;
#[doc = "Field `BENP` writer - desc BENP"]
pub type BENP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCONR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc BENA"]
    #[inline(always)]
    pub fn bena(&self) -> BENA_R {
        BENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc BENB"]
    #[inline(always)]
    pub fn benb(&self) -> BENB_R {
        BENB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BENP"]
    #[inline(always)]
    pub fn benp(&self) -> BENP_R {
        BENP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BENA"]
    #[inline(always)]
    pub fn bena(&mut self) -> BENA_W<0> {
        BENA_W::new(self)
    }
    #[doc = "Bit 2 - desc BENB"]
    #[inline(always)]
    pub fn benb(&mut self) -> BENB_W<2> {
        BENB_W::new(self)
    }
    #[doc = "Bit 8 - desc BENP"]
    #[inline(always)]
    pub fn benp(&mut self) -> BENP_W<8> {
        BENP_W::new(self)
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
#[doc = "desc BCONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bconr](index.html) module"]
pub struct BCONR_SPEC;
impl crate::RegisterSpec for BCONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bconr::R](R) reader structure"]
impl crate::Readable for BCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bconr::W](W) writer structure"]
impl crate::Writable for BCONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCONR to value 0"]
impl crate::Resettable for BCONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
