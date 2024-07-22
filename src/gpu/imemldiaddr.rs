#[doc = "Register `IMEMLDIADDR` reader"]
pub type R = crate::R<ImemldiaddrSpec>;
#[doc = "Register `IMEMLDIADDR` writer"]
pub type W = crate::W<ImemldiaddrSpec>;
#[doc = "Field `IMEM` reader - ADDR Load shader. Load shader instruction memory address."]
pub type ImemR = crate::FieldReader<u32>;
#[doc = "Field `IMEM` writer - ADDR Load shader. Load shader instruction memory address."]
pub type ImemW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR Load shader. Load shader instruction memory address."]
    #[inline(always)]
    pub fn imem(&self) -> ImemR {
        ImemR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDR Load shader. Load shader instruction memory address."]
    #[inline(always)]
    #[must_use]
    pub fn imem(&mut self) -> ImemW<ImemldiaddrSpec> {
        ImemW::new(self, 0)
    }
}
#[doc = "Load shader instruction memory address.\n\nYou can [`read`](crate::Reg::read) this register and get [`imemldiaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imemldiaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImemldiaddrSpec;
impl crate::RegisterSpec for ImemldiaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imemldiaddr::R`](R) reader structure"]
impl crate::Readable for ImemldiaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`imemldiaddr::W`](W) writer structure"]
impl crate::Writable for ImemldiaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMEMLDIADDR to value 0"]
impl crate::Resettable for ImemldiaddrSpec {
    const RESET_VALUE: u32 = 0;
}
