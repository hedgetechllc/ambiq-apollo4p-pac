#[doc = "Register `CMDLISTADDR` reader"]
pub type R = crate::R<CmdlistaddrSpec>;
#[doc = "Register `CMDLISTADDR` writer"]
pub type W = crate::W<CmdlistaddrSpec>;
#[doc = "Field `BASEPTR` reader - Command list base pointer."]
pub type BaseptrR = crate::FieldReader<u32>;
#[doc = "Field `BASEPTR` writer - Command list base pointer."]
pub type BaseptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command list base pointer."]
    #[inline(always)]
    pub fn baseptr(&self) -> BaseptrR {
        BaseptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command list base pointer."]
    #[inline(always)]
    #[must_use]
    pub fn baseptr(&mut self) -> BaseptrW<CmdlistaddrSpec> {
        BaseptrW::new(self, 0)
    }
}
#[doc = "Command list base pointer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdlistaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdlistaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdlistaddrSpec;
impl crate::RegisterSpec for CmdlistaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdlistaddr::R`](R) reader structure"]
impl crate::Readable for CmdlistaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdlistaddr::W`](W) writer structure"]
impl crate::Writable for CmdlistaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDLISTADDR to value 0"]
impl crate::Resettable for CmdlistaddrSpec {
    const RESET_VALUE: u32 = 0;
}
