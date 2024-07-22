#[doc = "Register `GAIN` reader"]
pub type R = crate::R<GainSpec>;
#[doc = "Register `GAIN` writer"]
pub type W = crate::W<GainSpec>;
#[doc = "Field `LGA` reader - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel A (slot 0)."]
pub type LgaR = crate::FieldReader;
#[doc = "Field `LGA` writer - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel A (slot 0)."]
pub type LgaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HGADELTA` reader - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGA field for channel A (slot 1). Note that HGADELTA must be LE (24 - LGA) dB."]
pub type HgadeltaR = crate::FieldReader;
#[doc = "Field `HGADELTA` writer - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGA field for channel A (slot 1). Note that HGADELTA must be LE (24 - LGA) dB."]
pub type HgadeltaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LGB` reader - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel B (slot 2)."]
pub type LgbR = crate::FieldReader;
#[doc = "Field `LGB` writer - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel B (slot 2)."]
pub type LgbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HGBDELTA` reader - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGB field for channel B (slot 3). Note that HGBDELTA must be LE (24 - LGB) dB."]
pub type HgbdeltaR = crate::FieldReader;
#[doc = "Field `HGBDELTA` writer - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGB field for channel B (slot 3). Note that HGBDELTA must be LE (24 - LGB) dB."]
pub type HgbdeltaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel A (slot 0)."]
    #[inline(always)]
    pub fn lga(&self) -> LgaR {
        LgaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGA field for channel A (slot 1). Note that HGADELTA must be LE (24 - LGA) dB."]
    #[inline(always)]
    pub fn hgadelta(&self) -> HgadeltaR {
        HgadeltaR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel B (slot 2)."]
    #[inline(always)]
    pub fn lgb(&self) -> LgbR {
        LgbR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGB field for channel B (slot 3). Note that HGBDELTA must be LE (24 - LGB) dB."]
    #[inline(always)]
    pub fn hgbdelta(&self) -> HgbdeltaR {
        HgbdeltaR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel A (slot 0)."]
    #[inline(always)]
    #[must_use]
    pub fn lga(&mut self) -> LgaW<GainSpec> {
        LgaW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGA field for channel A (slot 1). Note that HGADELTA must be LE (24 - LGA) dB."]
    #[inline(always)]
    #[must_use]
    pub fn hgadelta(&mut self) -> HgadeltaW<GainSpec> {
        HgadeltaW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Specifies the low gain code (0 to 60 decimal specifies -6.0 dB to 24.0 dB in half-dB increments) for channel B (slot 2)."]
    #[inline(always)]
    #[must_use]
    pub fn lgb(&mut self) -> LgbW<GainSpec> {
        LgbW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Specifies the high gain code (0 to 60 decimal specifies 0 dB to 30.0 dB in half-dB increments) as the delta from the LGB field for channel B (slot 3). Note that HGBDELTA must be LE (24 - LGB) dB."]
    #[inline(always)]
    #[must_use]
    pub fn hgbdelta(&mut self) -> HgbdeltaW<GainSpec> {
        HgbdeltaW::new(self, 24)
    }
}
#[doc = "PGA Gain Codes\n\nYou can [`read`](crate::Reg::read) this register and get [`gain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GainSpec;
impl crate::RegisterSpec for GainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gain::R`](R) reader structure"]
impl crate::Readable for GainSpec {}
#[doc = "`write(|w| ..)` method takes [`gain::W`](W) writer structure"]
impl crate::Writable for GainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAIN to value 0"]
impl crate::Resettable for GainSpec {
    const RESET_VALUE: u32 = 0;
}
