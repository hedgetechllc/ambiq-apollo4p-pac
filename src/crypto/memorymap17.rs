#[doc = "Register `MEMORYMAP17` reader"]
pub type R = crate::R<Memorymap17Spec>;
#[doc = "Register `MEMORYMAP17` writer"]
pub type W = crate::W<Memorymap17Spec>;
#[doc = "Field `PHYSADDRMAP17` reader - Contains the physical address in memory to map the R17 registero."]
pub type Physaddrmap17R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP17` writer - Contains the physical address in memory to map the R17 registero."]
pub type Physaddrmap17W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R17 registero."]
    #[inline(always)]
    pub fn physaddrmap17(&self) -> Physaddrmap17R {
        Physaddrmap17R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R17 registero."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap17(&mut self) -> Physaddrmap17W<Memorymap17Spec> {
        Physaddrmap17W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R17 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap17Spec;
impl crate::RegisterSpec for Memorymap17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap17::R`](R) reader structure"]
impl crate::Readable for Memorymap17Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap17::W`](W) writer structure"]
impl crate::Writable for Memorymap17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP17 to value 0"]
impl crate::Resettable for Memorymap17Spec {
    const RESET_VALUE: u32 = 0;
}
