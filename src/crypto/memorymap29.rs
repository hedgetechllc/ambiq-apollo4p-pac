#[doc = "Register `MEMORYMAP29` reader"]
pub type R = crate::R<Memorymap29Spec>;
#[doc = "Register `MEMORYMAP29` writer"]
pub type W = crate::W<Memorymap29Spec>;
#[doc = "Field `PHYSADDRMAP29` reader - Contains the physical address in memory to map the R29 register."]
pub type Physaddrmap29R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP29` writer - Contains the physical address in memory to map the R29 register."]
pub type Physaddrmap29W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R29 register."]
    #[inline(always)]
    pub fn physaddrmap29(&self) -> Physaddrmap29R {
        Physaddrmap29R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R29 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap29(&mut self) -> Physaddrmap29W<Memorymap29Spec> {
        Physaddrmap29W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R29 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap29Spec;
impl crate::RegisterSpec for Memorymap29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap29::R`](R) reader structure"]
impl crate::Readable for Memorymap29Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap29::W`](W) writer structure"]
impl crate::Writable for Memorymap29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP29 to value 0"]
impl crate::Resettable for Memorymap29Spec {
    const RESET_VALUE: u32 = 0;
}
