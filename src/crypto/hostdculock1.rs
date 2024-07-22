#[doc = "Register `HOSTDCULOCK1` reader"]
pub type R = crate::R<Hostdculock1Spec>;
#[doc = "Register `HOSTDCULOCK1` writer"]
pub type W = crate::W<Hostdculock1Spec>;
#[doc = "Field `HOSTDCULOCK1` reader - DCU_lock \\[63:32\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock1R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCULOCK1` writer - DCU_lock \\[63:32\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DCU_lock \\[63:32\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    pub fn hostdculock1(&self) -> Hostdculock1R {
        Hostdculock1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DCU_lock \\[63:32\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    #[must_use]
    pub fn hostdculock1(&mut self) -> Hostdculock1W<Hostdculock1Spec> {
        Hostdculock1W::new(self, 0)
    }
}
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdculock1Spec;
impl crate::RegisterSpec for Hostdculock1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdculock1::R`](R) reader structure"]
impl crate::Readable for Hostdculock1Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdculock1::W`](W) writer structure"]
impl crate::Writable for Hostdculock1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCULOCK1 to value 0"]
impl crate::Resettable for Hostdculock1Spec {
    const RESET_VALUE: u32 = 0;
}
