#[doc = "Register `MEMORYMAP27` reader"]
pub type R = crate::R<Memorymap27Spec>;
#[doc = "Register `MEMORYMAP27` writer"]
pub type W = crate::W<Memorymap27Spec>;
#[doc = "Field `PHYSADDRMAP27` reader - Contains the physical address in memory to map the R27 register to."]
pub type Physaddrmap27R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP27` writer - Contains the physical address in memory to map the R27 register to."]
pub type Physaddrmap27W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R27 register to."]
    #[inline(always)]
    pub fn physaddrmap27(&self) -> Physaddrmap27R {
        Physaddrmap27R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R27 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap27(&mut self) -> Physaddrmap27W<Memorymap27Spec> {
        Physaddrmap27W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R27 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap27Spec;
impl crate::RegisterSpec for Memorymap27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap27::R`](R) reader structure"]
impl crate::Readable for Memorymap27Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap27::W`](W) writer structure"]
impl crate::Writable for Memorymap27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP27 to value 0"]
impl crate::Resettable for Memorymap27Spec {
    const RESET_VALUE: u32 = 0;
}
