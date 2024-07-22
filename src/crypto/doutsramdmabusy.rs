#[doc = "Register `DOUTSRAMDMABUSY` reader"]
pub type R = crate::R<DoutsramdmabusySpec>;
#[doc = "Register `DOUTSRAMDMABUSY` writer"]
pub type W = crate::W<DoutsramdmabusySpec>;
#[doc = "DOUT SRAM DMA busy status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: all data was written to SRAM."]
    DataSram = 0,
    #[doc = "1: DOUT SRAM DMA busy."]
    DmaBusy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - DOUT SRAM DMA busy status."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::DataSram,
            true => Busy::DmaBusy,
        }
    }
    #[doc = "all data was written to SRAM."]
    #[inline(always)]
    pub fn is_data_sram(&self) -> bool {
        *self == Busy::DataSram
    }
    #[doc = "DOUT SRAM DMA busy."]
    #[inline(always)]
    pub fn is_dma_busy(&self) -> bool {
        *self == Busy::DmaBusy
    }
}
#[doc = "Field `BUSY` writer - DOUT SRAM DMA busy status."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG, Busy>;
impl<'a, REG> BusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "all data was written to SRAM."]
    #[inline(always)]
    pub fn data_sram(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::DataSram)
    }
    #[doc = "DOUT SRAM DMA busy."]
    #[inline(always)]
    pub fn dma_busy(self) -> &'a mut crate::W<REG> {
        self.variant(Busy::DmaBusy)
    }
}
impl R {
    #[doc = "Bit 0 - DOUT SRAM DMA busy status."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DOUT SRAM DMA busy status."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<DoutsramdmabusySpec> {
        BusyW::new(self, 0)
    }
}
#[doc = "This register holds the status of the SRAM DMA DOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`doutsramdmabusy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutsramdmabusy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutsramdmabusySpec;
impl crate::RegisterSpec for DoutsramdmabusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutsramdmabusy::R`](R) reader structure"]
impl crate::Readable for DoutsramdmabusySpec {}
#[doc = "`write(|w| ..)` method takes [`doutsramdmabusy::W`](W) writer structure"]
impl crate::Writable for DoutsramdmabusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTSRAMDMABUSY to value 0"]
impl crate::Resettable for DoutsramdmabusySpec {
    const RESET_VALUE: u32 = 0;
}
