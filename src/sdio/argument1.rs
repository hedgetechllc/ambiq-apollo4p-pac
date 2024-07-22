#[doc = "Register `ARGUMENT1` reader"]
pub type R = crate::R<Argument1Spec>;
#[doc = "Register `ARGUMENT1` writer"]
pub type W = crate::W<Argument1Spec>;
#[doc = "Field `CMDARG1` reader - The SD Command Argument is specified as bit39-8 of Command-Format."]
pub type Cmdarg1R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG1` writer - The SD Command Argument is specified as bit39-8 of Command-Format."]
pub type Cmdarg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command-Format."]
    #[inline(always)]
    pub fn cmdarg1(&self) -> Cmdarg1R {
        Cmdarg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command-Format."]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg1(&mut self) -> Cmdarg1W<Argument1Spec> {
        Cmdarg1W::new(self, 0)
    }
}
#[doc = "Argument1\n\nYou can [`read`](crate::Reg::read) this register and get [`argument1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Argument1Spec;
impl crate::RegisterSpec for Argument1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argument1::R`](R) reader structure"]
impl crate::Readable for Argument1Spec {}
#[doc = "`write(|w| ..)` method takes [`argument1::W`](W) writer structure"]
impl crate::Writable for Argument1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGUMENT1 to value 0"]
impl crate::Resettable for Argument1Spec {
    const RESET_VALUE: u32 = 0;
}
