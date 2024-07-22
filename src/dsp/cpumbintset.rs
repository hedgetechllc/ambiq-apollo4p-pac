#[doc = "Register `CPUMBINTSET` reader"]
pub type R = crate::R<CpumbintsetSpec>;
#[doc = "Register `CPUMBINTSET` writer"]
pub type W = crate::W<CpumbintsetSpec>;
#[doc = "Field `CPUMBINTSET` reader - CPU Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type CpumbintsetR = crate::FieldReader<u32>;
#[doc = "Field `CPUMBINTSET` writer - CPU Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
pub type CpumbintsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    pub fn cpumbintset(&self) -> CpumbintsetR {
        CpumbintsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU Mailbox interrupt Set. The corresponding data bit will set the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cpumbintset(&mut self) -> CpumbintsetW<CpumbintsetSpec> {
        CpumbintsetW::new(self, 0)
    }
}
#[doc = "CPU Mailbox Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpumbintsetSpec;
impl crate::RegisterSpec for CpumbintsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpumbintset::R`](R) reader structure"]
impl crate::Readable for CpumbintsetSpec {}
#[doc = "`write(|w| ..)` method takes [`cpumbintset::W`](W) writer structure"]
impl crate::Writable for CpumbintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUMBINTSET to value 0"]
impl crate::Resettable for CpumbintsetSpec {
    const RESET_VALUE: u32 = 0;
}
