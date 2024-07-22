#[doc = "Register `PGAADCIFCTRL` reader"]
pub type R = crate::R<PgaadcifctrlSpec>;
#[doc = "Register `PGAADCIFCTRL` writer"]
pub type W = crate::W<PgaadcifctrlSpec>;
#[doc = "Field `PGAADCIFCHAACTIVE` reader - PGAADCIF active signal for channels A0 and A1. Starts and stops 2 clocks after demultiplexed SOC signal."]
pub type PgaadcifchaactiveR = crate::FieldReader;
#[doc = "Field `PGAADCIFCHAACTIVE` writer - PGAADCIF active signal for channels A0 and A1. Starts and stops 2 clocks after demultiplexed SOC signal."]
pub type PgaadcifchaactiveW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGAADCIFCHAPDNB` reader - Power down for channels A0 and A1 (0 = powered down; 1 = standby)"]
pub type PgaadcifchapdnbR = crate::FieldReader;
#[doc = "Field `PGAADCIFCHAPDNB` writer - Power down for channels A0 and A1 (0 = powered down; 1 = standby)"]
pub type PgaadcifchapdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGAADCIFCHBACTIVE` reader - PGAADCIF active signal for channels B0 and B1. Starts and stops 2 clocks after demultiplexed SOC signal."]
pub type PgaadcifchbactiveR = crate::FieldReader;
#[doc = "Field `PGAADCIFCHBACTIVE` writer - PGAADCIF active signal for channels B0 and B1. Starts and stops 2 clocks after demultiplexed SOC signal."]
pub type PgaadcifchbactiveW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGAADCIFCHBPDNB` reader - Power down for channels B0 and B1 (0 = powered down; 1 = standby)"]
pub type PgaadcifchbpdnbR = crate::FieldReader;
#[doc = "Field `PGAADCIFCHBPDNB` writer - Power down for channels B0 and B1 (0 = powered down; 1 = standby)"]
pub type PgaadcifchbpdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGAADCIFVCOMPEN` reader - Enable for VCOMP output"]
pub type PgaadcifvcompenR = crate::BitReader;
#[doc = "Field `PGAADCIFVCOMPEN` writer - Enable for VCOMP output"]
pub type PgaadcifvcompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAADCIFVCOMPSEL` reader - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
pub type PgaadcifvcompselR = crate::FieldReader;
#[doc = "Field `PGAADCIFVCOMPSEL` writer - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
pub type PgaadcifvcompselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PGAADCIF active signal for channels A0 and A1. Starts and stops 2 clocks after demultiplexed SOC signal."]
    #[inline(always)]
    pub fn pgaadcifchaactive(&self) -> PgaadcifchaactiveR {
        PgaadcifchaactiveR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Power down for channels A0 and A1 (0 = powered down; 1 = standby)"]
    #[inline(always)]
    pub fn pgaadcifchapdnb(&self) -> PgaadcifchapdnbR {
        PgaadcifchapdnbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PGAADCIF active signal for channels B0 and B1. Starts and stops 2 clocks after demultiplexed SOC signal."]
    #[inline(always)]
    pub fn pgaadcifchbactive(&self) -> PgaadcifchbactiveR {
        PgaadcifchbactiveR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Power down for channels B0 and B1 (0 = powered down; 1 = standby)"]
    #[inline(always)]
    pub fn pgaadcifchbpdnb(&self) -> PgaadcifchbpdnbR {
        PgaadcifchbpdnbR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable for VCOMP output"]
    #[inline(always)]
    pub fn pgaadcifvcompen(&self) -> PgaadcifvcompenR {
        PgaadcifvcompenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
    #[inline(always)]
    pub fn pgaadcifvcompsel(&self) -> PgaadcifvcompselR {
        PgaadcifvcompselR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PGAADCIF active signal for channels A0 and A1. Starts and stops 2 clocks after demultiplexed SOC signal."]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifchaactive(&mut self) -> PgaadcifchaactiveW<PgaadcifctrlSpec> {
        PgaadcifchaactiveW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power down for channels A0 and A1 (0 = powered down; 1 = standby)"]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifchapdnb(&mut self) -> PgaadcifchapdnbW<PgaadcifctrlSpec> {
        PgaadcifchapdnbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PGAADCIF active signal for channels B0 and B1. Starts and stops 2 clocks after demultiplexed SOC signal."]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifchbactive(&mut self) -> PgaadcifchbactiveW<PgaadcifctrlSpec> {
        PgaadcifchbactiveW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Power down for channels B0 and B1 (0 = powered down; 1 = standby)"]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifchbpdnb(&mut self) -> PgaadcifchbpdnbW<PgaadcifctrlSpec> {
        PgaadcifchbpdnbW::new(self, 6)
    }
    #[doc = "Bit 12 - Enable for VCOMP output"]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifvcompen(&mut self) -> PgaadcifvcompenW<PgaadcifctrlSpec> {
        PgaadcifvcompenW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
    #[inline(always)]
    #[must_use]
    pub fn pgaadcifvcompsel(&mut self) -> PgaadcifvcompselW<PgaadcifctrlSpec> {
        PgaadcifvcompselW::new(self, 13)
    }
}
#[doc = "PGA ADCIF control\n\nYou can [`read`](crate::Reg::read) this register and get [`pgaadcifctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgaadcifctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgaadcifctrlSpec;
impl crate::RegisterSpec for PgaadcifctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgaadcifctrl::R`](R) reader structure"]
impl crate::Readable for PgaadcifctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pgaadcifctrl::W`](W) writer structure"]
impl crate::Writable for PgaadcifctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGAADCIFCTRL to value 0"]
impl crate::Resettable for PgaadcifctrlSpec {
    const RESET_VALUE: u32 = 0;
}
