#[doc = "Register `DMATRIGEN` reader"]
pub type R = crate::R<DmatrigenSpec>;
#[doc = "Register `DMATRIGEN` writer"]
pub type W = crate::W<DmatrigenSpec>;
#[doc = "Field `DTHR` reader - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
pub type DthrR = crate::BitReader;
#[doc = "Field `DTHR` writer - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
pub type DthrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTHR90` reader - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
pub type Dthr90R = crate::BitReader;
#[doc = "Field `DTHR90` writer - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
pub type Dthr90W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
    #[inline(always)]
    pub fn dthr(&self) -> DthrR {
        DthrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
    #[inline(always)]
    pub fn dthr90(&self) -> Dthr90R {
        Dthr90R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only"]
    #[inline(always)]
    #[must_use]
    pub fn dthr(&mut self) -> DthrW<DmatrigenSpec> {
        DthrW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function"]
    #[inline(always)]
    #[must_use]
    pub fn dthr90(&mut self) -> Dthr90W<DmatrigenSpec> {
        Dthr90W::new(self, 1)
    }
}
#[doc = "DMA Trigger Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatrigen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatrigen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatrigenSpec;
impl crate::RegisterSpec for DmatrigenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatrigen::R`](R) reader structure"]
impl crate::Readable for DmatrigenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatrigen::W`](W) writer structure"]
impl crate::Writable for DmatrigenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATRIGEN to value 0"]
impl crate::Resettable for DmatrigenSpec {
    const RESET_VALUE: u32 = 0;
}
