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
#[doc = "Field `TR` reader - desc TR"]
pub type TR_R = crate::BitReader<bool>;
#[doc = "Field `TR` writer - desc TR"]
pub type TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MD` reader - desc MD"]
pub type MD_R = crate::BitReader<bool>;
#[doc = "Field `MD` writer - desc MD"]
pub type MD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CT` reader - desc CT"]
pub type CT_R = crate::BitReader<bool>;
#[doc = "Field `CT` writer - desc CT"]
pub type CT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TOG_EN` reader - desc TOG_EN"]
pub type TOG_EN_R = crate::BitReader<bool>;
#[doc = "Field `TOG_EN` writer - desc TOG_EN"]
pub type TOG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TCK_SEL` reader - desc TCK_SEL"]
pub type TCK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCK_SEL` writer - desc TCK_SEL"]
pub type TCK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WT_FLAG` reader - desc WT_FLAG"]
pub type WT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `GATE` reader - desc GATE"]
pub type GATE_R = crate::BitReader<bool>;
#[doc = "Field `GATE` writer - desc GATE"]
pub type GATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GATE_P` reader - desc GATE_P"]
pub type GATE_P_R = crate::BitReader<bool>;
#[doc = "Field `GATE_P` writer - desc GATE_P"]
pub type GATE_P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TR"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc MD"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TOG_EN"]
    #[inline(always)]
    pub fn tog_en(&self) -> TOG_EN_R {
        TOG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc TCK_SEL"]
    #[inline(always)]
    pub fn tck_sel(&self) -> TCK_SEL_R {
        TCK_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - desc WT_FLAG"]
    #[inline(always)]
    pub fn wt_flag(&self) -> WT_FLAG_R {
        WT_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc GATE"]
    #[inline(always)]
    pub fn gate(&self) -> GATE_R {
        GATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc GATE_P"]
    #[inline(always)]
    pub fn gate_p(&self) -> GATE_P_R {
        GATE_P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TR"]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W<0> {
        TR_W::new(self)
    }
    #[doc = "Bit 1 - desc MD"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<1> {
        MD_W::new(self)
    }
    #[doc = "Bit 2 - desc CT"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<2> {
        CT_W::new(self)
    }
    #[doc = "Bit 3 - desc TOG_EN"]
    #[inline(always)]
    pub fn tog_en(&mut self) -> TOG_EN_W<3> {
        TOG_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - desc TCK_SEL"]
    #[inline(always)]
    pub fn tck_sel(&mut self) -> TCK_SEL_W<4> {
        TCK_SEL_W::new(self)
    }
    #[doc = "Bit 8 - desc GATE"]
    #[inline(always)]
    pub fn gate(&mut self) -> GATE_W<8> {
        GATE_W::new(self)
    }
    #[doc = "Bit 9 - desc GATE_P"]
    #[inline(always)]
    pub fn gate_p(&mut self) -> GATE_P_W<9> {
        GATE_P_W::new(self)
    }
    #[doc = "Bit 10 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<10> {
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
#[doc = "`reset()` method sets CR to value 0x08"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
