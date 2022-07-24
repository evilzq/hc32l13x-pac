#[doc = "Register `CRCH1` reader"]
pub struct R(crate::R<CRCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCH1` writer"]
pub struct W(crate::W<CRCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCH1_SPEC>;
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
impl From<crate::W<CRCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFA_CRA_BKSA` reader - desc CFA_CRA_BKSA"]
pub type CFA_CRA_BKSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFA_CRA_BKSA` writer - desc CFA_CRA_BKSA"]
pub type CFA_CRA_BKSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCH1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CFB_CRB_BKSB` reader - desc CFB_CRB_BKSB"]
pub type CFB_CRB_BKSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFB_CRB_BKSB` writer - desc CFB_CRB_BKSB"]
pub type CFB_CRB_BKSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCH1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CSA` reader - desc CSA"]
pub type CSA_R = crate::BitReader<bool>;
#[doc = "Field `CSA` writer - desc CSA"]
pub type CSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CSB` reader - desc CSB"]
pub type CSB_R = crate::BitReader<bool>;
#[doc = "Field `CSB` writer - desc CSB"]
pub type CSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `BUFEA` reader - desc BUFEA"]
pub type BUFEA_R = crate::BitReader<bool>;
#[doc = "Field `BUFEA` writer - desc BUFEA"]
pub type BUFEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `BUFEB` reader - desc BUFEB"]
pub type BUFEB_R = crate::BitReader<bool>;
#[doc = "Field `BUFEB` writer - desc BUFEB"]
pub type BUFEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CIEA` reader - desc CIEA"]
pub type CIEA_R = crate::BitReader<bool>;
#[doc = "Field `CIEA` writer - desc CIEA"]
pub type CIEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CIEB` reader - desc CIEB"]
pub type CIEB_R = crate::BitReader<bool>;
#[doc = "Field `CIEB` writer - desc CIEB"]
pub type CIEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CDEA` reader - desc CDEA"]
pub type CDEA_R = crate::BitReader<bool>;
#[doc = "Field `CDEA` writer - desc CDEA"]
pub type CDEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CDEB` reader - desc CDEB"]
pub type CDEB_R = crate::BitReader<bool>;
#[doc = "Field `CDEB` writer - desc CDEB"]
pub type CDEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CISB` reader - desc CISB"]
pub type CISB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CISB` writer - desc CISB"]
pub type CISB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCH1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CCGA` reader - desc CCGA"]
pub type CCGA_R = crate::BitReader<bool>;
#[doc = "Field `CCGA` writer - desc CCGA"]
pub type CCGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `CCGB` reader - desc CCGB"]
pub type CCGB_R = crate::BitReader<bool>;
#[doc = "Field `CCGB` writer - desc CCGB"]
pub type CCGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCH1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc CFA_CRA_BKSA"]
    #[inline(always)]
    pub fn cfa_cra_bksa(&self) -> CFA_CRA_BKSA_R {
        CFA_CRA_BKSA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc CFB_CRB_BKSB"]
    #[inline(always)]
    pub fn cfb_crb_bksb(&self) -> CFB_CRB_BKSB_R {
        CFB_CRB_BKSB_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    pub fn csb(&self) -> CSB_R {
        CSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    pub fn bufea(&self) -> BUFEA_R {
        BUFEA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    pub fn bufeb(&self) -> BUFEB_R {
        BUFEB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    pub fn ciea(&self) -> CIEA_R {
        CIEA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    pub fn cieb(&self) -> CIEB_R {
        CIEB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    pub fn cdea(&self) -> CDEA_R {
        CDEA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    pub fn cdeb(&self) -> CDEB_R {
        CDEB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    pub fn cisb(&self) -> CISB_R {
        CISB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    pub fn ccga(&self) -> CCGA_R {
        CCGA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    pub fn ccgb(&self) -> CCGB_R {
        CCGB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CFA_CRA_BKSA"]
    #[inline(always)]
    pub fn cfa_cra_bksa(&mut self) -> CFA_CRA_BKSA_W<0> {
        CFA_CRA_BKSA_W::new(self)
    }
    #[doc = "Bits 2:3 - desc CFB_CRB_BKSB"]
    #[inline(always)]
    pub fn cfb_crb_bksb(&mut self) -> CFB_CRB_BKSB_W<2> {
        CFB_CRB_BKSB_W::new(self)
    }
    #[doc = "Bit 4 - desc CSA"]
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W<4> {
        CSA_W::new(self)
    }
    #[doc = "Bit 5 - desc CSB"]
    #[inline(always)]
    pub fn csb(&mut self) -> CSB_W<5> {
        CSB_W::new(self)
    }
    #[doc = "Bit 6 - desc BUFEA"]
    #[inline(always)]
    pub fn bufea(&mut self) -> BUFEA_W<6> {
        BUFEA_W::new(self)
    }
    #[doc = "Bit 7 - desc BUFEB"]
    #[inline(always)]
    pub fn bufeb(&mut self) -> BUFEB_W<7> {
        BUFEB_W::new(self)
    }
    #[doc = "Bit 8 - desc CIEA"]
    #[inline(always)]
    pub fn ciea(&mut self) -> CIEA_W<8> {
        CIEA_W::new(self)
    }
    #[doc = "Bit 9 - desc CIEB"]
    #[inline(always)]
    pub fn cieb(&mut self) -> CIEB_W<9> {
        CIEB_W::new(self)
    }
    #[doc = "Bit 10 - desc CDEA"]
    #[inline(always)]
    pub fn cdea(&mut self) -> CDEA_W<10> {
        CDEA_W::new(self)
    }
    #[doc = "Bit 11 - desc CDEB"]
    #[inline(always)]
    pub fn cdeb(&mut self) -> CDEB_W<11> {
        CDEB_W::new(self)
    }
    #[doc = "Bits 12:13 - desc CISB"]
    #[inline(always)]
    pub fn cisb(&mut self) -> CISB_W<12> {
        CISB_W::new(self)
    }
    #[doc = "Bit 14 - desc CCGA"]
    #[inline(always)]
    pub fn ccga(&mut self) -> CCGA_W<14> {
        CCGA_W::new(self)
    }
    #[doc = "Bit 15 - desc CCGB"]
    #[inline(always)]
    pub fn ccgb(&mut self) -> CCGB_W<15> {
        CCGB_W::new(self)
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
#[doc = "desc CRCH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crch1](index.html) module"]
pub struct CRCH1_SPEC;
impl crate::RegisterSpec for CRCH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crch1::R](R) reader structure"]
impl crate::Readable for CRCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crch1::W](W) writer structure"]
impl crate::Writable for CRCH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCH1 to value 0x3000"]
impl crate::Resettable for CRCH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000
    }
}
