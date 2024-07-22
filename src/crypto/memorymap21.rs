#[doc = "Register `MEMORYMAP21` reader"]
pub type R = crate::R<Memorymap21Spec>;
#[doc = "Register `MEMORYMAP21` writer"]
pub type W = crate::W<Memorymap21Spec>;
#[doc = "Field `PHYSADDRMAP21` reader - Contains the physical address in memory to map the R21 register to."]
pub type Physaddrmap21R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP21` writer - Contains the physical address in memory to map the R21 register to."]
pub type Physaddrmap21W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R21 register to."]
    #[inline(always)]
    pub fn physaddrmap21(&self) -> Physaddrmap21R {
        Physaddrmap21R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R21 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap21(&mut self) -> Physaddrmap21W<Memorymap21Spec> {
        Physaddrmap21W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R21 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap21Spec;
impl crate::RegisterSpec for Memorymap21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap21::R`](R) reader structure"]
impl crate::Readable for Memorymap21Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap21::W`](W) writer structure"]
impl crate::Writable for Memorymap21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP21 to value 0"]
impl crate::Resettable for Memorymap21Spec {
    const RESET_VALUE: u32 = 0;
}
