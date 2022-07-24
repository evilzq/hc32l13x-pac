#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVDEN` reader - desc LVDEN"]
pub type LVDEN_R = crate::BitReader<bool>;
#[doc = "Field `LVDEN` writer - desc LVDEN"]
pub type LVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ACT` reader - desc ACT"]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - desc ACT"]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SOURCE_SEL` reader - desc SOURCE_SEL"]
pub type SOURCE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCE_SEL` writer - desc SOURCE_SEL"]
pub type SOURCE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VTDS` reader - desc VTDS"]
pub type VTDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTDS` writer - desc VTDS"]
pub type VTDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLTEN` reader - desc FLTEN"]
pub type FLTEN_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN` writer - desc FLTEN"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DEBOUNCE_TIME` reader - desc DEBOUNCE_TIME"]
pub type DEBOUNCE_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBOUNCE_TIME` writer - desc DEBOUNCE_TIME"]
pub type DEBOUNCE_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FTEN` reader - desc FTEN"]
pub type FTEN_R = crate::BitReader<bool>;
#[doc = "Field `FTEN` writer - desc FTEN"]
pub type FTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RTEN` reader - desc RTEN"]
pub type RTEN_R = crate::BitReader<bool>;
#[doc = "Field `RTEN` writer - desc RTEN"]
pub type RTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HTEN` reader - desc HTEN"]
pub type HTEN_R = crate::BitReader<bool>;
#[doc = "Field `HTEN` writer - desc HTEN"]
pub type HTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc LVDEN"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ACT"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc SOURCE_SEL"]
    #[inline(always)]
    pub fn source_sel(&self) -> SOURCE_SEL_R {
        SOURCE_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc VTDS"]
    #[inline(always)]
    pub fn vtds(&self) -> VTDS_R {
        VTDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - desc DEBOUNCE_TIME"]
    #[inline(always)]
    pub fn debounce_time(&self) -> DEBOUNCE_TIME_R {
        DEBOUNCE_TIME_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - desc FTEN"]
    #[inline(always)]
    pub fn ften(&self) -> FTEN_R {
        FTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc RTEN"]
    #[inline(always)]
    pub fn rten(&self) -> RTEN_R {
        RTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HTEN"]
    #[inline(always)]
    pub fn hten(&self) -> HTEN_R {
        HTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc LVDEN"]
    #[inline(always)]
    pub fn lvden(&mut self) -> LVDEN_W<0> {
        LVDEN_W::new(self)
    }
    #[doc = "Bit 1 - desc ACT"]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<1> {
        ACT_W::new(self)
    }
    #[doc = "Bits 2:3 - desc SOURCE_SEL"]
    #[inline(always)]
    pub fn source_sel(&mut self) -> SOURCE_SEL_W<2> {
        SOURCE_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc VTDS"]
    #[inline(always)]
    pub fn vtds(&mut self) -> VTDS_W<4> {
        VTDS_W::new(self)
    }
    #[doc = "Bit 8 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<8> {
        FLTEN_W::new(self)
    }
    #[doc = "Bits 9:11 - desc DEBOUNCE_TIME"]
    #[inline(always)]
    pub fn debounce_time(&mut self) -> DEBOUNCE_TIME_W<9> {
        DEBOUNCE_TIME_W::new(self)
    }
    #[doc = "Bit 12 - desc FTEN"]
    #[inline(always)]
    pub fn ften(&mut self) -> FTEN_W<12> {
        FTEN_W::new(self)
    }
    #[doc = "Bit 13 - desc RTEN"]
    #[inline(always)]
    pub fn rten(&mut self) -> RTEN_W<13> {
        RTEN_W::new(self)
    }
    #[doc = "Bit 14 - desc HTEN"]
    #[inline(always)]
    pub fn hten(&mut self) -> HTEN_W<14> {
        HTEN_W::new(self)
    }
    #[doc = "Bit 15 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<15> {
        IE_W::new(self)
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
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
