#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALT` reader - desc HALT"]
pub type HALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HALT` writer - desc HALT"]
pub type HALT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRIO` reader - desc PRIO"]
pub type PRIO_R = crate::BitReader<bool>;
#[doc = "Field `PRIO` writer - desc PRIO"]
pub type PRIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
#[doc = "Field `ST` reader - desc ST"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - desc ST"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:27 - desc HALT"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - desc ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - desc HALT"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W<24> {
        HALT_W::new(self)
    }
    #[doc = "Bit 28 - desc PRIO"]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<28> {
        PRIO_W::new(self)
    }
    #[doc = "Bit 30 - desc ST"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<30> {
        ST_W::new(self)
    }
    #[doc = "Bit 31 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
