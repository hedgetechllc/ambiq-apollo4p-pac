#[doc = "Register `MEMORYMAP11` reader"]
pub type R = crate::R<Memorymap11Spec>;
#[doc = "Register `MEMORYMAP11` writer"]
pub type W = crate::W<Memorymap11Spec>;
#[doc = "Field `PHYSADDRMAP11` reader - Contains the physical address in memory to map the R11 register."]
pub type Physaddrmap11R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP11` writer - Contains the physical address in memory to map the R11 register."]
pub type Physaddrmap11W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R11 register."]
    #[inline(always)]
    pub fn physaddrmap11(&self) -> Physaddrmap11R {
        Physaddrmap11R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R11 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap11(&mut self) -> Physaddrmap11W<Memorymap11Spec> {
        Physaddrmap11W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R11 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap11Spec;
impl crate::RegisterSpec for Memorymap11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap11::R`](R) reader structure"]
impl crate::Readable for Memorymap11Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap11::W`](W) writer structure"]
impl crate::Writable for Memorymap11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP11 to value 0"]
impl crate::Resettable for Memorymap11Spec {
    const RESET_VALUE: u32 = 0;
}
