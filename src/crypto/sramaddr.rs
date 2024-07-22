#[doc = "Register `SRAMADDR` reader"]
pub type R = crate::R<SramaddrSpec>;
#[doc = "Register `SRAMADDR` writer"]
pub type W = crate::W<SramaddrSpec>;
#[doc = "Field `SRAMADDR` reader - SRAM starting address"]
pub type SramaddrR = crate::FieldReader<u16>;
#[doc = "Field `SRAMADDR` writer - SRAM starting address"]
pub type SramaddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - SRAM starting address"]
    #[inline(always)]
    pub fn sramaddr(&self) -> SramaddrR {
        SramaddrR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - SRAM starting address"]
    #[inline(always)]
    #[must_use]
    pub fn sramaddr(&mut self) -> SramaddrW<SramaddrSpec> {
        SramaddrW::new(self, 0)
    }
}
#[doc = "first address given to SRAM DMA for read_write transactions from SRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sramaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramaddrSpec;
impl crate::RegisterSpec for SramaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramaddr::R`](R) reader structure"]
impl crate::Readable for SramaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sramaddr::W`](W) writer structure"]
impl crate::Writable for SramaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMADDR to value 0"]
impl crate::Resettable for SramaddrSpec {
    const RESET_VALUE: u32 = 0;
}
