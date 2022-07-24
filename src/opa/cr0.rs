#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `AZEN` reader - desc AZEN"]
pub type AZEN_R = crate::BitReader<bool>;
#[doc = "Field `AZEN` writer - desc AZEN"]
pub type AZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `UBUFSEL` reader - desc UBUFSEL"]
pub type UBUFSEL_R = crate::BitReader<bool>;
#[doc = "Field `UBUFSEL` writer - desc UBUFSEL"]
pub type UBUFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `RESSEL` reader - desc RESSEL"]
pub type RESSEL_R = crate::BitReader<bool>;
#[doc = "Field `RESSEL` writer - desc RESSEL"]
pub type RESSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `BIASSEL` reader - desc BIASSEL"]
pub type BIASSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL` writer - desc BIASSEL"]
pub type BIASSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `NEGSEL` reader - desc NEGSEL"]
pub type NEGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEGSEL` writer - desc NEGSEL"]
pub type NEGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `POSSEL` reader - desc POSSEL"]
pub type POSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POSSEL` writer - desc POSSEL"]
pub type POSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `PGAGAIN` reader - desc PGAGAIN"]
pub type PGAGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGAGAIN` writer - desc PGAGAIN"]
pub type PGAGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `POEN` reader - desc POEN"]
pub type POEN_R = crate::BitReader<bool>;
#[doc = "Field `POEN` writer - desc POEN"]
pub type POEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `RESINMUX` reader - desc RESINMUX"]
pub type RESINMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESINMUX` writer - desc RESINMUX"]
pub type RESINMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AZEN"]
    #[inline(always)]
    pub fn azen(&self) -> AZEN_R {
        AZEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UBUFSEL"]
    #[inline(always)]
    pub fn ubufsel(&self) -> UBUFSEL_R {
        UBUFSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RESSEL"]
    #[inline(always)]
    pub fn ressel(&self) -> RESSEL_R {
        RESSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - desc BIASSEL"]
    #[inline(always)]
    pub fn biassel(&self) -> BIASSEL_R {
        BIASSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - desc NEGSEL"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc POSSEL"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - desc PGAGAIN"]
    #[inline(always)]
    pub fn pgagain(&self) -> PGAGAIN_R {
        PGAGAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc POEN"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc RESINMUX"]
    #[inline(always)]
    pub fn resinmux(&self) -> RESINMUX_R {
        RESINMUX_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - desc AZEN"]
    #[inline(always)]
    pub fn azen(&mut self) -> AZEN_W<1> {
        AZEN_W::new(self)
    }
    #[doc = "Bit 2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - desc UBUFSEL"]
    #[inline(always)]
    pub fn ubufsel(&mut self) -> UBUFSEL_W<3> {
        UBUFSEL_W::new(self)
    }
    #[doc = "Bit 4 - desc RESSEL"]
    #[inline(always)]
    pub fn ressel(&mut self) -> RESSEL_W<4> {
        RESSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - desc BIASSEL"]
    #[inline(always)]
    pub fn biassel(&mut self) -> BIASSEL_W<5> {
        BIASSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - desc NEGSEL"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W<8> {
        NEGSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - desc POSSEL"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W<10> {
        POSSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - desc PGAGAIN"]
    #[inline(always)]
    pub fn pgagain(&mut self) -> PGAGAIN_W<12> {
        PGAGAIN_W::new(self)
    }
    #[doc = "Bit 15 - desc POEN"]
    #[inline(always)]
    pub fn poen(&mut self) -> POEN_W<15> {
        POEN_W::new(self)
    }
    #[doc = "Bits 16:17 - desc RESINMUX"]
    #[inline(always)]
    pub fn resinmux(&mut self) -> RESINMUX_W<16> {
        RESINMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0x0120"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0120
    }
}
