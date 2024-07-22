#[doc = "Register `MEMORYMAP26` reader"]
pub type R = crate::R<Memorymap26Spec>;
#[doc = "Register `MEMORYMAP26` writer"]
pub type W = crate::W<Memorymap26Spec>;
#[doc = "Field `PHYSADDRMAP26` reader - Contains the physical address in memory to map the R26 register to."]
pub type Physaddrmap26R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP26` writer - Contains the physical address in memory to map the R26 register to."]
pub type Physaddrmap26W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R26 register to."]
    #[inline(always)]
    pub fn physaddrmap26(&self) -> Physaddrmap26R {
        Physaddrmap26R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R26 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap26(&mut self) -> Physaddrmap26W<Memorymap26Spec> {
        Physaddrmap26W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R26 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap26Spec;
impl crate::RegisterSpec for Memorymap26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap26::R`](R) reader structure"]
impl crate::Readable for Memorymap26Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap26::W`](W) writer structure"]
impl crate::Writable for Memorymap26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP26 to value 0"]
impl crate::Resettable for Memorymap26Spec {
    const RESET_VALUE: u32 = 0;
}
