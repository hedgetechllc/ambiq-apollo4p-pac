#[doc = "Register `MEMORYMAP4` reader"]
pub type R = crate::R<Memorymap4Spec>;
#[doc = "Register `MEMORYMAP4` writer"]
pub type W = crate::W<Memorymap4Spec>;
#[doc = "Field `PHYSADDRMAP4` reader - Contains the physical address in memory to map the R4 register."]
pub type Physaddrmap4R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP4` writer - Contains the physical address in memory to map the R4 register."]
pub type Physaddrmap4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R4 register."]
    #[inline(always)]
    pub fn physaddrmap4(&self) -> Physaddrmap4R {
        Physaddrmap4R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R4 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap4(&mut self) -> Physaddrmap4W<Memorymap4Spec> {
        Physaddrmap4W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R4 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap4Spec;
impl crate::RegisterSpec for Memorymap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap4::R`](R) reader structure"]
impl crate::Readable for Memorymap4Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap4::W`](W) writer structure"]
impl crate::Writable for Memorymap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP4 to value 0"]
impl crate::Resettable for Memorymap4Spec {
    const RESET_VALUE: u32 = 0;
}
