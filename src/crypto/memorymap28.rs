#[doc = "Register `MEMORYMAP28` reader"]
pub type R = crate::R<Memorymap28Spec>;
#[doc = "Register `MEMORYMAP28` writer"]
pub type W = crate::W<Memorymap28Spec>;
#[doc = "Field `PHYSADDRMAP28` reader - Contains the physical address in memory to map the R28 register."]
pub type Physaddrmap28R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP28` writer - Contains the physical address in memory to map the R28 register."]
pub type Physaddrmap28W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R28 register."]
    #[inline(always)]
    pub fn physaddrmap28(&self) -> Physaddrmap28R {
        Physaddrmap28R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R28 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap28(&mut self) -> Physaddrmap28W<Memorymap28Spec> {
        Physaddrmap28W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R28 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap28Spec;
impl crate::RegisterSpec for Memorymap28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap28::R`](R) reader structure"]
impl crate::Readable for Memorymap28Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap28::W`](W) writer structure"]
impl crate::Writable for Memorymap28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP28 to value 0"]
impl crate::Resettable for Memorymap28Spec {
    const RESET_VALUE: u32 = 0;
}
