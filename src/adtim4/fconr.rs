#[doc = "Register `FCONR` reader"]
pub struct R(crate::R<FCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCONR` writer"]
pub struct W(crate::W<FCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCONR_SPEC>;
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
impl From<crate::W<FCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOFIENGA` reader - desc NOFIENGA"]
pub type NOFIENGA_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENGA` writer - desc NOFIENGA"]
pub type NOFIENGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKGA` reader - desc NOFICKGA"]
pub type NOFICKGA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKGA` writer - desc NOFICKGA"]
pub type NOFICKGA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOFIENGB` reader - desc NOFIENGB"]
pub type NOFIENGB_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENGB` writer - desc NOFIENGB"]
pub type NOFIENGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKGB` reader - desc NOFICKGB"]
pub type NOFICKGB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKGB` writer - desc NOFICKGB"]
pub type NOFICKGB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOFIENTA` reader - desc NOFIENTA"]
pub type NOFIENTA_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENTA` writer - desc NOFIENTA"]
pub type NOFIENTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKTA` reader - desc NOFICKTA"]
pub type NOFICKTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKTA` writer - desc NOFICKTA"]
pub type NOFICKTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOFIENTB` reader - desc NOFIENTB"]
pub type NOFIENTB_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENTB` writer - desc NOFIENTB"]
pub type NOFIENTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKTB` reader - desc NOFICKTB"]
pub type NOFICKTB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKTB` writer - desc NOFICKTB"]
pub type NOFICKTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOFIENTC` reader - desc NOFIENTC"]
pub type NOFIENTC_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENTC` writer - desc NOFIENTC"]
pub type NOFIENTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKTC` reader - desc NOFICKTC"]
pub type NOFICKTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKTC` writer - desc NOFICKTC"]
pub type NOFICKTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOFIENTD` reader - desc NOFIENTD"]
pub type NOFIENTD_R = crate::BitReader<bool>;
#[doc = "Field `NOFIENTD` writer - desc NOFIENTD"]
pub type NOFIENTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
#[doc = "Field `NOFICKTD` reader - desc NOFICKTD"]
pub type NOFICKTD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOFICKTD` writer - desc NOFICKTD"]
pub type NOFICKTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&self) -> NOFIENGA_R {
        NOFIENGA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&self) -> NOFICKGA_R {
        NOFICKGA_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&self) -> NOFIENGB_R {
        NOFIENGB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&self) -> NOFICKGB_R {
        NOFICKGB_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 16 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&self) -> NOFIENTA_R {
        NOFIENTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&self) -> NOFICKTA_R {
        NOFICKTA_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&self) -> NOFIENTB_R {
        NOFIENTB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&self) -> NOFICKTB_R {
        NOFICKTB_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - desc NOFIENTC"]
    #[inline(always)]
    pub fn nofientc(&self) -> NOFIENTC_R {
        NOFIENTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc NOFICKTC"]
    #[inline(always)]
    pub fn noficktc(&self) -> NOFICKTC_R {
        NOFICKTC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 28 - desc NOFIENTD"]
    #[inline(always)]
    pub fn nofientd(&self) -> NOFIENTD_R {
        NOFIENTD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - desc NOFICKTD"]
    #[inline(always)]
    pub fn noficktd(&self) -> NOFICKTD_R {
        NOFICKTD_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc NOFIENGA"]
    #[inline(always)]
    pub fn nofienga(&mut self) -> NOFIENGA_W<0> {
        NOFIENGA_W::new(self)
    }
    #[doc = "Bits 1:2 - desc NOFICKGA"]
    #[inline(always)]
    pub fn nofickga(&mut self) -> NOFICKGA_W<1> {
        NOFICKGA_W::new(self)
    }
    #[doc = "Bit 4 - desc NOFIENGB"]
    #[inline(always)]
    pub fn nofiengb(&mut self) -> NOFIENGB_W<4> {
        NOFIENGB_W::new(self)
    }
    #[doc = "Bits 5:6 - desc NOFICKGB"]
    #[inline(always)]
    pub fn nofickgb(&mut self) -> NOFICKGB_W<5> {
        NOFICKGB_W::new(self)
    }
    #[doc = "Bit 16 - desc NOFIENTA"]
    #[inline(always)]
    pub fn nofienta(&mut self) -> NOFIENTA_W<16> {
        NOFIENTA_W::new(self)
    }
    #[doc = "Bits 17:18 - desc NOFICKTA"]
    #[inline(always)]
    pub fn nofickta(&mut self) -> NOFICKTA_W<17> {
        NOFICKTA_W::new(self)
    }
    #[doc = "Bit 20 - desc NOFIENTB"]
    #[inline(always)]
    pub fn nofientb(&mut self) -> NOFIENTB_W<20> {
        NOFIENTB_W::new(self)
    }
    #[doc = "Bits 21:22 - desc NOFICKTB"]
    #[inline(always)]
    pub fn noficktb(&mut self) -> NOFICKTB_W<21> {
        NOFICKTB_W::new(self)
    }
    #[doc = "Bit 24 - desc NOFIENTC"]
    #[inline(always)]
    pub fn nofientc(&mut self) -> NOFIENTC_W<24> {
        NOFIENTC_W::new(self)
    }
    #[doc = "Bits 25:26 - desc NOFICKTC"]
    #[inline(always)]
    pub fn noficktc(&mut self) -> NOFICKTC_W<25> {
        NOFICKTC_W::new(self)
    }
    #[doc = "Bit 28 - desc NOFIENTD"]
    #[inline(always)]
    pub fn nofientd(&mut self) -> NOFIENTD_W<28> {
        NOFIENTD_W::new(self)
    }
    #[doc = "Bits 29:30 - desc NOFICKTD"]
    #[inline(always)]
    pub fn noficktd(&mut self) -> NOFICKTD_W<29> {
        NOFICKTD_W::new(self)
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
#[doc = "desc FCONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fconr](index.html) module"]
pub struct FCONR_SPEC;
impl crate::RegisterSpec for FCONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fconr::R](R) reader structure"]
impl crate::Readable for FCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fconr::W](W) writer structure"]
impl crate::Writable for FCONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCONR to value 0"]
impl crate::Resettable for FCONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
