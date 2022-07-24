#[doc = "Register `M1CR` reader"]
pub struct R(crate::R<M1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M1CR` writer"]
pub struct W(crate::W<M1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M1CR_SPEC>;
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
impl From<crate::W<M1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEN` reader - desc CTEN"]
pub type CTEN_R = crate::BitReader<bool>;
#[doc = "Field `CTEN` writer - desc CTEN"]
pub type CTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `CT` reader - desc CT"]
pub type CT_R = crate::BitReader<bool>;
#[doc = "Field `CT` writer - desc CT"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M1CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `EDG1ST` reader - desc EDG1ST"]
pub type EDG1ST_R = crate::BitReader<bool>;
#[doc = "Field `EDG1ST` writer - desc EDG1ST"]
pub type EDG1ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `EDG2ND` reader - desc EDG2ND"]
pub type EDG2ND_R = crate::BitReader<bool>;
#[doc = "Field `EDG2ND` writer - desc EDG2ND"]
pub type EDG2ND_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UIE_R = crate::BitReader<bool>;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M1CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type ONESHOT_R = crate::BitReader<bool>;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type ONESHOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, M1CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CTEN"]
    #[inline(always)]
    pub fn cten(&self) -> CTEN_R {
        CTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - desc EDG1ST"]
    #[inline(always)]
    pub fn edg1st(&self) -> EDG1ST_R {
        EDG1ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EDG2ND"]
    #[inline(always)]
    pub fn edg2nd(&self) -> EDG2ND_R {
        EDG2ND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<2> {
        CT_W::new(self)
    }
    #[doc = "Bits 4:6 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W<4> {
        PRS_W::new(self)
    }
    #[doc = "Bit 8 - desc EDG1ST"]
    #[inline(always)]
    pub fn edg1st(&mut self) -> EDG1ST_W<8> {
        EDG1ST_W::new(self)
    }
    #[doc = "Bit 9 - desc EDG2ND"]
    #[inline(always)]
    pub fn edg2nd(&mut self) -> EDG2ND_W<9> {
        EDG2ND_W::new(self)
    }
    #[doc = "Bit 10 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<10> {
        UIE_W::new(self)
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
#[doc = "desc M1CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1cr](index.html) module"]
pub struct M1CR_SPEC;
impl crate::RegisterSpec for M1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m1cr::R](R) reader structure"]
impl crate::Readable for M1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m1cr::W](W) writer structure"]
impl crate::Writable for M1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M1CR to value 0x0060_0008"]
impl crate::Resettable for M1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0060_0008
    }
}
