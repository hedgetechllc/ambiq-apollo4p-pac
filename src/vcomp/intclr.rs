#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
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
    pub fn outlow(&mut self) -> OutlowW<IntclrSpec> {
        OutlowW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is the vcompout high interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outhi(&mut self) -> OuthiW<IntclrSpec> {
        OuthiW::new(self, 1)
    }
}
#[doc = "Write a 1 to a bit in this register to clear the interrupt status associated with that bit.\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
