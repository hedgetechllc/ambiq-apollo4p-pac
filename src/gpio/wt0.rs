#[doc = "Register `WT0` reader"]
pub type R = crate::R<Wt0Spec>;
#[doc = "Register `WT0` writer"]
pub type W = crate::W<Wt0Spec>;
#[doc = "Field `WT0` reader - GPIO31-0 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt0R = crate::FieldReader<u32>;
#[doc = "Field `WT0` writer - GPIO31-0 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    pub fn wt0(&self) -> Wt0R {
        Wt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    #[must_use]
    pub fn wt0(&mut self) -> Wt0W<Wt0Spec> {
        Wt0W::new(self, 0)
    }
}
#[doc = "GPIO Output 0 (31-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wt0Spec;
impl crate::RegisterSpec for Wt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wt0::R`](R) reader structure"]
impl crate::Readable for Wt0Spec {}
#[doc = "`write(|w| ..)` method takes [`wt0::W`](W) writer structure"]
impl crate::Writable for Wt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WT0 to value 0"]
impl crate::Resettable for Wt0Spec {
    const RESET_VALUE: u32 = 0;
}
