#[doc = "Register `MEMORYMAP13` reader"]
pub type R = crate::R<Memorymap13Spec>;
#[doc = "Register `MEMORYMAP13` writer"]
pub type W = crate::W<Memorymap13Spec>;
#[doc = "Field `PHYSADDRMAP13` reader - Contains the physical address in memory to map the R13 register."]
pub type Physaddrmap13R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP13` writer - Contains the physical address in memory to map the R13 register."]
pub type Physaddrmap13W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R13 register."]
    #[inline(always)]
    pub fn physaddrmap13(&self) -> Physaddrmap13R {
        Physaddrmap13R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R13 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap13(&mut self) -> Physaddrmap13W<Memorymap13Spec> {
        Physaddrmap13W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R13 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap13Spec;
impl crate::RegisterSpec for Memorymap13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap13::R`](R) reader structure"]
impl crate::Readable for Memorymap13Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap13::W`](W) writer structure"]
impl crate::Writable for Memorymap13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP13 to value 0"]
impl crate::Resettable for Memorymap13Spec {
    const RESET_VALUE: u32 = 0;
}
