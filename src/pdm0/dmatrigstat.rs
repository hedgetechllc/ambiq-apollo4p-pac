#[doc = "Register `DMATRIGSTAT` reader"]
pub type R = crate::R<DmatrigstatSpec>;
#[doc = "Register `DMATRIGSTAT` writer"]
pub type W = crate::W<DmatrigstatSpec>;
#[doc = "Field `DTHRSTAT` reader - Triggered DMA from FIFO reaching threshold"]
pub type DthrstatR = crate::BitReader;
#[doc = "Field `DTHRSTAT` writer - Triggered DMA from FIFO reaching threshold"]
pub type DthrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHR90STAT` reader - Triggered DMA from FIFO reaching 90 percent full"]
pub type Dthr90statR = crate::BitReader;
#[doc = "Field `DTHR90STAT` writer - Triggered DMA from FIFO reaching 90 percent full"]
pub type Dthr90statW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggered DMA from FIFO reaching threshold"]
    #[inline(always)]
    pub fn dthrstat(&self) -> DthrstatR {
        DthrstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered DMA from FIFO reaching 90 percent full"]
    #[inline(always)]
    pub fn dthr90stat(&self) -> Dthr90statR {
        Dthr90statR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered DMA from FIFO reaching threshold"]
    #[inline(always)]
    #[must_use]
    pub fn dthrstat(&mut self) -> DthrstatW<DmatrigstatSpec> {
        DthrstatW::new(self, 0)
    }
    #[doc = "Bit 1 - Triggered DMA from FIFO reaching 90 percent full"]
    #[inline(always)]
    #[must_use]
    pub fn dthr90stat(&mut self) -> Dthr90statW<DmatrigstatSpec> {
        Dthr90statW::new(self, 1)
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
