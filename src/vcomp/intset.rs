#[doc = "Register `INTSET` reader"]
pub type R = crate::R<IntsetSpec>;
#[doc = "Register `INTSET` writer"]
pub type W = crate::W<IntsetSpec>;
#[doc = "Field `OUTLOW` reader - This bit is the vcompout low interrupt."]
pub type OutlowR = crate::BitReader;
#[doc = "Field `OUTLOW` writer - This bit is the vcompout low interrupt."]
pub type OutlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTHI` reader - This bit is the vcompout high interrupt."]
pub type OuthiR = crate::BitReader;
#[doc = "Field `OUTHI` writer - This bit is the vcompout high interrupt."]
pub type OuthiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub fn outlow(&self) -> OutlowR {
        OutlowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub fn outhi(&self) -> OuthiR {
        OuthiR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is the vcompout low interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outlow(&mut self) -> OutlowW<IntsetSpec> {
        OutlowW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is the vcompout high interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outhi(&mut self) -> OuthiW<IntsetSpec> {
        OuthiW::new(self, 1)
    }
}
#[doc = "Write a 1 to a bit in this register to instantly generate an interrupt from this module. (Generally used for testing purposes).\n\nYou can [`read`](crate::Reg::read) this register and get [`intset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsetSpec;
impl crate::RegisterSpec for IntsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intset::R`](R) reader structure"]
impl crate::Readable for IntsetSpec {}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for IntsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for IntsetSpec {
    const RESET_VALUE: u32 = 0;
}
