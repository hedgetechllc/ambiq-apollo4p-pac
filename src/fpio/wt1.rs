#[doc = "Register `WT1` reader"]
pub type R = crate::R<Wt1Spec>;
#[doc = "Register `WT1` writer"]
pub type W = crate::W<Wt1Spec>;
#[doc = "Field `WT1` reader - GPIO63-32 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt1R = crate::FieldReader<u32>;
#[doc = "Field `WT1` writer - GPIO63-32 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
pub type Wt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO63-32 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    pub fn wt1(&self) -> Wt1R {
        Wt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO63-32 Reads or writes pin state. Writes of 1 bits set output pad signal if the GPIO is enabled for output. Reads return status, including sets/clears through the WTS and WTC registers."]
    #[inline(always)]
    #[must_use]
    pub fn wt1(&mut self) -> Wt1W<Wt1Spec> {
        Wt1W::new(self, 0)
    }
}
#[doc = "GPIO Output 1 (63-32)\n\nYou can [`read`](crate::Reg::read) this register and get [`wt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wt1Spec;
impl crate::RegisterSpec for Wt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wt1::R`](R) reader structure"]
impl crate::Readable for Wt1Spec {}
#[doc = "`write(|w| ..)` method takes [`wt1::W`](W) writer structure"]
impl crate::Writable for Wt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WT1 to value 0"]
impl crate::Resettable for Wt1Spec {
    const RESET_VALUE: u32 = 0;
}
