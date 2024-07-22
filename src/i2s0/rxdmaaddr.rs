#[doc = "Register `RXDMAADDR` reader"]
pub type R = crate::R<RxdmaaddrSpec>;
#[doc = "Register `RXDMAADDR` writer"]
pub type W = crate::W<RxdmaaddrSpec>;
#[doc = "Field `RXTARGADDR` reader - Address bits of the target byte address for source of RX write DMA."]
pub type RxtargaddrR = crate::FieldReader<u32>;
#[doc = "Field `RXTARGADDR` writer - Address bits of the target byte address for source of RX write DMA."]
pub type RxtargaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address bits of the target byte address for source of RX write DMA."]
    #[inline(always)]
    pub fn rxtargaddr(&self) -> RxtargaddrR {
        RxtargaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address bits of the target byte address for source of RX write DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rxtargaddr(&mut self) -> RxtargaddrW<RxdmaaddrSpec> {
        RxtargaddrW::new(self, 0)
    }
}
#[doc = "The address which the DMA operation will store the incoming audio samples. This address is updated as the samples are stored.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdmaaddrSpec;
impl crate::RegisterSpec for RxdmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdmaaddr::R`](R) reader structure"]
impl crate::Readable for RxdmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdmaaddr::W`](W) writer structure"]
impl crate::Writable for RxdmaaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDMAADDR to value 0"]
impl crate::Resettable for RxdmaaddrSpec {
    const RESET_VALUE: u32 = 0;
}
