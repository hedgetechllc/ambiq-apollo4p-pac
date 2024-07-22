#[doc = "Register `MEMORYMAP22` reader"]
pub type R = crate::R<Memorymap22Spec>;
#[doc = "Register `MEMORYMAP22` writer"]
pub type W = crate::W<Memorymap22Spec>;
#[doc = "Field `PHYSADDRMAP22` reader - Contains the physical address in memory to map the R22 register to."]
pub type Physaddrmap22R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP22` writer - Contains the physical address in memory to map the R22 register to."]
pub type Physaddrmap22W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R22 register to."]
    #[inline(always)]
    pub fn physaddrmap22(&self) -> Physaddrmap22R {
        Physaddrmap22R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R22 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap22(&mut self) -> Physaddrmap22W<Memorymap22Spec> {
        Physaddrmap22W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R22 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap22Spec;
impl crate::RegisterSpec for Memorymap22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap22::R`](R) reader structure"]
impl crate::Readable for Memorymap22Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap22::W`](W) writer structure"]
impl crate::Writable for Memorymap22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP22 to value 0"]
impl crate::Resettable for Memorymap22Spec {
    const RESET_VALUE: u32 = 0;
}
