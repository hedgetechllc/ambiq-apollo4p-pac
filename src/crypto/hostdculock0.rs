#[doc = "Register `HOSTDCULOCK0` reader"]
pub type R = crate::R<Hostdculock0Spec>;
#[doc = "Register `HOSTDCULOCK0` writer"]
pub type W = crate::W<Hostdculock0Spec>;
#[doc = "Field `HOSTDCULOCK0` reader - DCU_lock \\[31:0\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock0R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCULOCK0` writer - DCU_lock \\[31:0\\]
register (a dedicated lock register per DCU bit)."]
pub type Hostdculock0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DCU_lock \\[31:0\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    pub fn hostdculock0(&self) -> Hostdculock0R {
        Hostdculock0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DCU_lock \\[31:0\\]
register (a dedicated lock register per DCU bit)."]
    #[inline(always)]
    #[must_use]
    pub fn hostdculock0(&mut self) -> Hostdculock0W<Hostdculock0Spec> {
        Hostdculock0W::new(self, 0)
    }
}
#[doc = "The DCU lock register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdculock0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdculock0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdculock0Spec;
impl crate::RegisterSpec for Hostdculock0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdculock0::R`](R) reader structure"]
impl crate::Readable for Hostdculock0Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdculock0::W`](W) writer structure"]
impl crate::Writable for Hostdculock0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCULOCK0 to value 0"]
impl crate::Resettable for Hostdculock0Spec {
    const RESET_VALUE: u32 = 0;
}
