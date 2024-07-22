#[doc = "Register `HOSTDCUEN2` reader"]
pub type R = crate::R<Hostdcuen2Spec>;
#[doc = "Register `HOSTDCUEN2` writer"]
pub type W = crate::W<Hostdcuen2Spec>;
#[doc = "Field `HOSTDCUEN2` reader - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen2R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCUEN2` writer - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    pub fn hostdcuen2(&self) -> Hostdcuen2R {
        Hostdcuen2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn hostdcuen2(&mut self) -> Hostdcuen2W<Hostdcuen2Spec> {
        Hostdcuen2W::new(self, 0)
    }
}
#[doc = "The DCU \\[95:64\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdcuen2Spec;
impl crate::RegisterSpec for Hostdcuen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdcuen2::R`](R) reader structure"]
impl crate::Readable for Hostdcuen2Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdcuen2::W`](W) writer structure"]
impl crate::Writable for Hostdcuen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCUEN2 to value 0xffff_ffff"]
impl crate::Resettable for Hostdcuen2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
