#[doc = "Register `HOSTDCUEN1` reader"]
pub type R = crate::R<Hostdcuen1Spec>;
#[doc = "Register `HOSTDCUEN1` writer"]
pub type W = crate::W<Hostdcuen1Spec>;
#[doc = "Field `HOSTDCUEN1` reader - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen1R = crate::FieldReader<u32>;
#[doc = "Field `HOSTDCUEN1` writer - Debug Control Unit (DCU) Enable bits."]
pub type Hostdcuen1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    pub fn hostdcuen1(&self) -> Hostdcuen1R {
        Hostdcuen1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Control Unit (DCU) Enable bits."]
    #[inline(always)]
    #[must_use]
    pub fn hostdcuen1(&mut self) -> Hostdcuen1W<Hostdcuen1Spec> {
        Hostdcuen1W::new(self, 0)
    }
}
#[doc = "The DCU \\[63:32\\]
enable register. Note: This is a special register, affected by internal logic. Test result of this register is NA.\n\nYou can [`read`](crate::Reg::read) this register and get [`hostdcuen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostdcuen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostdcuen1Spec;
impl crate::RegisterSpec for Hostdcuen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hostdcuen1::R`](R) reader structure"]
impl crate::Readable for Hostdcuen1Spec {}
#[doc = "`write(|w| ..)` method takes [`hostdcuen1::W`](W) writer structure"]
impl crate::Writable for Hostdcuen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOSTDCUEN1 to value 0xffff_ffff"]
impl crate::Resettable for Hostdcuen1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
