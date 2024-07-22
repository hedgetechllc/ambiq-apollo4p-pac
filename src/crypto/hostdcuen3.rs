#[doc = "Register `HOSTDCUEN3` reader"]
pub type R = crate::R<Hostdcuen3Spec>;
#[doc = "Register `HOSTDCUEN3` writer"]
pub type W = crate::W<Hostdcuen3Spec>;
#[doc = "Field `HOSTDCUEN3` reader - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen3R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCUEN3` writer - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    pub fn hostdcuen3(&self) -> Hostdcuen3R {
        Hostdcuen3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn hostdcuen3(&mut self) -> Hostdcuen3W<Hostdcuen3Spec> {
        Hostdcuen3W::new(self, 0)
    }
}
#[doc = "The DCU \\[1271:96\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdcuen3Spec;
impl crate::RegisterSpec for Hostdcuen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdcuen3::R`](R) reader structure"]
impl crate::Readable for Hostdcuen3Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdcuen3::W`](W) writer structure"]
impl crate::Writable for Hostdcuen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCUEN3 to value 0xffff_ffff"]
impl crate::Resettable for Hostdcuen3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
