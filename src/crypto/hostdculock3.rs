#[doc = "Register `HOSTDCULOCK3` reader"]
pub type R = crate::R<Hostdculock3Spec>;
#[doc = "Register `HOSTDCULOCK3` writer"]
pub type W = crate::W<Hostdculock3Spec>;
#[doc = "Field `HOSTDCULOCK3` reader - DCU_lock \\[127:96\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock3R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCULOCK3` writer - DCU_lock \\[127:96\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DCU_lock \\[127:96\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    pub fn hostdculock3(&self) -> Hostdculock3R {
        Hostdculock3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DCU_lock \\[127:96\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    #[must_use]
    pub fn hostdculock3(&mut self) -> Hostdculock3W<Hostdculock3Spec> {
        Hostdculock3W::new(self, 0)
    }
}
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdculock3Spec;
impl crate::RegisterSpec for Hostdculock3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdculock3::R`](R) reader structure"]
impl crate::Readable for Hostdculock3Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdculock3::W`](W) writer structure"]
impl crate::Writable for Hostdculock3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCULOCK3 to value 0"]
impl crate::Resettable for Hostdculock3Spec {
    const RESET_VALUE: u32 = 0;
}
