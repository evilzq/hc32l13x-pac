#[doc = "Register `ITRIG` reader"]
pub struct R(crate::R<ITRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITRIG` writer"]
pub struct W(crate::W<ITRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITRIG_SPEC>;
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
impl From<crate::W<ITRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IAOS0S` reader - desc IAOS0S"]
pub type IAOS0S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IAOS0S` writer - desc IAOS0S"]
pub type IAOS0S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `IAOS1S` reader - desc IAOS1S"]
pub type IAOS1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IAOS1S` writer - desc IAOS1S"]
pub type IAOS1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `IAOS2S` reader - desc IAOS2S"]
pub type IAOS2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IAOS2S` writer - desc IAOS2S"]
pub type IAOS2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `IAOS3S` reader - desc IAOS3S"]
pub type IAOS3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IAOS3S` writer - desc IAOS3S"]
pub type IAOS3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITRIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc IAOS0S"]
    #[inline(always)]
    pub fn iaos0s(&self) -> IAOS0S_R {
        IAOS0S_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc IAOS1S"]
    #[inline(always)]
    pub fn iaos1s(&self) -> IAOS1S_R {
        IAOS1S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc IAOS2S"]
    #[inline(always)]
    pub fn iaos2s(&self) -> IAOS2S_R {
        IAOS2S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc IAOS3S"]
    #[inline(always)]
    pub fn iaos3s(&self) -> IAOS3S_R {
        IAOS3S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc IAOS0S"]
    #[inline(always)]
    pub fn iaos0s(&mut self) -> IAOS0S_W<0> {
        IAOS0S_W::new(self)
    }
    #[doc = "Bits 4:7 - desc IAOS1S"]
    #[inline(always)]
    pub fn iaos1s(&mut self) -> IAOS1S_W<4> {
        IAOS1S_W::new(self)
    }
    #[doc = "Bits 8:11 - desc IAOS2S"]
    #[inline(always)]
    pub fn iaos2s(&mut self) -> IAOS2S_W<8> {
        IAOS2S_W::new(self)
    }
    #[doc = "Bits 12:15 - desc IAOS3S"]
    #[inline(always)]
    pub fn iaos3s(&mut self) -> IAOS3S_W<12> {
        IAOS3S_W::new(self)
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
#[doc = "desc ITRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itrig](index.html) module"]
pub struct ITRIG_SPEC;
impl crate::RegisterSpec for ITRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itrig::R](R) reader structure"]
impl crate::Readable for ITRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itrig::W](W) writer structure"]
impl crate::Writable for ITRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITRIG to value 0"]
impl crate::Resettable for ITRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
