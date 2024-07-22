#[doc = "Register `WT2` reader"]
pub type R = crate::R<Wt2Spec>;
#[doc = "Register `WT2` writer"]
pub type W = crate::W<Wt2Spec>;
#[doc = "Field `WT2` reader - GPIO95-64 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt2R = crate::FieldReader<u32>;
#[doc = "Field `WT2` writer - GPIO95-64 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO95-64 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    pub fn wt2(&self) -> Wt2R {
        Wt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO95-64 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    #[must_use]
    pub fn wt2(&mut self) -> Wt2W<Wt2Spec> {
        Wt2W::new(self, 0)
    }
}
#[doc = "GPIO Output 2 (95-64)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wt2Spec;
impl crate::RegisterSpec for Wt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wt2::R`](R) reader structure"]
impl crate::Readable for Wt2Spec {}
#[doc = "`write(|w| ..)` method takes [`wt2::W`](W) writer structure"]
impl crate::Writable for Wt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WT2 to value 0"]
impl crate::Resettable for Wt2Spec {
    const RESET_VALUE: u32 = 0;
}
