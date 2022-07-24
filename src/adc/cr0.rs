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
#[doc = "Field `CLKDIV` reader - desc CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - desc CLKDIV"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SGLMUX` reader - desc SGLMUX"]
pub type SGLMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGLMUX` writer - desc SGLMUX"]
pub type SGLMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `REF` reader - desc REF"]
pub type REF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF` writer - desc REF"]
pub type REF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `BUF` reader - desc BUF"]
pub type BUF_R = crate::BitReader<bool>;
#[doc = "Field `BUF` writer - desc BUF"]
pub type BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SAM` reader - desc SAM"]
pub type SAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAM` writer - desc SAM"]
pub type SAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `INREFEN` reader - desc INREFEN"]
pub type INREFEN_R = crate::BitReader<bool>;
#[doc = "Field `INREFEN` writer - desc INREFEN"]
pub type INREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - desc SGLMUX"]
    #[inline(always)]
    pub fn sglmux(&self) -> SGLMUX_R {
        SGLMUX_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:10 - desc REF"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - desc BUF"]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc SAM"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc INREFEN"]
    #[inline(always)]
    pub fn inrefen(&self) -> INREFEN_R {
        INREFEN_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 2:3 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<2> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 4:8 - desc SGLMUX"]
    #[inline(always)]
    pub fn sglmux(&mut self) -> SGLMUX_W<4> {
        SGLMUX_W::new(self)
    }
    #[doc = "Bits 9:10 - desc REF"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W<9> {
        REF_W::new(self)
    }
    #[doc = "Bit 11 - desc BUF"]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W<11> {
        BUF_W::new(self)
    }
    #[doc = "Bits 12:13 - desc SAM"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W<12> {
        SAM_W::new(self)
    }
    #[doc = "Bit 14 - desc INREFEN"]
    #[inline(always)]
    pub fn inrefen(&mut self) -> INREFEN_W<14> {
        INREFEN_W::new(self)
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
#[doc = "`reset()` method sets CR0 to value 0x67f0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x67f0
    }
}
