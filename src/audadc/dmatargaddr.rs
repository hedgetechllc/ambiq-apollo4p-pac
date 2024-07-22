#[doc = "Register `DMATARGADDR` reader"]
pub type R = crate::R<DmatargaddrSpec>;
#[doc = "Register `DMATARGADDR` writer"]
pub type W = crate::W<DmatargaddrSpec>;
#[doc = "Field `LTARGADDR` reader - DMA Target Address"]
pub type LtargaddrR = crate::FieldReader<u32>;
#[doc = "Field `LTARGADDR` writer - DMA Target Address"]
pub type LtargaddrW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `UTARGADDR` reader - SRAM Target"]
pub type UtargaddrR = crate::FieldReader;
#[doc = "Field `UTARGADDR` writer - SRAM Target"]
pub type UtargaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:27 - DMA Target Address"]
    #[inline(always)]
    pub fn ltargaddr(&self) -> LtargaddrR {
        LtargaddrR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - SRAM Target"]
    #[inline(always)]
    pub fn utargaddr(&self) -> UtargaddrR {
        UtargaddrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - DMA Target Address"]
    #[inline(always)]
    #[must_use]
    pub fn ltargaddr(&mut self) -> LtargaddrW<DmatargaddrSpec> {
        LtargaddrW::new(self, 0)
    }
    #[doc = "Bits 28:31 - SRAM Target"]
    #[inline(always)]
    #[must_use]
    pub fn utargaddr(&mut self) -> UtargaddrW<DmatargaddrSpec> {
        UtargaddrW::new(self, 28)
    }
}
#[doc = "DMA Target Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatargaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatargaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatargaddrSpec;
impl crate::RegisterSpec for DmatargaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatargaddr::R`](R) reader structure"]
impl crate::Readable for DmatargaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatargaddr::W`](W) writer structure"]
impl crate::Writable for DmatargaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATARGADDR to value 0x1000_0000"]
impl crate::Resettable for DmatargaddrSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
