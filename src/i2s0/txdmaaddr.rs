#[doc = "Register `TXDMAADDR` reader"]
pub type R = crate::R<TxdmaaddrSpec>;
#[doc = "Register `TXDMAADDR` writer"]
pub type W = crate::W<TxdmaaddrSpec>;
#[doc = "Field `TXTARGADDR` reader - Address bits of the target byte address for source of TX write DMA."]
pub type TxtargaddrR = crate::FieldReader<u32>;
#[doc = "Field `TXTARGADDR` writer - Address bits of the target byte address for source of TX write DMA."]
pub type TxtargaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address bits of the target byte address for source of TX write DMA."]
    #[inline(always)]
    pub fn txtargaddr(&self) -> TxtargaddrR {
        TxtargaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address bits of the target byte address for source of TX write DMA."]
    #[inline(always)]
    #[must_use]
    pub fn txtargaddr(&mut self) -> TxtargaddrW<TxdmaaddrSpec> {
        TxtargaddrW::new(self, 0)
    }
}
#[doc = "The address which the DMA operation will fetch the audio samples. This address is updated as the samples are stored.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdmaaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdmaaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdmaaddrSpec;
impl crate::RegisterSpec for TxdmaaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdmaaddr::R`](R) reader structure"]
impl crate::Readable for TxdmaaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdmaaddr::W`](W) writer structure"]
impl crate::Writable for TxdmaaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDMAADDR to value 0"]
impl crate::Resettable for TxdmaaddrSpec {
    const RESET_VALUE: u32 = 0;
}
