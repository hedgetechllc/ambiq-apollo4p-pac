#[doc = "Register `IMON0` reader"]
pub type R = crate::R<Imon0Spec>;
#[doc = "Register `IMON0` writer"]
pub type W = crate::W<Imon0Spec>;
#[doc = "Field `IACCESS` reader - Total accesses to Instruction cache"]
pub type IaccessR = crate::FieldReader<u32>;
#[doc = "Field `IACCESS` writer - Total accesses to Instruction cache"]
pub type IaccessW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total accesses to Instruction cache"]
    #[inline(always)]
    pub fn iaccess(&self) -> IaccessR {
        IaccessR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total accesses to Instruction cache"]
    #[inline(always)]
    #[must_use]
    pub fn iaccess(&mut self) -> IaccessW<Imon0Spec> {
        IaccessW::new(self, 0)
    }
}
#[doc = "Instruction Cache Total Accesses\n\nYou can [`read`](crate::Reg::read) this register and get [`imon0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imon0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imon0Spec;
impl crate::RegisterSpec for Imon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imon0::R`](R) reader structure"]
impl crate::Readable for Imon0Spec {}
#[doc = "`write(|w| ..)` method takes [`imon0::W`](W) writer structure"]
impl crate::Writable for Imon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMON0 to value 0"]
impl crate::Resettable for Imon0Spec {
    const RESET_VALUE: u32 = 0;
}
