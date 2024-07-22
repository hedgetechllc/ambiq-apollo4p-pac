#[doc = "Register `MEMORYMAP5` reader"]
pub type R = crate::R<Memorymap5Spec>;
#[doc = "Register `MEMORYMAP5` writer"]
pub type W = crate::W<Memorymap5Spec>;
#[doc = "Field `PHYSADDRMAP5` reader - Contains the physical address in memory to map the R5 register."]
pub type Physaddrmap5R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP5` writer - Contains the physical address in memory to map the R5 register."]
pub type Physaddrmap5W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R5 register."]
    #[inline(always)]
    pub fn physaddrmap5(&self) -> Physaddrmap5R {
        Physaddrmap5R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R5 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap5(&mut self) -> Physaddrmap5W<Memorymap5Spec> {
        Physaddrmap5W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R5 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap5Spec;
impl crate::RegisterSpec for Memorymap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap5::R`](R) reader structure"]
impl crate::Readable for Memorymap5Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap5::W`](W) writer structure"]
impl crate::Writable for Memorymap5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP5 to value 0"]
impl crate::Resettable for Memorymap5Spec {
    const RESET_VALUE: u32 = 0;
}
