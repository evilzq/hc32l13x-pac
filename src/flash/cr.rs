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
#[doc = "Field `OP` reader - desc OP"]
pub type OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OP` writer - desc OP"]
pub type OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAIT` reader - desc WAIT"]
pub type WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT` writer - desc WAIT"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `IE` reader - desc IE"]
pub type IE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IE` writer - desc IE"]
pub type IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DPSTB_EN` reader - desc DPSTB_EN"]
pub type DPSTB_EN_R = crate::BitReader<bool>;
#[doc = "Field `DPSTB_EN` writer - desc DPSTB_EN"]
pub type DPSTB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:1 - desc OP"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 9 - desc DPSTB_EN"]
    #[inline(always)]
    pub fn dpstb_en(&self) -> DPSTB_EN_R {
        DPSTB_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc OP"]
    #[inline(always)]
    pub fn op(&mut self) -> OP_W<0> {
        OP_W::new(self)
    }
    #[doc = "Bits 2:3 - desc WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<2> {
        WAIT_W::new(self)
    }
    #[doc = "Bits 5:6 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<5> {
        IE_W::new(self)
    }
    #[doc = "Bit 9 - desc DPSTB_EN"]
    #[inline(always)]
    pub fn dpstb_en(&mut self) -> DPSTB_EN_W<9> {
        DPSTB_EN_W::new(self)
    }
    #[doc = "Bits 10:31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<10> {
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
