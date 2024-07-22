#[doc = "Register `MEMORYMAP16` reader"]
pub type R = crate::R<Memorymap16Spec>;
#[doc = "Register `MEMORYMAP16` writer"]
pub type W = crate::W<Memorymap16Spec>;
#[doc = "Field `PHYSADDRMAP16` reader - Contains the physical address in memory to map the R16 register."]
pub type Physaddrmap16R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP16` writer - Contains the physical address in memory to map the R16 register."]
pub type Physaddrmap16W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R16 register."]
    #[inline(always)]
    pub fn physaddrmap16(&self) -> Physaddrmap16R {
        Physaddrmap16R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R16 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap16(&mut self) -> Physaddrmap16W<Memorymap16Spec> {
        Physaddrmap16W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R16 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap16Spec;
impl crate::RegisterSpec for Memorymap16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap16::R`](R) reader structure"]
impl crate::Readable for Memorymap16Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap16::W`](W) writer structure"]
impl crate::Writable for Memorymap16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP16 to value 0"]
impl crate::Resettable for Memorymap16Spec {
    const RESET_VALUE: u32 = 0;
}
