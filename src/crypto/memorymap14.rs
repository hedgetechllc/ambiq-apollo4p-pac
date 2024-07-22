#[doc = "Register `MEMORYMAP14` reader"]
pub type R = crate::R<Memorymap14Spec>;
#[doc = "Register `MEMORYMAP14` writer"]
pub type W = crate::W<Memorymap14Spec>;
#[doc = "Field `PHYSADDRMAP14` reader - Contains the physical address in memory to map the R14 register."]
pub type Physaddrmap14R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP14` writer - Contains the physical address in memory to map the R14 register."]
pub type Physaddrmap14W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R14 register."]
    #[inline(always)]
    pub fn physaddrmap14(&self) -> Physaddrmap14R {
        Physaddrmap14R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R14 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap14(&mut self) -> Physaddrmap14W<Memorymap14Spec> {
        Physaddrmap14W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R14 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap14Spec;
impl crate::RegisterSpec for Memorymap14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap14::R`](R) reader structure"]
impl crate::Readable for Memorymap14Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap14::W`](W) writer structure"]
impl crate::Writable for Memorymap14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP14 to value 0"]
impl crate::Resettable for Memorymap14Spec {
    const RESET_VALUE: u32 = 0;
}
