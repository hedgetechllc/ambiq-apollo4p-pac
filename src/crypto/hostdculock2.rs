#[doc = "Register `HOSTDCULOCK2` reader"]
pub type R = crate::R<Hostdculock2Spec>;
#[doc = "Register `HOSTDCULOCK2` writer"]
pub type W = crate::W<Hostdculock2Spec>;
#[doc = "Field `HOSTDCULOCK2` reader - DCU_lock \\[95:64\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock2R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCULOCK2` writer - DCU_lock \\[95:64\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DCU_lock \\[95:64\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    pub fn hostdculock2(&self) -> Hostdculock2R {
        Hostdculock2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DCU_lock \\[95:64\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    #[must_use]
    pub fn hostdculock2(&mut self) -> Hostdculock2W<Hostdculock2Spec> {
        Hostdculock2W::new(self, 0)
    }
}
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdculock2Spec;
impl crate::RegisterSpec for Hostdculock2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdculock2::R`](R) reader structure"]
impl crate::Readable for Hostdculock2Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdculock2::W`](W) writer structure"]
impl crate::Writable for Hostdculock2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCULOCK2 to value 0"]
impl crate::Resettable for Hostdculock2Spec {
    const RESET_VALUE: u32 = 0;
}
