#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIGN` reader - desc ALIGN"]
pub type ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `ALIGN` writer - desc ALIGN"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `THCH` reader - desc THCH"]
pub type THCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THCH` writer - desc THCH"]
pub type THCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMASQR` reader - desc DMASQR"]
pub type DMASQR_R = crate::BitReader<bool>;
#[doc = "Field `DMASQR` writer - desc DMASQR"]
pub type DMASQR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `DMAJQR` reader - desc DMAJQR"]
pub type DMAJQR_R = crate::BitReader<bool>;
#[doc = "Field `DMAJQR` writer - desc DMAJQR"]
pub type DMAJQR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RACCEN` reader - desc RACCEN"]
pub type RACCEN_R = crate::BitReader<bool>;
#[doc = "Field `RACCEN` writer - desc RACCEN"]
pub type RACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `LTCMP` reader - desc LTCMP"]
pub type LTCMP_R = crate::BitReader<bool>;
#[doc = "Field `LTCMP` writer - desc LTCMP"]
pub type LTCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `HTCMP` reader - desc HTCMP"]
pub type HTCMP_R = crate::BitReader<bool>;
#[doc = "Field `HTCMP` writer - desc HTCMP"]
pub type HTCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `REGCMP` reader - desc REGCMP"]
pub type REGCMP_R = crate::BitReader<bool>;
#[doc = "Field `REGCMP` writer - desc REGCMP"]
pub type REGCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RACCCLR` reader - desc RACCCLR"]
pub type RACCCLR_R = crate::BitReader<bool>;
#[doc = "Field `RACCCLR` writer - desc RACCCLR"]
pub type RACCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - desc THCH"]
    #[inline(always)]
    pub fn thch(&self) -> THCH_R {
        THCH_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - desc DMASQR"]
    #[inline(always)]
    pub fn dmasqr(&self) -> DMASQR_R {
        DMASQR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc DMAJQR"]
    #[inline(always)]
    pub fn dmajqr(&self) -> DMAJQR_R {
        DMAJQR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc RACCEN"]
    #[inline(always)]
    pub fn raccen(&self) -> RACCEN_R {
        RACCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc LTCMP"]
    #[inline(always)]
    pub fn ltcmp(&self) -> LTCMP_R {
        LTCMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HTCMP"]
    #[inline(always)]
    pub fn htcmp(&self) -> HTCMP_R {
        HTCMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc REGCMP"]
    #[inline(always)]
    pub fn regcmp(&self) -> REGCMP_R {
        REGCMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc RACCCLR"]
    #[inline(always)]
    pub fn raccclr(&self) -> RACCCLR_R {
        RACCCLR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<2> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 3:7 - desc THCH"]
    #[inline(always)]
    pub fn thch(&mut self) -> THCH_W<3> {
        THCH_W::new(self)
    }
    #[doc = "Bit 8 - desc DMASQR"]
    #[inline(always)]
    pub fn dmasqr(&mut self) -> DMASQR_W<8> {
        DMASQR_W::new(self)
    }
    #[doc = "Bit 9 - desc DMAJQR"]
    #[inline(always)]
    pub fn dmajqr(&mut self) -> DMAJQR_W<9> {
        DMAJQR_W::new(self)
    }
    #[doc = "Bit 10 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<10> {
        MODE_W::new(self)
    }
    #[doc = "Bit 11 - desc RACCEN"]
    #[inline(always)]
    pub fn raccen(&mut self) -> RACCEN_W<11> {
        RACCEN_W::new(self)
    }
    #[doc = "Bit 12 - desc LTCMP"]
    #[inline(always)]
    pub fn ltcmp(&mut self) -> LTCMP_W<12> {
        LTCMP_W::new(self)
    }
    #[doc = "Bit 13 - desc HTCMP"]
    #[inline(always)]
    pub fn htcmp(&mut self) -> HTCMP_W<13> {
        HTCMP_W::new(self)
    }
    #[doc = "Bit 14 - desc REGCMP"]
    #[inline(always)]
    pub fn regcmp(&mut self) -> REGCMP_W<14> {
        REGCMP_W::new(self)
    }
    #[doc = "Bit 15 - desc RACCCLR"]
    #[inline(always)]
    pub fn raccclr(&mut self) -> RACCCLR_W<15> {
        RACCCLR_W::new(self)
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
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0x8000"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
