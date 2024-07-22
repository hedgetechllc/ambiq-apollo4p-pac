#[doc = "Register `DMACLKENABLE` reader"]
pub type R = crate::R<DmaclkenableSpec>;
#[doc = "Register `DMACLKENABLE` writer"]
pub type W = crate::W<DmaclkenableSpec>;
#[doc = "Enable the DMA clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: the DMA clock is enabled."]
    DmaE = 1,
    #[doc = "0: the DMA clock is disabled."]
    DmaD = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable the DMA clock."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::DmaE,
            false => En::DmaD,
        }
    }
    #[doc = "the DMA clock is enabled."]
    #[inline(always)]
    pub fn is_dma_e(&self) -> bool {
        *self == En::DmaE
    }
    #[doc = "the DMA clock is disabled."]
    #[inline(always)]
    pub fn is_dma_d(&self) -> bool {
        *self == En::DmaD
    }
}
#[doc = "Field `EN` writer - Enable the DMA clock."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the DMA clock is enabled."]
    #[inline(always)]
    pub fn dma_e(self) -> &'a mut crate::W<REG> {
        self.variant(En::DmaE)
    }
    #[doc = "the DMA clock is disabled."]
    #[inline(always)]
    pub fn dma_d(self) -> &'a mut crate::W<REG> {
        self.variant(En::DmaD)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the DMA clock."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the DMA clock."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DmaclkenableSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "DMA_CLK enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaclkenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaclkenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaclkenableSpec;
impl crate::RegisterSpec for DmaclkenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaclkenable::R`](R) reader structure"]
impl crate::Readable for DmaclkenableSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaclkenable::W`](W) writer structure"]
impl crate::Writable for DmaclkenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACLKENABLE to value 0"]
impl crate::Resettable for DmaclkenableSpec {
    const RESET_VALUE: u32 = 0;
}
