#[doc = "Register `MEMORYMAP23` reader"]
pub type R = crate::R<Memorymap23Spec>;
#[doc = "Register `MEMORYMAP23` writer"]
pub type W = crate::W<Memorymap23Spec>;
#[doc = "Field `PHYSADDRMAP23` reader - Contains the physical address in memory to map the R23 register to."]
pub type Physaddrmap23R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP23` writer - Contains the physical address in memory to map the R23 register to."]
pub type Physaddrmap23W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R23 register to."]
    #[inline(always)]
    pub fn physaddrmap23(&self) -> Physaddrmap23R {
        Physaddrmap23R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R23 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap23(&mut self) -> Physaddrmap23W<Memorymap23Spec> {
        Physaddrmap23W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R23 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap23Spec;
impl crate::RegisterSpec for Memorymap23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap23::R`](R) reader structure"]
impl crate::Readable for Memorymap23Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap23::W`](W) writer structure"]
impl crate::Writable for Memorymap23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP23 to value 0"]
impl crate::Resettable for Memorymap23Spec {
    const RESET_VALUE: u32 = 0;
}
