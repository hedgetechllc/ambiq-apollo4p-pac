#[doc = "Register `MEMORYMAP25` reader"]
pub type R = crate::R<Memorymap25Spec>;
#[doc = "Register `MEMORYMAP25` writer"]
pub type W = crate::W<Memorymap25Spec>;
#[doc = "Field `PHYSADDRMAP25` reader - Contains the physical address in memory to map the R25 register to."]
pub type Physaddrmap25R = crate::FieldReader<u16>;
#[doc = "Field `PHYSADDRMAP25` writer - Contains the physical address in memory to map the R25 register to."]
pub type Physaddrmap25W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R25 register to."]
    #[inline(always)]
    pub fn physaddrmap25(&self) -> Physaddrmap25R {
        Physaddrmap25R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Contains the physical address in memory to map the R25 register to."]
    #[inline(always)]
    #[must_use]
    pub fn physaddrmap25(&mut self) -> Physaddrmap25W<Memorymap25Spec> {
        Physaddrmap25W::new(self, 1)
    }
}
#[doc = "This register maps the virtual register R25 to a physical address in memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`memorymap25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memorymap25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Memorymap25Spec;
impl crate::RegisterSpec for Memorymap25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memorymap25::R`](R) reader structure"]
impl crate::Readable for Memorymap25Spec {}
#[doc = "`write(|w| ..)` method takes [`memorymap25::W`](W) writer structure"]
impl crate::Writable for Memorymap25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMORYMAP25 to value 0"]
impl crate::Resettable for Memorymap25Spec {
    const RESET_VALUE: u32 = 0;
}
