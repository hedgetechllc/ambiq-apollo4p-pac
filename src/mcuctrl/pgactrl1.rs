#[doc = "Register `PGACTRL1` reader"]
pub type R = crate::R<Pgactrl1Spec>;
#[doc = "Register `PGACTRL1` writer"]
pub type W = crate::W<Pgactrl1Spec>;
#[doc = "Field `PGACHA0GAIN1SEL` reader - Channel A0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgacha0gain1selR = crate::FieldReader;
#[doc = "Field `PGACHA0GAIN1SEL` writer - Channel A0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgacha0gain1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGACHA0GAIN2DIV2SEL` reader - Channel A0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgacha0gain2div2selR = crate::BitReader;
#[doc = "Field `PGACHA0GAIN2DIV2SEL` writer - Channel A0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgacha0gain2div2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHA0GAIN2SEL` reader - Channel A0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgacha0gain2selR = crate::FieldReader;
#[doc = "Field `PGACHA0GAIN2SEL` writer - Channel A0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgacha0gain2selW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PGACHA1GAIN1SEL` reader - Channel A1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgacha1gain1selR = crate::FieldReader;
#[doc = "Field `PGACHA1GAIN1SEL` writer - Channel A1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgacha1gain1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGACHA1GAIN2DIV2SEL` reader - Channel A1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgacha1gain2div2selR = crate::BitReader;
#[doc = "Field `PGACHA1GAIN2DIV2SEL` writer - Channel A1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgacha1gain2div2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHA1GAIN2SEL` reader - Channel A1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgacha1gain2selR = crate::FieldReader;
#[doc = "Field `PGACHA1GAIN2SEL` writer - Channel A1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgacha1gain2selW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PGACHABYPASSEN` reader - Bypass enable for Channels A0 and A1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
pub type PgachabypassenR = crate::FieldReader;
#[doc = "Field `PGACHABYPASSEN` writer - Bypass enable for Channels A0 and A1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
pub type PgachabypassenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHAOPAMPINPDNB` reader - Channels A0 and A1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHABYPASSEN = 0."]
pub type PgachaopampinpdnbR = crate::FieldReader;
#[doc = "Field `PGACHAOPAMPINPDNB` writer - Channels A0 and A1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHABYPASSEN = 0."]
pub type PgachaopampinpdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHAOPAMPOUTPDNB` reader - Channels A0 and A1 output stage opamp power down (0: powered down, 1: powered up)"]
pub type PgachaopampoutpdnbR = crate::FieldReader;
#[doc = "Field `PGACHAOPAMPOUTPDNB` writer - Channels A0 and A1 output stage opamp power down (0: powered down, 1: powered up)"]
pub type PgachaopampoutpdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHAVCMGENPDNB` reader - Channel A VCMGEN power down (0: powered down, 1: powered up)"]
pub type PgachavcmgenpdnbR = crate::BitReader;
#[doc = "Field `PGACHAVCMGENPDNB` writer - Channel A VCMGEN power down (0: powered down, 1: powered up)"]
pub type PgachavcmgenpdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHAVCMGENQCHARGEEN` reader - Channel A VCMGEN quick charge enable (pulsed during channel powerup)"]
pub type PgachavcmgenqchargeenR = crate::BitReader;
#[doc = "Field `PGACHAVCMGENQCHARGEEN` writer - Channel A VCMGEN quick charge enable (pulsed during channel powerup)"]
pub type PgachavcmgenqchargeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAIREFGENPDNB` reader - IREFGEN power down (0: powered down, 1: powered up)"]
pub type PgairefgenpdnbR = crate::BitReader;
#[doc = "Field `PGAIREFGENPDNB` writer - IREFGEN power down (0: powered down, 1: powered up)"]
pub type PgairefgenpdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAVREFGENPDNB` reader - VREFGEN power down (0: powered down, 1: powered up)"]
pub type PgavrefgenpdnbR = crate::BitReader;
#[doc = "Field `PGAVREFGENPDNB` writer - VREFGEN power down (0: powered down, 1: powered up)"]
pub type PgavrefgenpdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAVREFGENQUICKSTARTEN` reader - VREFGEN quick start enable (pulsed during startup)"]
pub type PgavrefgenquickstartenR = crate::BitReader;
#[doc = "Field `PGAVREFGENQUICKSTARTEN` writer - VREFGEN quick start enable (pulsed during startup)"]
pub type PgavrefgenquickstartenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCOMPSELPGA` reader - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
pub type VcompselpgaR = crate::BitReader;
#[doc = "Field `VCOMPSELPGA` writer - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
pub type VcompselpgaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAGAINAOVRD` reader - Apply BYPASS and GAIN bits from this register (for channel A) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
pub type PgagainaovrdR = crate::BitReader;
#[doc = "Field `PGAGAINAOVRD` writer - Apply BYPASS and GAIN bits from this register (for channel A) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
pub type PgagainaovrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Channel A0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    pub fn pgacha0gain1sel(&self) -> Pgacha0gain1selR {
        Pgacha0gain1selR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel A0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    pub fn pgacha0gain2div2sel(&self) -> Pgacha0gain2div2selR {
        Pgacha0gain2div2selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Channel A0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    pub fn pgacha0gain2sel(&self) -> Pgacha0gain2selR {
        Pgacha0gain2selR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11 - Channel A1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    pub fn pgacha1gain1sel(&self) -> Pgacha1gain1selR {
        Pgacha1gain1selR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Channel A1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    pub fn pgacha1gain2div2sel(&self) -> Pgacha1gain2div2selR {
        Pgacha1gain2div2selR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Channel A1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    pub fn pgacha1gain2sel(&self) -> Pgacha1gain2selR {
        Pgacha1gain2selR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:19 - Bypass enable for Channels A0 and A1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
    #[inline(always)]
    pub fn pgachabypassen(&self) -> PgachabypassenR {
        PgachabypassenR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channels A0 and A1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHABYPASSEN = 0."]
    #[inline(always)]
    pub fn pgachaopampinpdnb(&self) -> PgachaopampinpdnbR {
        PgachaopampinpdnbR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channels A0 and A1 output stage opamp power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgachaopampoutpdnb(&self) -> PgachaopampoutpdnbR {
        PgachaopampoutpdnbR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Channel A VCMGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgachavcmgenpdnb(&self) -> PgachavcmgenpdnbR {
        PgachavcmgenpdnbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel A VCMGEN quick charge enable (pulsed during channel powerup)"]
    #[inline(always)]
    pub fn pgachavcmgenqchargeen(&self) -> PgachavcmgenqchargeenR {
        PgachavcmgenqchargeenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IREFGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgairefgenpdnb(&self) -> PgairefgenpdnbR {
        PgairefgenpdnbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - VREFGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgavrefgenpdnb(&self) -> PgavrefgenpdnbR {
        PgavrefgenpdnbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - VREFGEN quick start enable (pulsed during startup)"]
    #[inline(always)]
    pub fn pgavrefgenquickstarten(&self) -> PgavrefgenquickstartenR {
        PgavrefgenquickstartenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
    #[inline(always)]
    pub fn vcompselpga(&self) -> VcompselpgaR {
        VcompselpgaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Apply BYPASS and GAIN bits from this register (for channel A) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
    #[inline(always)]
    pub fn pgagainaovrd(&self) -> PgagainaovrdR {
        PgagainaovrdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel A0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha0gain1sel(&mut self) -> Pgacha0gain1selW<Pgactrl1Spec> {
        Pgacha0gain1selW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel A0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha0gain2div2sel(&mut self) -> Pgacha0gain2div2selW<Pgactrl1Spec> {
        Pgacha0gain2div2selW::new(self, 3)
    }
    #[doc = "Bits 4:8 - Channel A0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha0gain2sel(&mut self) -> Pgacha0gain2selW<Pgactrl1Spec> {
        Pgacha0gain2selW::new(self, 4)
    }
    #[doc = "Bits 9:11 - Channel A1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha1gain1sel(&mut self) -> Pgacha1gain1selW<Pgactrl1Spec> {
        Pgacha1gain1selW::new(self, 9)
    }
    #[doc = "Bit 12 - Channel A1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha1gain2div2sel(&mut self) -> Pgacha1gain2div2selW<Pgactrl1Spec> {
        Pgacha1gain2div2selW::new(self, 12)
    }
    #[doc = "Bits 13:17 - Channel A1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgacha1gain2sel(&mut self) -> Pgacha1gain2selW<Pgactrl1Spec> {
        Pgacha1gain2selW::new(self, 13)
    }
    #[doc = "Bits 18:19 - Bypass enable for Channels A0 and A1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachabypassen(&mut self) -> PgachabypassenW<Pgactrl1Spec> {
        PgachabypassenW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channels A0 and A1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHABYPASSEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn pgachaopampinpdnb(&mut self) -> PgachaopampinpdnbW<Pgactrl1Spec> {
        PgachaopampinpdnbW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Channels A0 and A1 output stage opamp power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachaopampoutpdnb(&mut self) -> PgachaopampoutpdnbW<Pgactrl1Spec> {
        PgachaopampoutpdnbW::new(self, 22)
    }
    #[doc = "Bit 24 - Channel A VCMGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachavcmgenpdnb(&mut self) -> PgachavcmgenpdnbW<Pgactrl1Spec> {
        PgachavcmgenpdnbW::new(self, 24)
    }
    #[doc = "Bit 25 - Channel A VCMGEN quick charge enable (pulsed during channel powerup)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachavcmgenqchargeen(&mut self) -> PgachavcmgenqchargeenW<Pgactrl1Spec> {
        PgachavcmgenqchargeenW::new(self, 25)
    }
    #[doc = "Bit 26 - IREFGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgairefgenpdnb(&mut self) -> PgairefgenpdnbW<Pgactrl1Spec> {
        PgairefgenpdnbW::new(self, 26)
    }
    #[doc = "Bit 27 - VREFGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgavrefgenpdnb(&mut self) -> PgavrefgenpdnbW<Pgactrl1Spec> {
        PgavrefgenpdnbW::new(self, 27)
    }
    #[doc = "Bit 28 - VREFGEN quick start enable (pulsed during startup)"]
    #[inline(always)]
    #[must_use]
    pub fn pgavrefgenquickstarten(&mut self) -> PgavrefgenquickstartenW<Pgactrl1Spec> {
        PgavrefgenquickstartenW::new(self, 28)
    }
    #[doc = "Bit 29 - Select for VCOMP output (0: A0, 1: A1, 2: B0, 3: B1)"]
    #[inline(always)]
    #[must_use]
    pub fn vcompselpga(&mut self) -> VcompselpgaW<Pgactrl1Spec> {
        VcompselpgaW::new(self, 29)
    }
    #[doc = "Bit 31 - Apply BYPASS and GAIN bits from this register (for channel A) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
    #[inline(always)]
    #[must_use]
    pub fn pgagainaovrd(&mut self) -> PgagainaovrdW<Pgactrl1Spec> {
        PgagainaovrdW::new(self, 31)
    }
}
#[doc = "PGA control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pgactrl1Spec;
impl crate::RegisterSpec for Pgactrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgactrl1::R`](R) reader structure"]
impl crate::Readable for Pgactrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pgactrl1::W`](W) writer structure"]
impl crate::Writable for Pgactrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGACTRL1 to value 0"]
impl crate::Resettable for Pgactrl1Spec {
    const RESET_VALUE: u32 = 0;
}
