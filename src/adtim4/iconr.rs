#[doc = "Register `ICONR` reader"]
pub struct R(crate::R<ICONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICONR` writer"]
pub struct W(crate::W<ICONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICONR_SPEC>;
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
impl From<crate::W<ICONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTENA` reader - desc INTENA"]
pub type INTENA_R = crate::BitReader<bool>;
#[doc = "Field `INTENA` writer - desc INTENA"]
pub type INTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENB` reader - desc INTENB"]
pub type INTENB_R = crate::BitReader<bool>;
#[doc = "Field `INTENB` writer - desc INTENB"]
pub type INTENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENC` reader - desc INTENC"]
pub type INTENC_R = crate::BitReader<bool>;
#[doc = "Field `INTENC` writer - desc INTENC"]
pub type INTENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTEND` reader - desc INTEND"]
pub type INTEND_R = crate::BitReader<bool>;
#[doc = "Field `INTEND` writer - desc INTEND"]
pub type INTEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENOVF` reader - desc INTENOVF"]
pub type INTENOVF_R = crate::BitReader<bool>;
#[doc = "Field `INTENOVF` writer - desc INTENOVF"]
pub type INTENOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENUDF` reader - desc INTENUDF"]
pub type INTENUDF_R = crate::BitReader<bool>;
#[doc = "Field `INTENUDF` writer - desc INTENUDF"]
pub type INTENUDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENDE` reader - desc INTENDE"]
pub type INTENDE_R = crate::BitReader<bool>;
#[doc = "Field `INTENDE` writer - desc INTENDE"]
pub type INTENDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSAML` reader - desc INTENSAML"]
pub type INTENSAML_R = crate::BitReader<bool>;
#[doc = "Field `INTENSAML` writer - desc INTENSAML"]
pub type INTENSAML_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSAMH` reader - desc INTENSAMH"]
pub type INTENSAMH_R = crate::BitReader<bool>;
#[doc = "Field `INTENSAMH` writer - desc INTENSAMH"]
pub type INTENSAMH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSAU` reader - desc INTENSAU"]
pub type INTENSAU_R = crate::BitReader<bool>;
#[doc = "Field `INTENSAU` writer - desc INTENSAU"]
pub type INTENSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSAD` reader - desc INTENSAD"]
pub type INTENSAD_R = crate::BitReader<bool>;
#[doc = "Field `INTENSAD` writer - desc INTENSAD"]
pub type INTENSAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSBU` reader - desc INTENSBU"]
pub type INTENSBU_R = crate::BitReader<bool>;
#[doc = "Field `INTENSBU` writer - desc INTENSBU"]
pub type INTENSBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `INTENSBD` reader - desc INTENSBD"]
pub type INTENSBD_R = crate::BitReader<bool>;
#[doc = "Field `INTENSBD` writer - desc INTENSBD"]
pub type INTENSBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&self) -> INTENA_R {
        INTENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&self) -> INTENB_R {
        INTENB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc INTENC"]
    #[inline(always)]
    pub fn intenc(&self) -> INTENC_R {
        INTENC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc INTEND"]
    #[inline(always)]
    pub fn intend(&self) -> INTEND_R {
        INTEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INTENOVF"]
    #[inline(always)]
    pub fn intenovf(&self) -> INTENOVF_R {
        INTENOVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc INTENUDF"]
    #[inline(always)]
    pub fn intenudf(&self) -> INTENUDF_R {
        INTENUDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc INTENDE"]
    #[inline(always)]
    pub fn intende(&self) -> INTENDE_R {
        INTENDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - desc INTENSAML"]
    #[inline(always)]
    pub fn intensaml(&self) -> INTENSAML_R {
        INTENSAML_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc INTENSAMH"]
    #[inline(always)]
    pub fn intensamh(&self) -> INTENSAMH_R {
        INTENSAMH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc INTENSAU"]
    #[inline(always)]
    pub fn intensau(&self) -> INTENSAU_R {
        INTENSAU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc INTENSAD"]
    #[inline(always)]
    pub fn intensad(&self) -> INTENSAD_R {
        INTENSAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc INTENSBU"]
    #[inline(always)]
    pub fn intensbu(&self) -> INTENSBU_R {
        INTENSBU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc INTENSBD"]
    #[inline(always)]
    pub fn intensbd(&self) -> INTENSBD_R {
        INTENSBD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INTENA"]
    #[inline(always)]
    pub fn intena(&mut self) -> INTENA_W<0> {
        INTENA_W::new(self)
    }
    #[doc = "Bit 1 - desc INTENB"]
    #[inline(always)]
    pub fn intenb(&mut self) -> INTENB_W<1> {
        INTENB_W::new(self)
    }
    #[doc = "Bit 2 - desc INTENC"]
    #[inline(always)]
    pub fn intenc(&mut self) -> INTENC_W<2> {
        INTENC_W::new(self)
    }
    #[doc = "Bit 3 - desc INTEND"]
    #[inline(always)]
    pub fn intend(&mut self) -> INTEND_W<3> {
        INTEND_W::new(self)
    }
    #[doc = "Bit 6 - desc INTENOVF"]
    #[inline(always)]
    pub fn intenovf(&mut self) -> INTENOVF_W<6> {
        INTENOVF_W::new(self)
    }
    #[doc = "Bit 7 - desc INTENUDF"]
    #[inline(always)]
    pub fn intenudf(&mut self) -> INTENUDF_W<7> {
        INTENUDF_W::new(self)
    }
    #[doc = "Bit 8 - desc INTENDE"]
    #[inline(always)]
    pub fn intende(&mut self) -> INTENDE_W<8> {
        INTENDE_W::new(self)
    }
    #[doc = "Bit 14 - desc INTENSAML"]
    #[inline(always)]
    pub fn intensaml(&mut self) -> INTENSAML_W<14> {
        INTENSAML_W::new(self)
    }
    #[doc = "Bit 15 - desc INTENSAMH"]
    #[inline(always)]
    pub fn intensamh(&mut self) -> INTENSAMH_W<15> {
        INTENSAMH_W::new(self)
    }
    #[doc = "Bit 16 - desc INTENSAU"]
    #[inline(always)]
    pub fn intensau(&mut self) -> INTENSAU_W<16> {
        INTENSAU_W::new(self)
    }
    #[doc = "Bit 17 - desc INTENSAD"]
    #[inline(always)]
    pub fn intensad(&mut self) -> INTENSAD_W<17> {
        INTENSAD_W::new(self)
    }
    #[doc = "Bit 18 - desc INTENSBU"]
    #[inline(always)]
    pub fn intensbu(&mut self) -> INTENSBU_W<18> {
        INTENSBU_W::new(self)
    }
    #[doc = "Bit 19 - desc INTENSBD"]
    #[inline(always)]
    pub fn intensbd(&mut self) -> INTENSBD_W<19> {
        INTENSBD_W::new(self)
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
#[doc = "desc ICONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iconr](index.html) module"]
pub struct ICONR_SPEC;
impl crate::RegisterSpec for ICONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iconr::R](R) reader structure"]
impl crate::Readable for ICONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iconr::W](W) writer structure"]
impl crate::Writable for ICONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICONR to value 0"]
impl crate::Resettable for ICONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
