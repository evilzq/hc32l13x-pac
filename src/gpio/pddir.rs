#[doc = "Register `PDDIR` reader"]
pub struct R(crate::R<PDDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDIR` writer"]
pub struct W(crate::W<PDDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDIR_SPEC>;
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
impl From<crate::W<PDDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD00` reader - desc PD00"]
pub type PD00_R = crate::BitReader<bool>;
#[doc = "Field `PD00` writer - desc PD00"]
pub type PD00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD01` reader - desc PD01"]
pub type PD01_R = crate::BitReader<bool>;
#[doc = "Field `PD01` writer - desc PD01"]
pub type PD01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD02` reader - desc PD02"]
pub type PD02_R = crate::BitReader<bool>;
#[doc = "Field `PD02` writer - desc PD02"]
pub type PD02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD03` reader - desc PD03"]
pub type PD03_R = crate::BitReader<bool>;
#[doc = "Field `PD03` writer - desc PD03"]
pub type PD03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD04` reader - desc PD04"]
pub type PD04_R = crate::BitReader<bool>;
#[doc = "Field `PD04` writer - desc PD04"]
pub type PD04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD05` reader - desc PD05"]
pub type PD05_R = crate::BitReader<bool>;
#[doc = "Field `PD05` writer - desc PD05"]
pub type PD05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD06` reader - desc PD06"]
pub type PD06_R = crate::BitReader<bool>;
#[doc = "Field `PD06` writer - desc PD06"]
pub type PD06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `PD07` reader - desc PD07"]
pub type PD07_R = crate::BitReader<bool>;
#[doc = "Field `PD07` writer - desc PD07"]
pub type PD07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PD00"]
    #[inline(always)]
    pub fn pd00(&self) -> PD00_R {
        PD00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PD01"]
    #[inline(always)]
    pub fn pd01(&self) -> PD01_R {
        PD01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PD02"]
    #[inline(always)]
    pub fn pd02(&self) -> PD02_R {
        PD02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&self) -> PD03_R {
        PD03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PD04"]
    #[inline(always)]
    pub fn pd04(&self) -> PD04_R {
        PD04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PD05"]
    #[inline(always)]
    pub fn pd05(&self) -> PD05_R {
        PD05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PD06"]
    #[inline(always)]
    pub fn pd06(&self) -> PD06_R {
        PD06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&self) -> PD07_R {
        PD07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PD00"]
    #[inline(always)]
    pub fn pd00(&mut self) -> PD00_W<0> {
        PD00_W::new(self)
    }
    #[doc = "Bit 1 - desc PD01"]
    #[inline(always)]
    pub fn pd01(&mut self) -> PD01_W<1> {
        PD01_W::new(self)
    }
    #[doc = "Bit 2 - desc PD02"]
    #[inline(always)]
    pub fn pd02(&mut self) -> PD02_W<2> {
        PD02_W::new(self)
    }
    #[doc = "Bit 3 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&mut self) -> PD03_W<3> {
        PD03_W::new(self)
    }
    #[doc = "Bit 4 - desc PD04"]
    #[inline(always)]
    pub fn pd04(&mut self) -> PD04_W<4> {
        PD04_W::new(self)
    }
    #[doc = "Bit 5 - desc PD05"]
    #[inline(always)]
    pub fn pd05(&mut self) -> PD05_W<5> {
        PD05_W::new(self)
    }
    #[doc = "Bit 6 - desc PD06"]
    #[inline(always)]
    pub fn pd06(&mut self) -> PD06_W<6> {
        PD06_W::new(self)
    }
    #[doc = "Bit 7 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&mut self) -> PD07_W<7> {
        PD07_W::new(self)
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
#[doc = "desc PDDIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddir](index.html) module"]
pub struct PDDIR_SPEC;
impl crate::RegisterSpec for PDDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddir::R](R) reader structure"]
impl crate::Readable for PDDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddir::W](W) writer structure"]
impl crate::Writable for PDDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDDIR to value 0xffff_ffff"]
impl crate::Resettable for PDDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
