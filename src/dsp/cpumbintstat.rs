#[doc = "Register `CPUMBINTSTAT` reader"]
pub type R = crate::R<CpumbintstatSpec>;
#[doc = "Register `CPUMBINTSTAT` writer"]
pub type W = crate::W<CpumbintstatSpec>;
#[doc = "Field `CPUMBINTSTAT` reader - CPU CPU Mailbox interrupt status"]
pub type CpumbintstatR = crate::FieldReader<u32>;
#[doc = "Field `CPUMBINTSTAT` writer - CPU CPU Mailbox interrupt status"]
pub type CpumbintstatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU CPU Mailbox interrupt status"]
    #[inline(always)]
    pub fn cpumbintstat(&self) -> CpumbintstatR {
        CpumbintstatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU CPU Mailbox interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn cpumbintstat(&mut self) -> CpumbintstatW<CpumbintstatSpec> {
        CpumbintstatW::new(self, 0)
    }
}
#[doc = "CPU Mailbox Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpumbintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpumbintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpumbintstatSpec;
impl crate::RegisterSpec for CpumbintstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpumbintstat::R`](R) reader structure"]
impl crate::Readable for CpumbintstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cpumbintstat::W`](W) writer structure"]
impl crate::Writable for CpumbintstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUMBINTSTAT to value 0"]
impl crate::Resettable for CpumbintstatSpec {
    const RESET_VALUE: u32 = 0;
}
