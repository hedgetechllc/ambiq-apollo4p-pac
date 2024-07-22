#[doc = "Register `DMADEVADDR` reader"]
pub type R = crate::R<DmadevaddrSpec>;
#[doc = "Register `DMADEVADDR` writer"]
pub type W = crate::W<DmadevaddrSpec>;
#[doc = "Field `DEVADDR` reader - SPI Device address for automated DMA transactions (both read and write)."]
pub type DevaddrR = crate::FieldReader<u32>;
#[doc = "Field `DEVADDR` writer - SPI Device address for automated DMA transactions (both read and write)."]
pub type DevaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI Device address for automated DMA transactions (both read and write)."]
    #[inline(always)]
    pub fn devaddr(&self) -> DevaddrR {
        DevaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI Device address for automated DMA transactions (both read and write)."]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DevaddrW<DmadevaddrSpec> {
        DevaddrW::new(self, 0)
    }
}
#[doc = "DMA Device Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadevaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadevaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadevaddrSpec;
impl crate::RegisterSpec for DmadevaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadevaddr::R`](R) reader structure"]
impl crate::Readable for DmadevaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmadevaddr::W`](W) writer structure"]
impl crate::Writable for DmadevaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMADEVADDR to value 0"]
impl crate::Resettable for DmadevaddrSpec {
    const RESET_VALUE: u32 = 0;
}
