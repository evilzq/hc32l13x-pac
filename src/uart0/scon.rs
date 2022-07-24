#[doc = "Register `SCON` reader"]
pub struct R(crate::R<SCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCON` writer"]
pub struct W(crate::W<SCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCON_SPEC>;
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
impl From<crate::W<SCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCIE` reader - desc RCIE"]
pub type RCIE_R = crate::BitReader<bool>;
#[doc = "Field `RCIE` writer - desc RCIE"]
pub type RCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - desc TCIE"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - desc TCIE"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `B8CONT` reader - desc B8CONT"]
pub type B8CONT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B8CONT` writer - desc B8CONT"]
pub type B8CONT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `REN` reader - desc REN"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - desc REN"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `ADRDET` reader - desc ADRDET"]
pub type ADRDET_R = crate::BitReader<bool>;
#[doc = "Field `ADRDET` writer - desc ADRDET"]
pub type ADRDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `SM` reader - desc SM"]
pub type SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SM` writer - desc SM"]
pub type SM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXEIE` reader - desc TXEIE"]
pub type TXEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXEIE` writer - desc TXEIE"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `OVER` reader - desc OVER"]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - desc OVER"]
pub type OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `PEIE` reader - desc PEIE"]
pub type PEIE_R = crate::BitReader<bool>;
#[doc = "Field `PEIE` writer - desc PEIE"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `STOPBIT` reader - desc STOPBIT"]
pub type STOPBIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOPBIT` writer - desc STOPBIT"]
pub type STOPBIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMARXEN` reader - desc DMARXEN"]
pub type DMARXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMARXEN` writer - desc DMARXEN"]
pub type DMARXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `DMATXEN` reader - desc DMATXEN"]
pub type DMATXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMATXEN` writer - desc DMATXEN"]
pub type DMATXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `RTSEN` reader - desc RTSEN"]
pub type RTSEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSEN` writer - desc RTSEN"]
pub type RTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `CTSEN` reader - desc CTSEN"]
pub type CTSEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSEN` writer - desc CTSEN"]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `CTSIE` reader - desc CTSIE"]
pub type CTSIE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIE` writer - desc CTSIE"]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `FEIE` reader - desc FEIE"]
pub type FEIE_R = crate::BitReader<bool>;
#[doc = "Field `FEIE` writer - desc FEIE"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc RCIE"]
    #[inline(always)]
    pub fn rcie(&self) -> RCIE_R {
        RCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc B8CONT"]
    #[inline(always)]
    pub fn b8cont(&self) -> B8CONT_R {
        B8CONT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc REN"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ADRDET"]
    #[inline(always)]
    pub fn adrdet(&self) -> ADRDET_R {
        ADRDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SM"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc OVER"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - desc STOPBIT"]
    #[inline(always)]
    pub fn stopbit(&self) -> STOPBIT_R {
        STOPBIT_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - desc DMARXEN"]
    #[inline(always)]
    pub fn dmarxen(&self) -> DMARXEN_R {
        DMARXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc DMATXEN"]
    #[inline(always)]
    pub fn dmatxen(&self) -> DMATXEN_R {
        DMATXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc CTSIE"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RCIE"]
    #[inline(always)]
    pub fn rcie(&mut self) -> RCIE_W<0> {
        RCIE_W::new(self)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    #[doc = "Bits 2:3 - desc B8CONT"]
    #[inline(always)]
    pub fn b8cont(&mut self) -> B8CONT_W<2> {
        B8CONT_W::new(self)
    }
    #[doc = "Bit 4 - desc REN"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<4> {
        REN_W::new(self)
    }
    #[doc = "Bit 5 - desc ADRDET"]
    #[inline(always)]
    pub fn adrdet(&mut self) -> ADRDET_W<5> {
        ADRDET_W::new(self)
    }
    #[doc = "Bits 6:7 - desc SM"]
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W<6> {
        SM_W::new(self)
    }
    #[doc = "Bit 8 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<8> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 9 - desc OVER"]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W<9> {
        OVER_W::new(self)
    }
    #[doc = "Bit 13 - desc PEIE"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<13> {
        PEIE_W::new(self)
    }
    #[doc = "Bits 14:15 - desc STOPBIT"]
    #[inline(always)]
    pub fn stopbit(&mut self) -> STOPBIT_W<14> {
        STOPBIT_W::new(self)
    }
    #[doc = "Bit 16 - desc DMARXEN"]
    #[inline(always)]
    pub fn dmarxen(&mut self) -> DMARXEN_W<16> {
        DMARXEN_W::new(self)
    }
    #[doc = "Bit 17 - desc DMATXEN"]
    #[inline(always)]
    pub fn dmatxen(&mut self) -> DMATXEN_W<17> {
        DMATXEN_W::new(self)
    }
    #[doc = "Bit 18 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W<18> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 19 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W<19> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 20 - desc CTSIE"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<20> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 21 - desc FEIE"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W<21> {
        FEIE_W::new(self)
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
#[doc = "desc SCON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scon](index.html) module"]
pub struct SCON_SPEC;
impl crate::RegisterSpec for SCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scon::R](R) reader structure"]
impl crate::Readable for SCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scon::W](W) writer structure"]
impl crate::Writable for SCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCON to value 0"]
impl crate::Resettable for SCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
