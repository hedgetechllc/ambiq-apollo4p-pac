#[doc = "Register `DMATRIGSTAT` reader"]
pub type R = crate::R<DmatrigstatSpec>;
#[doc = "Register `DMATRIGSTAT` writer"]
pub type W = crate::W<DmatrigstatSpec>;
#[doc = "Field `D75STAT` reader - Triggered DMA from FIFO 75 percent Full"]
pub type D75statR = crate::BitReader;
#[doc = "Field `D75STAT` writer - Triggered DMA from FIFO 75 percent Full"]
pub type D75statW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFULLSTAT` reader - Triggered DMA from FIFO 100 percent Full"]
pub type DfullstatR = crate::BitReader;
#[doc = "Field `DFULLSTAT` writer - Triggered DMA from FIFO 100 percent Full"]
pub type DfullstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    pub fn d75stat(&self) -> D75statR {
        D75statR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfullstat(&self) -> DfullstatR {
        DfullstatR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered DMA from FIFO 75 percent Full"]
    #[inline(always)]
    #[must_use]
    pub fn d75stat(&mut self) -> D75statW<DmatrigstatSpec> {
        D75statW::new(self, 0)
    }
    #[doc = "Bit 1 - Triggered DMA from FIFO 100 percent Full"]
    #[inline(always)]
    #[must_use]
    pub fn dfullstat(&mut self) -> DfullstatW<DmatrigstatSpec> {
        DfullstatW::new(self, 1)
    }
}
#[doc = "DMA Trigger Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatrigstatSpec;
impl crate::RegisterSpec for DmatrigstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatrigstat::R`](R) reader structure"]
impl crate::Readable for DmatrigstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatrigstat::W`](W) writer structure"]
impl crate::Writable for DmatrigstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATRIGSTAT to value 0"]
impl crate::Resettable for DmatrigstatSpec {
    const RESET_VALUE: u32 = 0;
}
