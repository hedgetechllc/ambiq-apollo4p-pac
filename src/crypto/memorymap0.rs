#[doc = "Register `MEMORYMAP0` reader"]
pub type R = crate::R<Memorymap0Spec>;
#[doc = "Register `MEMORYMAP0` writer"]
pub type W = crate::W<Memorymap0Spec>;
#[doc = "Field `PHYSADDRMAP0` reader - Contains the physical address in memory to map the R0 register."]
pub type Physaddrmap0R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP0` writer - Contains the physical address in memory to map the R0 register."]
pub type Physaddrmap0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R0 register."]
    #[inline(always)]
    pub fn physaddrmap0(&self) -> Physaddrmap0R {
        Physaddrmap0R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R0 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap0(&mut self) -> Physaddrmap0W<Memorymap0Spec> {
        Physaddrmap0W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R0 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap0Spec;
impl crate::RegisterSpec for Memorymap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap0::R`](R) reader structure"]
impl crate::Readable for Memorymap0Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap0::W`](W) writer structure"]
impl crate::Writable for Memorymap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP0 to value 0"]
impl crate::Resettable for Memorymap0Spec {
    const RESET_VALUE: u32 = 0;
}
