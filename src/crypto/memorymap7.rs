#[doc = "Register `MEMORYMAP7` reader"]
pub type R = crate::R<Memorymap7Spec>;
#[doc = "Register `MEMORYMAP7` writer"]
pub type W = crate::W<Memorymap7Spec>;
#[doc = "Field `PHYSADDRMAP7` reader - Contains the physical address in memory to map the R7 register."]
pub type Physaddrmap7R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP7` writer - Contains the physical address in memory to map the R7 register."]
pub type Physaddrmap7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R7 register."]
    #[inline(always)]
    pub fn physaddrmap7(&self) -> Physaddrmap7R {
        Physaddrmap7R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R7 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap7(&mut self) -> Physaddrmap7W<Memorymap7Spec> {
        Physaddrmap7W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R7 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap7Spec;
impl crate::RegisterSpec for Memorymap7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap7::R`](R) reader structure"]
impl crate::Readable for Memorymap7Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap7::W`](W) writer structure"]
impl crate::Writable for Memorymap7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP7 to value 0"]
impl crate::Resettable for Memorymap7Spec {
    const RESET_VALUE: u32 = 0;
}
