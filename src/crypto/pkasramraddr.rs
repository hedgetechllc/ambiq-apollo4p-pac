#[doc = "Register `PKASRAMRADDR` reader"]
pub type R = crate::R<PkasramraddrSpec>;
#[doc = "Register `PKASRAMRADDR` writer"]
pub type W = crate::W<PkasramraddrSpec>;
#[doc = "Field `PKASRAMRADDR` reader - PKA SRAM read starting address"]
pub type PkasramraddrR = crate::FieldReader<u32>;
#[doc = "Field `PKASRAMRADDR` writer - PKA SRAM read starting address"]
pub type PkasramraddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PKA SRAM read starting address"]
    #[inline(always)]
    pub fn pkasramraddr(&self) -> PkasramraddrR {
        PkasramraddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PKA SRAM read starting address"]
    #[inline(always)]
    #[must_use]
    pub fn pkasramraddr(&mut self) -> PkasramraddrW<PkasramraddrSpec> {
        PkasramraddrW::new(self, 0)
    }
}
#[doc = "first address given to PKA SRAM for read transactions.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkasramraddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkasramraddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkasramraddrSpec;
impl crate::RegisterSpec for PkasramraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkasramraddr::R`](R) reader structure"]
impl crate::Readable for PkasramraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pkasramraddr::W`](W) writer structure"]
impl crate::Writable for PkasramraddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASRAMRADDR to value 0"]
impl crate::Resettable for PkasramraddrSpec {
    const RESET_VALUE: u32 = 0;
}
