#[doc = "Register `CONFB1` reader"]
pub struct R(crate::R<CONFB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFB1` writer"]
pub struct W(crate::W<CONFB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFB1_SPEC>;
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
impl From<crate::W<CONFB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSK` reader - desc MSK"]
pub type MSK_R = crate::BitReader<bool>;
#[doc = "Field `MSK` writer - desc MSK"]
pub type MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `STAT` reader - desc STAT"]
pub type STAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT` writer - desc STAT"]
pub type STAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFB1_SPEC, u8, u8, 3, O>;
#[doc = "Field `FIS_IE` reader - desc FIS_IE"]
pub type FIS_IE_R = crate::BitReader<bool>;
#[doc = "Field `FIS_IE` writer - desc FIS_IE"]
pub type FIS_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `ERR_IE` reader - desc ERR_IE"]
pub type ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `ERR_IE` writer - desc ERR_IE"]
pub type ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `RD` reader - desc RD"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - desc RD"]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `RS` reader - desc RS"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - desc RS"]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `RC` reader - desc RC"]
pub type RC_R = crate::BitReader<bool>;
#[doc = "Field `RC` writer - desc RC"]
pub type RC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `FD` reader - desc FD"]
pub type FD_R = crate::BitReader<bool>;
#[doc = "Field `FD` writer - desc FD"]
pub type FD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `FS` reader - desc FS"]
pub type FS_R = crate::BitReader<bool>;
#[doc = "Field `FS` writer - desc FS"]
pub type FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFB1_SPEC, bool, O>;
#[doc = "Field `WIDTH` reader - desc WIDTH"]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - desc WIDTH"]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFB1_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - desc MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFB1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - desc MSK"]
    #[inline(always)]
    pub fn msk(&self) -> MSK_R {
        MSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - desc FIS_IE"]
    #[inline(always)]
    pub fn fis_ie(&self) -> FIS_IE_R {
        FIS_IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc RD"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc RS"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc RC"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc FD"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc FS"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - desc WIDTH"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc MSK"]
    #[inline(always)]
    pub fn msk(&mut self) -> MSK_W<0> {
        MSK_W::new(self)
    }
    #[doc = "Bits 16:18 - desc STAT"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W<16> {
        STAT_W::new(self)
    }
    #[doc = "Bit 19 - desc FIS_IE"]
    #[inline(always)]
    pub fn fis_ie(&mut self) -> FIS_IE_W<19> {
        FIS_IE_W::new(self)
    }
    #[doc = "Bit 20 - desc ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&mut self) -> ERR_IE_W<20> {
        ERR_IE_W::new(self)
    }
    #[doc = "Bit 21 - desc RD"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W<21> {
        RD_W::new(self)
    }
    #[doc = "Bit 22 - desc RS"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W<22> {
        RS_W::new(self)
    }
    #[doc = "Bit 23 - desc RC"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W<23> {
        RC_W::new(self)
    }
    #[doc = "Bit 24 - desc FD"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W<24> {
        FD_W::new(self)
    }
    #[doc = "Bit 25 - desc FS"]
    #[inline(always)]
    pub fn fs(&mut self) -> FS_W<25> {
        FS_W::new(self)
    }
    #[doc = "Bits 26:27 - desc WIDTH"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<26> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 28:29 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<28> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CONFB1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confb1](index.html) module"]
pub struct CONFB1_SPEC;
impl crate::RegisterSpec for CONFB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confb1::R](R) reader structure"]
impl crate::Readable for CONFB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confb1::W](W) writer structure"]
impl crate::Writable for CONFB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFB1 to value 0"]
impl crate::Resettable for CONFB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
