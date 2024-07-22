#[doc = "Register `CPUMBINTCLR` reader"]
pub type R = crate::R<CpumbintclrSpec>;
#[doc = "Register `CPUMBINTCLR` writer"]
pub type W = crate::W<CpumbintclrSpec>;
#[doc = "Field `CPUMBINTCLR` reader - CPU Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type CpumbintclrR = crate::FieldReader<u32>;
#[doc = "Field `CPUMBINTCLR` writer - CPU Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
pub type CpumbintclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    pub fn cpumbintclr(&self) -> CpumbintclrR {
        CpumbintclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU Mailbox interrupt Clear. The corresponding data bit will clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpumbintclr(&mut self) -> CpumbintclrW<CpumbintclrSpec> {
        CpumbintclrW::new(self, 0)
    }
}
#[doc = "CPU Mailbox Interrupt Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpumbintclrSpec;
impl crate::RegisterSpec for CpumbintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpumbintclr::R`](R) reader structure"]
impl crate::Readable for CpumbintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpumbintclr::W`](W) writer structure"]
impl crate::Writable for CpumbintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUMBINTCLR to value 0"]
impl crate::Resettable for CpumbintclrSpec {
    const RESET_VALUE: u32 = 0;
}
