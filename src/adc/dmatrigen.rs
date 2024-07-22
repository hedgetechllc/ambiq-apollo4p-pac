#[doc = "Register `DMATRIGEN` reader"]
pub type R = crate::R<DmatrigenSpec>;
#[doc = "Register `DMATRIGEN` writer"]
pub type W = crate::W<DmatrigenSpec>;
#[doc = "Field `DFIFO75` reader - Trigger DMA upon FIFO 75 percent Full"]
pub type Dfifo75R = crate::BitReader;
#[doc = "Field `DFIFO75` writer - Trigger DMA upon FIFO 75 percent Full"]
pub type Dfifo75W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFOFULL` reader - Trigger DMA upon FIFO 100 percent Full"]
pub type DfifofullR = crate::BitReader;
#[doc = "Field `DFIFOFULL` writer - Trigger DMA upon FIFO 100 percent Full"]
pub type DfifofullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    pub fn dfifo75(&self) -> Dfifo75R {
        Dfifo75R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    pub fn dfifofull(&self) -> DfifofullR {
        DfifofullR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger DMA upon FIFO 75 percent Full"]
    #[inline(always)]
    #[must_use]
    pub fn dfifo75(&mut self) -> Dfifo75W<DmatrigenSpec> {
        Dfifo75W::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger DMA upon FIFO 100 percent Full"]
    #[inline(always)]
    #[must_use]
    pub fn dfifofull(&mut self) -> DfifofullW<DmatrigenSpec> {
        DfifofullW::new(self, 1)
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
