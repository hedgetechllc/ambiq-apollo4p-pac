#[doc = "Register `RXFIFOSTATUS` reader"]
pub type R = crate::R<RxfifostatusSpec>;
#[doc = "Register `RXFIFOSTATUS` writer"]
pub type W = crate::W<RxfifostatusSpec>;
#[doc = "Field `RXSAMPLECNT` reader - The count of the number of samples currently in the receive FIFO."]
pub type RxsamplecntR = crate::FieldReader<u32>;
#[doc = "Field `RXSAMPLECNT` writer - The count of the number of samples currently in the receive FIFO."]
pub type RxsamplecntW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `RXEMPTY` reader - Receive FIFO empty bit. a 1 indicates the receive FIFO is empty."]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXEMPTY` writer - Receive FIFO empty bit. a 1 indicates the receive FIFO is empty."]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - The count of the number of samples currently in the receive FIFO."]
    #[inline(always)]
    pub fn rxsamplecnt(&self) -> RxsamplecntR {
        RxsamplecntR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - Receive FIFO empty bit. a 1 indicates the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - The count of the number of samples currently in the receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rxsamplecnt(&mut self) -> RxsamplecntW<RxfifostatusSpec> {
        RxsamplecntW::new(self, 0)
    }
    #[doc = "Bit 28 - Receive FIFO empty bit. a 1 indicates the receive FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn rxempty(&mut self) -> RxemptyW<RxfifostatusSpec> {
        RxemptyW::new(self, 28)
    }
}
#[doc = "Holds the number of samples currently in the receive FIFO, and the empty condition flag\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifostatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifostatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifostatusSpec;
impl crate::RegisterSpec for RxfifostatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifostatus::R`](R) reader structure"]
impl crate::Readable for RxfifostatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifostatus::W`](W) writer structure"]
impl crate::Writable for RxfifostatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFIFOSTATUS to value 0"]
impl crate::Resettable for RxfifostatusSpec {
    const RESET_VALUE: u32 = 0;
}
