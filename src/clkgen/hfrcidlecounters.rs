#[doc = "Register `HFRCIDLECOUNTERS` reader"]
pub type R = crate::R<HfrcidlecountersSpec>;
#[doc = "Register `HFRCIDLECOUNTERS` writer"]
pub type W = crate::W<HfrcidlecountersSpec>;
#[doc = "Field `HFRCPWRDOWNDELAY` reader - Idle counter for HFRC POWER DOWN DELAY"]
pub type HfrcpwrdowndelayR = crate::FieldReader;
#[doc = "Field `HFRCPWRDOWNDELAY` writer - Idle counter for HFRC POWER DOWN DELAY"]
pub type HfrcpwrdowndelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HFRCCLKREQDELAY` reader - Idle counter for HFRC CLK REQ DELAY"]
pub type HfrcclkreqdelayR = crate::FieldReader;
#[doc = "Field `HFRCCLKREQDELAY` writer - Idle counter for HFRC CLK REQ DELAY"]
pub type HfrcclkreqdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HFRC2PWRDOWNDELAY` reader - Idle counter for HFRC2 POWER DOWN DELAY"]
pub type Hfrc2pwrdowndelayR = crate::FieldReader;
#[doc = "Field `HFRC2PWRDOWNDELAY` writer - Idle counter for HFRC2 POWER DOWN DELAY"]
pub type Hfrc2pwrdowndelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HFRC2CLKREQDELAY` reader - Enable for the PLL clock through clkgen"]
pub type Hfrc2clkreqdelayR = crate::FieldReader;
#[doc = "Field `HFRC2CLKREQDELAY` writer - Enable for the PLL clock through clkgen"]
pub type Hfrc2clkreqdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UPDATEENABLE` reader - usage : Clear UPDATEENABLE or 1'b0; Update other bits fields of this register. Set this register to 1'b1 for HW to update."]
pub type UpdateenableR = crate::BitReader;
#[doc = "Field `UPDATEENABLE` writer - usage : Clear UPDATEENABLE or 1'b0; Update other bits fields of this register. Set this register to 1'b1 for HW to update."]
pub type UpdateenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Idle counter for HFRC POWER DOWN DELAY"]
    #[inline(always)]
    pub fn hfrcpwrdowndelay(&self) -> HfrcpwrdowndelayR {
        HfrcpwrdowndelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Idle counter for HFRC CLK REQ DELAY"]
    #[inline(always)]
    pub fn hfrcclkreqdelay(&self) -> HfrcclkreqdelayR {
        HfrcclkreqdelayR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Idle counter for HFRC2 POWER DOWN DELAY"]
    #[inline(always)]
    pub fn hfrc2pwrdowndelay(&self) -> Hfrc2pwrdowndelayR {
        Hfrc2pwrdowndelayR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    pub fn hfrc2clkreqdelay(&self) -> Hfrc2clkreqdelayR {
        Hfrc2clkreqdelayR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - usage : Clear UPDATEENABLE or 1'b0; Update other bits fields of this register. Set this register to 1'b1 for HW to update."]
    #[inline(always)]
    pub fn updateenable(&self) -> UpdateenableR {
        UpdateenableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Idle counter for HFRC POWER DOWN DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcpwrdowndelay(&mut self) -> HfrcpwrdowndelayW<HfrcidlecountersSpec> {
        HfrcpwrdowndelayW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Idle counter for HFRC CLK REQ DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcclkreqdelay(&mut self) -> HfrcclkreqdelayW<HfrcidlecountersSpec> {
        HfrcclkreqdelayW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Idle counter for HFRC2 POWER DOWN DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn hfrc2pwrdowndelay(&mut self) -> Hfrc2pwrdowndelayW<HfrcidlecountersSpec> {
        Hfrc2pwrdowndelayW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Enable for the PLL clock through clkgen"]
    #[inline(always)]
    #[must_use]
    pub fn hfrc2clkreqdelay(&mut self) -> Hfrc2clkreqdelayW<HfrcidlecountersSpec> {
        Hfrc2clkreqdelayW::new(self, 24)
    }
    #[doc = "Bit 31 - usage : Clear UPDATEENABLE or 1'b0; Update other bits fields of this register. Set this register to 1'b1 for HW to update."]
    #[inline(always)]
    #[must_use]
    pub fn updateenable(&mut self) -> UpdateenableW<HfrcidlecountersSpec> {
        UpdateenableW::new(self, 31)
    }
}
#[doc = "Provides SW controlled # idle cycles before powering down HFRC, HFRC2., core clock enable(s)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcidlecounters::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcidlecounters::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrcidlecountersSpec;
impl crate::RegisterSpec for HfrcidlecountersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcidlecounters::R`](R) reader structure"]
impl crate::Readable for HfrcidlecountersSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrcidlecounters::W`](W) writer structure"]
impl crate::Writable for HfrcidlecountersSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFRCIDLECOUNTERS to value 0x0405_0405"]
impl crate::Resettable for HfrcidlecountersSpec {
    const RESET_VALUE: u32 = 0x0405_0405;
}
