#[doc = "Register `MEMORYMAP9` reader"]
pub type R = crate::R<Memorymap9Spec>;
#[doc = "Register `MEMORYMAP9` writer"]
pub type W = crate::W<Memorymap9Spec>;
#[doc = "Field `PHYSADDRMAP9` reader - Contains the physical address in memory to map the R9 register."]
pub type Physaddrmap9R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP9` writer - Contains the physical address in memory to map the R9 register."]
pub type Physaddrmap9W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R9 register."]
    #[inline(always)]
    pub fn physaddrmap9(&self) -> Physaddrmap9R {
        Physaddrmap9R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R9 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap9(&mut self) -> Physaddrmap9W<Memorymap9Spec> {
        Physaddrmap9W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R9 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap9Spec;
impl crate::RegisterSpec for Memorymap9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap9::R`](R) reader structure"]
impl crate::Readable for Memorymap9Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap9::W`](W) writer structure"]
impl crate::Writable for Memorymap9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP9 to value 0"]
impl crate::Resettable for Memorymap9Spec {
    const RESET_VALUE: u32 = 0;
}
