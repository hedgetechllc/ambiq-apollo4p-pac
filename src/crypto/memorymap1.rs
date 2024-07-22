#[doc = "Register `MEMORYMAP1` reader"]
pub type R = crate::R<Memorymap1Spec>;
#[doc = "Register `MEMORYMAP1` writer"]
pub type W = crate::W<Memorymap1Spec>;
#[doc = "Field `PHYSADDRMAP1` reader - Contains the physical address in memory to map the R1 register."]
pub type Physaddrmap1R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP1` writer - Contains the physical address in memory to map the R1 register."]
pub type Physaddrmap1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R1 register."]
    #[inline(always)]
    pub fn physaddrmap1(&self) -> Physaddrmap1R {
        Physaddrmap1R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R1 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap1(&mut self) -> Physaddrmap1W<Memorymap1Spec> {
        Physaddrmap1W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R1 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap1Spec;
impl crate::RegisterSpec for Memorymap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap1::R`](R) reader structure"]
impl crate::Readable for Memorymap1Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap1::W`](W) writer structure"]
impl crate::Writable for Memorymap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP1 to value 0"]
impl crate::Resettable for Memorymap1Spec {
    const RESET_VALUE: u32 = 0;
}
