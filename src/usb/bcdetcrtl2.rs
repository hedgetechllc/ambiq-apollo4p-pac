#[doc = "Register `BCDETCRTL2` reader"]
pub type R = crate::R<Bcdetcrtl2Spec>;
#[doc = "Register `BCDETCRTL2` writer"]
pub type W = crate::W<Bcdetcrtl2Spec>;
#[doc = "Field `CHARGEDETBYP` reader - BC detection bypass"]
pub type ChargedetbypR = crate::BitReader;
#[doc = "Field `CHARGEDETBYP` writer - BC detection bypass"]
pub type ChargedetbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDPATTACHED` reader - Force output dp_attached"]
pub type ForcedpattachedR = crate::BitReader;
#[doc = "Field `FORCEDPATTACHED` writer - Force output dp_attached"]
pub type ForcedpattachedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCECPDET` reader - Force output charging port detected"]
pub type ForcecpdetR = crate::BitReader;
#[doc = "Field `FORCECPDET` writer - Force output charging port detected"]
pub type ForcecpdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDCPDET` reader - Force output dedicated charging port detected"]
pub type ForcedcpdetR = crate::BitReader;
#[doc = "Field `FORCEDCPDET` writer - Force output dedicated charging port detected"]
pub type ForcedcpdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCWEAKPULLUPTUNE` reader - Weak source resistor to both DP and DM tuning. Trimmable."]
pub type BcweakpulluptuneR = crate::FieldReader;
#[doc = "Field `BCWEAKPULLUPTUNE` writer - Weak source resistor to both DP and DM tuning. Trimmable."]
pub type BcweakpulluptuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BCWEAKPULLDOWNTUNE` reader - Weak sink resistor to both DP and DM tuning. Trimmable."]
pub type BcweakpulldowntuneR = crate::FieldReader;
#[doc = "Field `BCWEAKPULLDOWNTUNE` writer - Weak sink resistor to both DP and DM tuning. Trimmable."]
pub type BcweakpulldowntuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - BC detection bypass"]
    #[inline(always)]
    pub fn chargedetbyp(&self) -> ChargedetbypR {
        ChargedetbypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force output dp_attached"]
    #[inline(always)]
    pub fn forcedpattached(&self) -> ForcedpattachedR {
        ForcedpattachedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force output charging port detected"]
    #[inline(always)]
    pub fn forcecpdet(&self) -> ForcecpdetR {
        ForcecpdetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force output dedicated charging port detected"]
    #[inline(always)]
    pub fn forcedcpdet(&self) -> ForcedcpdetR {
        ForcedcpdetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Weak source resistor to both DP and DM tuning. Trimmable."]
    #[inline(always)]
    pub fn bcweakpulluptune(&self) -> BcweakpulluptuneR {
        BcweakpulluptuneR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Weak sink resistor to both DP and DM tuning. Trimmable."]
    #[inline(always)]
    pub fn bcweakpulldowntune(&self) -> BcweakpulldowntuneR {
        BcweakpulldowntuneR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BC detection bypass"]
    #[inline(always)]
    #[must_use]
    pub fn chargedetbyp(&mut self) -> ChargedetbypW<Bcdetcrtl2Spec> {
        ChargedetbypW::new(self, 0)
    }
    #[doc = "Bit 1 - Force output dp_attached"]
    #[inline(always)]
    #[must_use]
    pub fn forcedpattached(&mut self) -> ForcedpattachedW<Bcdetcrtl2Spec> {
        ForcedpattachedW::new(self, 1)
    }
    #[doc = "Bit 2 - Force output charging port detected"]
    #[inline(always)]
    #[must_use]
    pub fn forcecpdet(&mut self) -> ForcecpdetW<Bcdetcrtl2Spec> {
        ForcecpdetW::new(self, 2)
    }
    #[doc = "Bit 3 - Force output dedicated charging port detected"]
    #[inline(always)]
    #[must_use]
    pub fn forcedcpdet(&mut self) -> ForcedcpdetW<Bcdetcrtl2Spec> {
        ForcedcpdetW::new(self, 3)
    }
    #[doc = "Bits 8:9 - Weak source resistor to both DP and DM tuning. Trimmable."]
    #[inline(always)]
    #[must_use]
    pub fn bcweakpulluptune(&mut self) -> BcweakpulluptuneW<Bcdetcrtl2Spec> {
        BcweakpulluptuneW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Weak sink resistor to both DP and DM tuning. Trimmable."]
    #[inline(always)]
    #[must_use]
    pub fn bcweakpulldowntune(&mut self) -> BcweakpulldowntuneW<Bcdetcrtl2Spec> {
        BcweakpulldowntuneW::new(self, 10)
    }
}
#[doc = "Battery Charging auxillary detection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcdetcrtl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdetcrtl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcdetcrtl2Spec;
impl crate::RegisterSpec for Bcdetcrtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdetcrtl2::R`](R) reader structure"]
impl crate::Readable for Bcdetcrtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdetcrtl2::W`](W) writer structure"]
impl crate::Writable for Bcdetcrtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDETCRTL2 to value 0x0500"]
impl crate::Resettable for Bcdetcrtl2Spec {
    const RESET_VALUE: u32 = 0x0500;
}
