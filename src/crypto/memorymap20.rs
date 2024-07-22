#[doc = "Register `MEMORYMAP20` reader"]
pub type R = crate::R<Memorymap20Spec>;
#[doc = "Register `MEMORYMAP20` writer"]
pub type W = crate::W<Memorymap20Spec>;
#[doc = "Field `PHYSADDRMAP20` reader - Contains the physical address in memory to map the R20 register to."]
pub type Physaddrmap20R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP20` writer - Contains the physical address in memory to map the R20 register to."]
pub type Physaddrmap20W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R20 register to."]
    #[inline(always)]
    pub fn physaddrmap20(&self) -> Physaddrmap20R {
        Physaddrmap20R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R20 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap20(&mut self) -> Physaddrmap20W<Memorymap20Spec> {
        Physaddrmap20W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R20 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap20Spec;
impl crate::RegisterSpec for Memorymap20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap20::R`](R) reader structure"]
impl crate::Readable for Memorymap20Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap20::W`](W) writer structure"]
impl crate::Writable for Memorymap20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP20 to value 0"]
impl crate::Resettable for Memorymap20Spec {
    const RESET_VALUE: u32 = 0;
}
