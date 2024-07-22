#[doc = "Register `PLLLOCKCNT` reader"]
pub type R = crate::R<PlllockcntSpec>;
#[doc = "Register `PLLLOCKCNT` writer"]
pub type W = crate::W<PlllockcntSpec>;
#[doc = "Field `PLLCNTVAL` reader - Pll counter value in terms of low power clock."]
pub type PllcntvalR = crate::FieldReader<u16>;
#[doc = "Field `PLLCNTVAL` writer - Pll counter value in terms of low power clock."]
pub type PllcntvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pll counter value in terms of low power clock."]
    #[inline(always)]
    pub fn pllcntval(&self) -> PllcntvalR {
        PllcntvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pll counter value in terms of low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn pllcntval(&mut self) -> PllcntvalW<PlllockcntSpec> {
        PllcntvalW::new(self, 0)
    }
}
#[doc = "The PLL counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`plllockcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plllockcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlllockcntSpec;
impl crate::RegisterSpec for PlllockcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plllockcnt::R`](R) reader structure"]
impl crate::Readable for PlllockcntSpec {}
#[doc = "`write(|w| ..)` method takes [`plllockcnt::W`](W) writer structure"]
impl crate::Writable for PlllockcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLLOCKCNT to value 0x07d0"]
impl crate::Resettable for PlllockcntSpec {
    const RESET_VALUE: u32 = 0x07d0;
}
