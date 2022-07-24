#[doc = "Register `PCONR` reader"]
pub struct R(crate::R<PCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONR` writer"]
pub struct W(crate::W<PCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONR_SPEC>;
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
impl From<crate::W<PCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCA` reader - desc CAPCA"]
pub type CAPCA_R = crate::BitReader<bool>;
#[doc = "Field `CAPCA` writer - desc CAPCA"]
pub type CAPCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STACA` reader - desc STACA"]
pub type STACA_R = crate::BitReader<bool>;
#[doc = "Field `STACA` writer - desc STACA"]
pub type STACA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STPCA` reader - desc STPCA"]
pub type STPCA_R = crate::BitReader<bool>;
#[doc = "Field `STPCA` writer - desc STPCA"]
pub type STPCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STASTPSA` reader - desc STASTPSA"]
pub type STASTPSA_R = crate::BitReader<bool>;
#[doc = "Field `STASTPSA` writer - desc STASTPSA"]
pub type STASTPSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `CMPCA` reader - desc CMPCA"]
pub type CMPCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPCA` writer - desc CMPCA"]
pub type CMPCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PERCA` reader - desc PERCA"]
pub type PERCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERCA` writer - desc PERCA"]
pub type PERCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUTENA` reader - desc OUTENA"]
pub type OUTENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTENA` writer - desc OUTENA"]
pub type OUTENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `DISSELA` reader - desc DISSELA"]
pub type DISSELA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISSELA` writer - desc DISSELA"]
pub type DISSELA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DISVALA` reader - desc DISVALA"]
pub type DISVALA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISVALA` writer - desc DISVALA"]
pub type DISVALA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CAPCB` reader - desc CAPCB"]
pub type CAPCB_R = crate::BitReader<bool>;
#[doc = "Field `CAPCB` writer - desc CAPCB"]
pub type CAPCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STACB` reader - desc STACB"]
pub type STACB_R = crate::BitReader<bool>;
#[doc = "Field `STACB` writer - desc STACB"]
pub type STACB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STPCB` reader - desc STPCB"]
pub type STPCB_R = crate::BitReader<bool>;
#[doc = "Field `STPCB` writer - desc STPCB"]
pub type STPCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `STASTPSB` reader - desc STASTPSB"]
pub type STASTPSB_R = crate::BitReader<bool>;
#[doc = "Field `STASTPSB` writer - desc STASTPSB"]
pub type STASTPSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `CMPCB` reader - desc CMPCB"]
pub type CMPCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPCB` writer - desc CMPCB"]
pub type CMPCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PERCB` reader - desc PERCB"]
pub type PERCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERCB` writer - desc PERCB"]
pub type PERCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OUTENB` reader - desc OUTENB"]
pub type OUTENB_R = crate::BitReader<bool>;
#[doc = "Field `OUTENB` writer - desc OUTENB"]
pub type OUTENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
#[doc = "Field `DISSELB` reader - desc DISSELB"]
pub type DISSELB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISSELB` writer - desc DISSELB"]
pub type DISSELB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DISVALB` reader - desc DISVALB"]
pub type DISVALB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISVALB` writer - desc DISVALB"]
pub type DISVALB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CAPCA"]
    #[inline(always)]
    pub fn capca(&self) -> CAPCA_R {
        CAPCA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&self) -> STACA_R {
        STACA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&self) -> STPCA_R {
        STPCA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STASTPSA"]
    #[inline(always)]
    pub fn stastpsa(&self) -> STASTPSA_R {
        STASTPSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc CMPCA"]
    #[inline(always)]
    pub fn cmpca(&self) -> CMPCA_R {
        CMPCA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc PERCA"]
    #[inline(always)]
    pub fn perca(&self) -> PERCA_R {
        PERCA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&self) -> OUTENA_R {
        OUTENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc DISSELA"]
    #[inline(always)]
    pub fn dissela(&self) -> DISSELA_R {
        DISSELA_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - desc DISVALA"]
    #[inline(always)]
    pub fn disvala(&self) -> DISVALA_R {
        DISVALA_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - desc CAPCB"]
    #[inline(always)]
    pub fn capcb(&self) -> CAPCB_R {
        CAPCB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&self) -> STACB_R {
        STACB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&self) -> STPCB_R {
        STPCB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc STASTPSB"]
    #[inline(always)]
    pub fn stastpsb(&self) -> STASTPSB_R {
        STASTPSB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - desc CMPCB"]
    #[inline(always)]
    pub fn cmpcb(&self) -> CMPCB_R {
        CMPCB_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - desc PERCB"]
    #[inline(always)]
    pub fn percb(&self) -> PERCB_R {
        PERCB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&self) -> OUTENB_R {
        OUTENB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc DISSELB"]
    #[inline(always)]
    pub fn disselb(&self) -> DISSELB_R {
        DISSELB_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - desc DISVALB"]
    #[inline(always)]
    pub fn disvalb(&self) -> DISVALB_R {
        DISVALB_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAPCA"]
    #[inline(always)]
    pub fn capca(&mut self) -> CAPCA_W<0> {
        CAPCA_W::new(self)
    }
    #[doc = "Bit 1 - desc STACA"]
    #[inline(always)]
    pub fn staca(&mut self) -> STACA_W<1> {
        STACA_W::new(self)
    }
    #[doc = "Bit 2 - desc STPCA"]
    #[inline(always)]
    pub fn stpca(&mut self) -> STPCA_W<2> {
        STPCA_W::new(self)
    }
    #[doc = "Bit 3 - desc STASTPSA"]
    #[inline(always)]
    pub fn stastpsa(&mut self) -> STASTPSA_W<3> {
        STASTPSA_W::new(self)
    }
    #[doc = "Bits 4:5 - desc CMPCA"]
    #[inline(always)]
    pub fn cmpca(&mut self) -> CMPCA_W<4> {
        CMPCA_W::new(self)
    }
    #[doc = "Bits 6:7 - desc PERCA"]
    #[inline(always)]
    pub fn perca(&mut self) -> PERCA_W<6> {
        PERCA_W::new(self)
    }
    #[doc = "Bit 8 - desc OUTENA"]
    #[inline(always)]
    pub fn outena(&mut self) -> OUTENA_W<8> {
        OUTENA_W::new(self)
    }
    #[doc = "Bits 9:10 - desc DISSELA"]
    #[inline(always)]
    pub fn dissela(&mut self) -> DISSELA_W<9> {
        DISSELA_W::new(self)
    }
    #[doc = "Bits 11:12 - desc DISVALA"]
    #[inline(always)]
    pub fn disvala(&mut self) -> DISVALA_W<11> {
        DISVALA_W::new(self)
    }
    #[doc = "Bit 16 - desc CAPCB"]
    #[inline(always)]
    pub fn capcb(&mut self) -> CAPCB_W<16> {
        CAPCB_W::new(self)
    }
    #[doc = "Bit 17 - desc STACB"]
    #[inline(always)]
    pub fn stacb(&mut self) -> STACB_W<17> {
        STACB_W::new(self)
    }
    #[doc = "Bit 18 - desc STPCB"]
    #[inline(always)]
    pub fn stpcb(&mut self) -> STPCB_W<18> {
        STPCB_W::new(self)
    }
    #[doc = "Bit 19 - desc STASTPSB"]
    #[inline(always)]
    pub fn stastpsb(&mut self) -> STASTPSB_W<19> {
        STASTPSB_W::new(self)
    }
    #[doc = "Bits 20:21 - desc CMPCB"]
    #[inline(always)]
    pub fn cmpcb(&mut self) -> CMPCB_W<20> {
        CMPCB_W::new(self)
    }
    #[doc = "Bits 22:23 - desc PERCB"]
    #[inline(always)]
    pub fn percb(&mut self) -> PERCB_W<22> {
        PERCB_W::new(self)
    }
    #[doc = "Bit 24 - desc OUTENB"]
    #[inline(always)]
    pub fn outenb(&mut self) -> OUTENB_W<24> {
        OUTENB_W::new(self)
    }
    #[doc = "Bits 25:26 - desc DISSELB"]
    #[inline(always)]
    pub fn disselb(&mut self) -> DISSELB_W<25> {
        DISSELB_W::new(self)
    }
    #[doc = "Bits 27:28 - desc DISVALB"]
    #[inline(always)]
    pub fn disvalb(&mut self) -> DISVALB_W<27> {
        DISVALB_W::new(self)
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
#[doc = "desc PCONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconr](index.html) module"]
pub struct PCONR_SPEC;
impl crate::RegisterSpec for PCONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pconr::R](R) reader structure"]
impl crate::Readable for PCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pconr::W](W) writer structure"]
impl crate::Writable for PCONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONR to value 0"]
impl crate::Resettable for PCONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
