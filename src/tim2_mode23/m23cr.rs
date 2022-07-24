#[doc = "Register `M23CR` reader"]
pub struct R(crate::R<M23CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M23CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M23CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M23CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M23CR` writer"]
pub struct W(crate::W<M23CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M23CR_SPEC>;
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
impl From<crate::W<M23CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M23CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEN` reader - desc CTEN"]
pub type CTEN_R = crate::BitReader<bool>;
#[doc = "Field `CTEN` writer - desc CTEN"]
pub type CTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - desc COMP"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `CT` reader - desc CT"]
pub type CT_R = crate::BitReader<bool>;
#[doc = "Field `CT` writer - desc CT"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `PWM2S` reader - desc PWM2S"]
pub type PWM2S_R = crate::BitReader<bool>;
#[doc = "Field `PWM2S` writer - desc PWM2S"]
pub type PWM2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M23CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUFPEN` reader - desc BUFPEN"]
pub type BUFPEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFPEN` writer - desc BUFPEN"]
pub type BUFPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `CRG` reader - desc CRG"]
pub type CRG_R = crate::BitReader<bool>;
#[doc = "Field `CRG` writer - desc CRG"]
pub type CRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `CFG` reader - desc CFG"]
pub type CFG_R = crate::BitReader<bool>;
#[doc = "Field `CFG` writer - desc CFG"]
pub type CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UIE_R = crate::BitReader<bool>;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `UDE` reader - desc UDE"]
pub type UDE_R = crate::BitReader<bool>;
#[doc = "Field `UDE` writer - desc UDE"]
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M23CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `CSG` reader - desc CSG"]
pub type CSG_R = crate::BitReader<bool>;
#[doc = "Field `CSG` writer - desc CSG"]
pub type CSG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `OCCS` reader - desc OCCS"]
pub type OCCS_R = crate::BitReader<bool>;
#[doc = "Field `OCCS` writer - desc OCCS"]
pub type OCCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `URS` reader - desc URS"]
pub type URS_R = crate::BitReader<bool>;
#[doc = "Field `URS` writer - desc URS"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `TDE` reader - desc TDE"]
pub type TDE_R = crate::BitReader<bool>;
#[doc = "Field `TDE` writer - desc TDE"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `BIE` reader - desc BIE"]
pub type BIE_R = crate::BitReader<bool>;
#[doc = "Field `BIE` writer - desc BIE"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `CIS` reader - desc CIS"]
pub type CIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CIS` writer - desc CIS"]
pub type CIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M23CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OCCE` reader - desc OCCE"]
pub type OCCE_R = crate::BitReader<bool>;
#[doc = "Field `OCCE` writer - desc OCCE"]
pub type OCCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `TG` writer - desc TG"]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `UG` writer - desc UG"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `BG` writer - desc BG"]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, M23CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CTEN"]
    #[inline(always)]
    pub fn cten(&self) -> CTEN_R {
        CTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    pub fn pwm2s(&self) -> PWM2S_R {
        PWM2S_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    pub fn bufpen(&self) -> BUFPEN_R {
        BUFPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc CRG"]
    #[inline(always)]
    pub fn crg(&self) -> CRG_R {
        CRG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CSG"]
    #[inline(always)]
    pub fn csg(&self) -> CSG_R {
        CSG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - desc CIS"]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CTEN"]
    #[inline(always)]
    pub fn cten(&mut self) -> CTEN_W<0> {
        CTEN_W::new(self)
    }
    #[doc = "Bit 1 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<1> {
        COMP_W::new(self)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<2> {
        CT_W::new(self)
    }
    #[doc = "Bit 3 - desc PWM2S"]
    #[inline(always)]
    pub fn pwm2s(&mut self) -> PWM2S_W<3> {
        PWM2S_W::new(self)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W<4> {
        PRS_W::new(self)
    }
    #[doc = "Bit 7 - desc BUFPEN"]
    #[inline(always)]
    pub fn bufpen(&mut self) -> BUFPEN_W<7> {
        BUFPEN_W::new(self)
    }
    #[doc = "Bit 8 - desc CRG"]
    #[inline(always)]
    pub fn crg(&mut self) -> CRG_W<8> {
        CRG_W::new(self)
    }
    #[doc = "Bit 9 - desc CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W<9> {
        CFG_W::new(self)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<10> {
        UIE_W::new(self)
    }
    #[doc = "Bit 11 - desc UDE"]
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<11> {
        UDE_W::new(self)
    }
    #[doc = "Bits 12:13 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<12> {
        MODE_W::new(self)
    }
    #[doc = "Bit 14 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W<14> {
        ONESHOT_W::new(self)
    }
    #[doc = "Bit 15 - desc CSG"]
    #[inline(always)]
    pub fn csg(&mut self) -> CSG_W<15> {
        CSG_W::new(self)
    }
    #[doc = "Bit 16 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&mut self) -> OCCS_W<16> {
        OCCS_W::new(self)
    }
    #[doc = "Bit 17 - desc URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<17> {
        URS_W::new(self)
    }
    #[doc = "Bit 18 - desc TDE"]
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<18> {
        TDE_W::new(self)
    }
    #[doc = "Bit 19 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<19> {
        TIE_W::new(self)
    }
    #[doc = "Bit 20 - desc BIE"]
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<20> {
        BIE_W::new(self)
    }
    #[doc = "Bits 21:22 - desc CIS"]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W<21> {
        CIS_W::new(self)
    }
    #[doc = "Bit 23 - desc OCCE"]
    #[inline(always)]
    pub fn occe(&mut self) -> OCCE_W<23> {
        OCCE_W::new(self)
    }
    #[doc = "Bit 24 - desc TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<24> {
        TG_W::new(self)
    }
    #[doc = "Bit 25 - desc UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<25> {
        UG_W::new(self)
    }
    #[doc = "Bit 26 - desc BG"]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<26> {
        BG_W::new(self)
    }
    #[doc = "Bit 27 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<27> {
        DIR_W::new(self)
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
#[doc = "desc M23CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m23cr](index.html) module"]
pub struct M23CR_SPEC;
impl crate::RegisterSpec for M23CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m23cr::R](R) reader structure"]
impl crate::Readable for M23CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m23cr::W](W) writer structure"]
impl crate::Writable for M23CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M23CR to value 0x0060_0008"]
impl crate::Resettable for M23CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0060_0008
    }
}
