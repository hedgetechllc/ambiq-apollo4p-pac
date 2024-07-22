#[doc = "Register `WT3` reader"]
pub type R = crate::R<Wt3Spec>;
#[doc = "Register `WT3` writer"]
pub type W = crate::W<Wt3Spec>;
#[doc = "Field `WT3` reader - GPIO127-96 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt3R = crate::FieldReader<u32>;
#[doc = "Field `WT3` writer - GPIO127-96 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO127-96 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    pub fn wt3(&self) -> Wt3R {
        Wt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO127-96 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    #[must_use]
    pub fn wt3(&mut self) -> Wt3W<Wt3Spec> {
        Wt3W::new(self, 0)
    }
}
#[doc = "GPIO Output 3 (127-96)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wt3Spec;
impl crate::RegisterSpec for Wt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wt3::R`](R) reader structure"]
impl crate::Readable for Wt3Spec {}
#[doc = "`write(|w| ..)` method takes [`wt3::W`](W) writer structure"]
impl crate::Writable for Wt3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WT3 to value 0"]
impl crate::Resettable for Wt3Spec {
    const RESET_VALUE: u32 = 0;
}
