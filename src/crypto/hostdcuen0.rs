#[doc = "Register `HOSTDCUEN0` reader"]
pub type R = crate::R<Hostdcuen0Spec>;
#[doc = "Register `HOSTDCUEN0` writer"]
pub type W = crate::W<Hostdcuen0Spec>;
#[doc = "Field `HOSTDCUEN0` reader - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen0R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCUEN0` writer - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    pub fn hostdcuen0(&self) -> Hostdcuen0R {
        Hostdcuen0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn hostdcuen0(&mut self) -> Hostdcuen0W<Hostdcuen0Spec> {
        Hostdcuen0W::new(self, 0)
    }
}
#[doc = "The DCU \\[31:0\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdcuen0Spec;
impl crate::RegisterSpec for Hostdcuen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdcuen0::R`](R) reader structure"]
impl crate::Readable for Hostdcuen0Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdcuen0::W`](W) writer structure"]
impl crate::Writable for Hostdcuen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCUEN0 to value 0xffff_ffff"]
impl crate::Resettable for Hostdcuen0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
