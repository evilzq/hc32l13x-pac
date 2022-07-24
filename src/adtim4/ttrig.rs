#[doc = "Register `TTRIG` reader"]
pub struct R(crate::R<TTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTRIG` writer"]
pub struct W(crate::W<TTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTRIG_SPEC>;
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
impl From<crate::W<TTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGAS` reader - desc TRIGAS"]
pub type TRIGAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGAS` writer - desc TRIGAS"]
pub type TRIGAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIGBS` reader - desc TRIGBS"]
pub type TRIGBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGBS` writer - desc TRIGBS"]
pub type TRIGBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIGCS` reader - desc TRIGCS"]
pub type TRIGCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGCS` writer - desc TRIGCS"]
pub type TRIGCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIGDS` reader - desc TRIGDS"]
pub type TRIGDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGDS` writer - desc TRIGDS"]
pub type TRIGDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTRIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc TRIGAS"]
    #[inline(always)]
    pub fn trigas(&self) -> TRIGAS_R {
        TRIGAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc TRIGBS"]
    #[inline(always)]
    pub fn trigbs(&self) -> TRIGBS_R {
        TRIGBS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc TRIGCS"]
    #[inline(always)]
    pub fn trigcs(&self) -> TRIGCS_R {
        TRIGCS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc TRIGDS"]
    #[inline(always)]
    pub fn trigds(&self) -> TRIGDS_R {
        TRIGDS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc TRIGAS"]
    #[inline(always)]
    pub fn trigas(&mut self) -> TRIGAS_W<0> {
        TRIGAS_W::new(self)
    }
    #[doc = "Bits 4:7 - desc TRIGBS"]
    #[inline(always)]
    pub fn trigbs(&mut self) -> TRIGBS_W<4> {
        TRIGBS_W::new(self)
    }
    #[doc = "Bits 8:11 - desc TRIGCS"]
    #[inline(always)]
    pub fn trigcs(&mut self) -> TRIGCS_W<8> {
        TRIGCS_W::new(self)
    }
    #[doc = "Bits 12:15 - desc TRIGDS"]
    #[inline(always)]
    pub fn trigds(&mut self) -> TRIGDS_W<12> {
        TRIGDS_W::new(self)
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
#[doc = "desc TTRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttrig](index.html) module"]
pub struct TTRIG_SPEC;
impl crate::RegisterSpec for TTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttrig::R](R) reader structure"]
impl crate::Readable for TTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttrig::W](W) writer structure"]
impl crate::Writable for TTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTRIG to value 0"]
impl crate::Resettable for TTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
