#[doc = "Register `MEMORYMAP30` reader"]
pub type R = crate::R<Memorymap30Spec>;
#[doc = "Register `MEMORYMAP30` writer"]
pub type W = crate::W<Memorymap30Spec>;
#[doc = "Field `PHYSADDRMAP30` reader - Contains the physical address in memory to map the R30 register."]
pub type Physaddrmap30R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP30` writer - Contains the physical address in memory to map the R30 register."]
pub type Physaddrmap30W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R30 register."]
    #[inline(always)]
    pub fn physaddrmap30(&self) -> Physaddrmap30R {
        Physaddrmap30R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R30 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap30(&mut self) -> Physaddrmap30W<Memorymap30Spec> {
        Physaddrmap30W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R30 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap30Spec;
impl crate::RegisterSpec for Memorymap30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap30::R`](R) reader structure"]
impl crate::Readable for Memorymap30Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap30::W`](W) writer structure"]
impl crate::Writable for Memorymap30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP30 to value 0"]
impl crate::Resettable for Memorymap30Spec {
    const RESET_VALUE: u32 = 0;
}
