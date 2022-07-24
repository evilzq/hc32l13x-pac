#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_EN` reader - desc INT_EN"]
pub type INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_EN` writer - desc INT_EN"]
pub type INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `HDMA_RX` reader - desc HDMA_RX"]
pub type HDMA_RX_R = crate::BitReader<bool>;
#[doc = "Field `HDMA_RX` writer - desc HDMA_RX"]
pub type HDMA_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `HDMA_TX` reader - desc HDMA_TX"]
pub type HDMA_TX_R = crate::BitReader<bool>;
#[doc = "Field `HDMA_TX` writer - desc HDMA_TX"]
pub type HDMA_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TXEIE` reader - desc TXEIE"]
pub type TXEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXEIE` writer - desc TXEIE"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RXNEIE` reader - desc RXNEIE"]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - desc RXNEIE"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - desc INT_EN"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HDMA_RX"]
    #[inline(always)]
    pub fn hdma_rx(&self) -> HDMA_RX_R {
        HDMA_RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HDMA_TX"]
    #[inline(always)]
    pub fn hdma_tx(&self) -> HDMA_TX_R {
        HDMA_TX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc INT_EN"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W<2> {
        INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - desc HDMA_RX"]
    #[inline(always)]
    pub fn hdma_rx(&mut self) -> HDMA_RX_W<3> {
        HDMA_RX_W::new(self)
    }
    #[doc = "Bit 4 - desc HDMA_TX"]
    #[inline(always)]
    pub fn hdma_tx(&mut self) -> HDMA_TX_W<4> {
        HDMA_TX_W::new(self)
    }
    #[doc = "Bit 5 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<5> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 6 - desc RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<6> {
        RXNEIE_W::new(self)
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
#[doc = "desc CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x03"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
