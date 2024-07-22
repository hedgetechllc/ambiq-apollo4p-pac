#[doc = "Register `MEMORYMAP3` reader"]
pub type R = crate::R<Memorymap3Spec>;
#[doc = "Register `MEMORYMAP3` writer"]
pub type W = crate::W<Memorymap3Spec>;
#[doc = "Field `PHYSADDRMAP3` reader - Contains the physical address in memory to map the R3 register."]
pub type Physaddrmap3R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP3` writer - Contains the physical address in memory to map the R3 register."]
pub type Physaddrmap3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R3 register."]
    #[inline(always)]
    pub fn physaddrmap3(&self) -> Physaddrmap3R {
        Physaddrmap3R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R3 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap3(&mut self) -> Physaddrmap3W<Memorymap3Spec> {
        Physaddrmap3W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R3 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap3Spec;
impl crate::RegisterSpec for Memorymap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap3::R`](R) reader structure"]
impl crate::Readable for Memorymap3Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap3::W`](W) writer structure"]
impl crate::Writable for Memorymap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP3 to value 0"]
impl crate::Resettable for Memorymap3Spec {
    const RESET_VALUE: u32 = 0;
}
