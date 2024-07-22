#[doc = "Register `MEMORYMAP24` reader"]
pub type R = crate::R<Memorymap24Spec>;
#[doc = "Register `MEMORYMAP24` writer"]
pub type W = crate::W<Memorymap24Spec>;
#[doc = "Field `PHYSADDRMAP24` reader - Contains the physical address in memory to map the R24 register to."]
pub type Physaddrmap24R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP24` writer - Contains the physical address in memory to map the R24 register to."]
pub type Physaddrmap24W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R24 register to."]
    #[inline(always)]
    pub fn physaddrmap24(&self) -> Physaddrmap24R {
        Physaddrmap24R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R24 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap24(&mut self) -> Physaddrmap24W<Memorymap24Spec> {
        Physaddrmap24W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R24 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap24Spec;
impl crate::RegisterSpec for Memorymap24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap24::R`](R) reader structure"]
impl crate::Readable for Memorymap24Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap24::W`](W) writer structure"]
impl crate::Writable for Memorymap24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP24 to value 0"]
impl crate::Resettable for Memorymap24Spec {
    const RESET_VALUE: u32 = 0;
}
