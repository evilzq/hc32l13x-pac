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
#[doc = "Field `PRDS` reader - desc PRDS"]
pub type PRDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRDS` writer - desc PRDS"]
pub type PRDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `AMPM` reader - desc AMPM"]
pub type AMPM_R = crate::BitReader<bool>;
#[doc = "Field `AMPM` writer - desc AMPM"]
pub type AMPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `HZ1OE` reader - desc HZ1OE"]
pub type HZ1OE_R = crate::BitReader<bool>;
#[doc = "Field `HZ1OE` writer - desc HZ1OE"]
pub type HZ1OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `HZ1SEL` reader - desc HZ1SEL"]
pub type HZ1SEL_R = crate::BitReader<bool>;
#[doc = "Field `HZ1SEL` writer - desc HZ1SEL"]
pub type HZ1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `PRDX` reader - desc PRDX"]
pub type PRDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRDX` writer - desc PRDX"]
pub type PRDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `PRDSEL` reader - desc PRDSEL"]
pub type PRDSEL_R = crate::BitReader<bool>;
#[doc = "Field `PRDSEL` writer - desc PRDSEL"]
pub type PRDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - desc PRDS"]
    #[inline(always)]
    pub fn prds(&self) -> PRDS_R {
        PRDS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc AMPM"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HZ1OE"]
    #[inline(always)]
    pub fn hz1oe(&self) -> HZ1OE_R {
        HZ1OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HZ1SEL"]
    #[inline(always)]
    pub fn hz1sel(&self) -> HZ1SEL_R {
        HZ1SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - desc PRDX"]
    #[inline(always)]
    pub fn prdx(&self) -> PRDX_R {
        PRDX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - desc PRDSEL"]
    #[inline(always)]
    pub fn prdsel(&self) -> PRDSEL_R {
        PRDSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PRDS"]
    #[inline(always)]
    pub fn prds(&mut self) -> PRDS_W<0> {
        PRDS_W::new(self)
    }
    #[doc = "Bit 3 - desc AMPM"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AMPM_W<3> {
        AMPM_W::new(self)
    }
    #[doc = "Bit 5 - desc HZ1OE"]
    #[inline(always)]
    pub fn hz1oe(&mut self) -> HZ1OE_W<5> {
        HZ1OE_W::new(self)
    }
    #[doc = "Bit 6 - desc HZ1SEL"]
    #[inline(always)]
    pub fn hz1sel(&mut self) -> HZ1SEL_W<6> {
        HZ1SEL_W::new(self)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<7> {
        START_W::new(self)
    }
    #[doc = "Bits 8:13 - desc PRDX"]
    #[inline(always)]
    pub fn prdx(&mut self) -> PRDX_W<8> {
        PRDX_W::new(self)
    }
    #[doc = "Bit 14 - desc PRDSEL"]
    #[inline(always)]
    pub fn prdsel(&mut self) -> PRDSEL_W<14> {
        PRDSEL_W::new(self)
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
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
