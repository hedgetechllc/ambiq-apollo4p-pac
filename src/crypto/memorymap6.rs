#[doc = "Register `MEMORYMAP6` reader"]
pub type R = crate::R<Memorymap6Spec>;
#[doc = "Register `MEMORYMAP6` writer"]
pub type W = crate::W<Memorymap6Spec>;
#[doc = "Field `PHYSADDRMAP6` reader - Contains the physical address in memory to map the R6 register."]
pub type Physaddrmap6R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP6` writer - Contains the physical address in memory to map the R6 register."]
pub type Physaddrmap6W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R6 register."]
    #[inline(always)]
    pub fn physaddrmap6(&self) -> Physaddrmap6R {
        Physaddrmap6R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R6 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap6(&mut self) -> Physaddrmap6W<Memorymap6Spec> {
        Physaddrmap6W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R6 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap6Spec;
impl crate::RegisterSpec for Memorymap6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap6::R`](R) reader structure"]
impl crate::Readable for Memorymap6Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap6::W`](W) writer structure"]
impl crate::Writable for Memorymap6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP6 to value 0"]
impl crate::Resettable for Memorymap6Spec {
    const RESET_VALUE: u32 = 0;
}
