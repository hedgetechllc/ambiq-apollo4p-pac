#[doc = "Register `FIFOADD` reader"]
pub type R = crate::R<FifoaddSpec>;
#[doc = "Register `FIFOADD` writer"]
pub type W = crate::W<FifoaddSpec>;
#[doc = "Field `INFIFOADD` reader - Sets the start address of the selected IN endpoint FIFO."]
pub type InfifoaddR = crate::FieldReader<u16>;
#[doc = "Field `INFIFOADD` writer - Sets the start address of the selected IN endpoint FIFO."]
pub type InfifoaddW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `OUTFIFOADD` reader - Sets the start address of the selected OUT endpoint FIFO."]
pub type OutfifoaddR = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFOADD` writer - Sets the start address of the selected OUT endpoint FIFO."]
pub type OutfifoaddW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Sets the start address of the selected IN endpoint FIFO."]
    #[inline(always)]
    pub fn infifoadd(&self) -> InfifoaddR {
        InfifoaddR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Sets the start address of the selected OUT endpoint FIFO."]
    #[inline(always)]
    pub fn outfifoadd(&self) -> OutfifoaddR {
        OutfifoaddR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Sets the start address of the selected IN endpoint FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn infifoadd(&mut self) -> InfifoaddW<FifoaddSpec> {
        InfifoaddW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Sets the start address of the selected OUT endpoint FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifoadd(&mut self) -> OutfifoaddW<FifoaddSpec> {
        OutfifoaddW::new(self, 16)
    }
}
#[doc = "Sets the start address of the selected IN and OUT endpoint FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoaddSpec;
impl crate::RegisterSpec for FifoaddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoadd::R`](R) reader structure"]
impl crate::Readable for FifoaddSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoadd::W`](W) writer structure"]
impl crate::Writable for FifoaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOADD to value 0"]
impl crate::Resettable for FifoaddSpec {
    const RESET_VALUE: u32 = 0;
}
