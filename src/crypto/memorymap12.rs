#[doc = "Register `MEMORYMAP12` reader"]
pub type R = crate::R<Memorymap12Spec>;
#[doc = "Register `MEMORYMAP12` writer"]
pub type W = crate::W<Memorymap12Spec>;
#[doc = "Field `PHYSADDRMAP12` reader - Contains the physical address in memory to map the R12 register."]
pub type Physaddrmap12R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP12` writer - Contains the physical address in memory to map the R12 register."]
pub type Physaddrmap12W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R12 register."]
    #[inline(always)]
    pub fn physaddrmap12(&self) -> Physaddrmap12R {
        Physaddrmap12R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R12 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap12(&mut self) -> Physaddrmap12W<Memorymap12Spec> {
        Physaddrmap12W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R12 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap12Spec;
impl crate::RegisterSpec for Memorymap12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap12::R`](R) reader structure"]
impl crate::Readable for Memorymap12Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap12::W`](W) writer structure"]
impl crate::Writable for Memorymap12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP12 to value 0"]
impl crate::Resettable for Memorymap12Spec {
    const RESET_VALUE: u32 = 0;
}
