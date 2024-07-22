#[doc = "Register `MEMORYMAP2` reader"]
pub type R = crate::R<Memorymap2Spec>;
#[doc = "Register `MEMORYMAP2` writer"]
pub type W = crate::W<Memorymap2Spec>;
#[doc = "Field `PHYSADDRMAP2` reader - Contains the physical address in memory to map the R2 register."]
pub type Physaddrmap2R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP2` writer - Contains the physical address in memory to map the R2 register."]
pub type Physaddrmap2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R2 register."]
    #[inline(always)]
    pub fn physaddrmap2(&self) -> Physaddrmap2R {
        Physaddrmap2R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R2 register."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap2(&mut self) -> Physaddrmap2W<Memorymap2Spec> {
        Physaddrmap2W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R2 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap2Spec;
impl crate::RegisterSpec for Memorymap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap2::R`](R) reader structure"]
impl crate::Readable for Memorymap2Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap2::W`](W) writer structure"]
impl crate::Writable for Memorymap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP2 to value 0"]
impl crate::Resettable for Memorymap2Spec {
    const RESET_VALUE: u32 = 0;
}
