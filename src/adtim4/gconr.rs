#[doc = "Register `GCONR` reader"]
pub struct R(crate::R<GCONR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCONR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCONR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCONR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCONR` writer"]
pub struct W(crate::W<GCONR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCONR_SPEC>;
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
impl From<crate::W<GCONR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCONR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - desc START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - desc START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCONR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCONR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CKDIV` reader - desc CKDIV"]
pub type CKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKDIV` writer - desc CKDIV"]
pub type CKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCONR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCONR_SPEC, bool, O>;
#[doc = "Field `ZMSKREV` reader - desc ZMSKREV"]
pub type ZMSKREV_R = crate::BitReader<bool>;
#[doc = "Field `ZMSKREV` writer - desc ZMSKREV"]
pub type ZMSKREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCONR_SPEC, bool, O>;
#[doc = "Field `ZMSKPOS` reader - desc ZMSKPOS"]
pub type ZMSKPOS_R = crate::BitReader<bool>;
#[doc = "Field `ZMSKPOS` writer - desc ZMSKPOS"]
pub type ZMSKPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCONR_SPEC, bool, O>;
#[doc = "Field `ZMSK` reader - desc ZMSK"]
pub type ZMSK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZMSK` writer - desc ZMSK"]
pub type ZMSK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCONR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCONR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc ZMSKREV"]
    #[inline(always)]
    pub fn zmskrev(&self) -> ZMSKREV_R {
        ZMSKREV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc ZMSKPOS"]
    #[inline(always)]
    pub fn zmskpos(&self) -> ZMSKPOS_R {
        ZMSKPOS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc ZMSK"]
    #[inline(always)]
    pub fn zmsk(&self) -> ZMSK_R {
        ZMSK_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W<4> {
        CKDIV_W::new(self)
    }
    #[doc = "Bit 8 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<8> {
        DIR_W::new(self)
    }
    #[doc = "Bit 16 - desc ZMSKREV"]
    #[inline(always)]
    pub fn zmskrev(&mut self) -> ZMSKREV_W<16> {
        ZMSKREV_W::new(self)
    }
    #[doc = "Bit 17 - desc ZMSKPOS"]
    #[inline(always)]
    pub fn zmskpos(&mut self) -> ZMSKPOS_W<17> {
        ZMSKPOS_W::new(self)
    }
    #[doc = "Bits 18:19 - desc ZMSK"]
    #[inline(always)]
    pub fn zmsk(&mut self) -> ZMSK_W<18> {
        ZMSK_W::new(self)
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
#[doc = "desc GCONR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gconr](index.html) module"]
pub struct GCONR_SPEC;
impl crate::RegisterSpec for GCONR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gconr::R](R) reader structure"]
impl crate::Readable for GCONR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gconr::W](W) writer structure"]
impl crate::Writable for GCONR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCONR to value 0x0100"]
impl crate::Resettable for GCONR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
