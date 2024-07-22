#[doc = "Register `PGACTRL2` reader"]
pub type R = crate::R<Pgactrl2Spec>;
#[doc = "Register `PGACTRL2` writer"]
pub type W = crate::W<Pgactrl2Spec>;
#[doc = "Field `PGACHB0GAIN1SEL` reader - Channel B0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgachb0gain1selR = crate::FieldReader;
#[doc = "Field `PGACHB0GAIN1SEL` writer - Channel B0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgachb0gain1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGACHB0GAIN2DIV2SEL` reader - Channel B0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgachb0gain2div2selR = crate::BitReader;
#[doc = "Field `PGACHB0GAIN2DIV2SEL` writer - Channel B0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgachb0gain2div2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHB0GAIN2SEL` reader - Channel B0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgachb0gain2selR = crate::FieldReader;
#[doc = "Field `PGACHB0GAIN2SEL` writer - Channel B0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgachb0gain2selW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PGACHB1GAIN1SEL` reader - Channel B1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgachb1gain1selR = crate::FieldReader;
#[doc = "Field `PGACHB1GAIN1SEL` writer - Channel B1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
pub type Pgachb1gain1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGACHB1GAIN2DIV2SEL` reader - Channel B1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgachb1gain2div2selR = crate::BitReader;
#[doc = "Field `PGACHB1GAIN2DIV2SEL` writer - Channel B1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
pub type Pgachb1gain2div2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHB1GAIN2SEL` reader - Channel B1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgachb1gain2selR = crate::FieldReader;
#[doc = "Field `PGACHB1GAIN2SEL` writer - Channel B1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
pub type Pgachb1gain2selW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PGACHBBYPASSEN` reader - Bypass enable for Channels B0 and B1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
pub type PgachbbypassenR = crate::FieldReader;
#[doc = "Field `PGACHBBYPASSEN` writer - Bypass enable for Channels B0 and B1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
pub type PgachbbypassenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHBOPAMPINPDNB` reader - Channels B0 and B1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHBBYPASSEN = 0."]
pub type PgachbopampinpdnbR = crate::FieldReader;
#[doc = "Field `PGACHBOPAMPINPDNB` writer - Channels B0 and B1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHBBYPASSEN = 0."]
pub type PgachbopampinpdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHBOPAMPOUTPDNB` reader - Channels B0 and B1 output stage opamp power down (0: powered down, 1: powered up)"]
pub type PgachbopampoutpdnbR = crate::FieldReader;
#[doc = "Field `PGACHBOPAMPOUTPDNB` writer - Channels B0 and B1 output stage opamp power down (0: powered down, 1: powered up)"]
pub type PgachbopampoutpdnbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGACHBVCMGENPDNB` reader - Channel B VCMGEN power down (0: powered down, 1: powered up)"]
pub type PgachbvcmgenpdnbR = crate::BitReader;
#[doc = "Field `PGACHBVCMGENPDNB` writer - Channel B VCMGEN power down (0: powered down, 1: powered up)"]
pub type PgachbvcmgenpdnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGACHBVCMGENQCHARGEEN` reader - Channel B VCMGEN quick charge enable (pulsed during channel powerup)"]
pub type PgachbvcmgenqchargeenR = crate::BitReader;
#[doc = "Field `PGACHBVCMGENQCHARGEEN` writer - Channel B VCMGEN quick charge enable (pulsed during channel powerup)"]
pub type PgachbvcmgenqchargeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAGAINBOVRD` reader - Apply BYPASS and GAIN bits from this register (for channel B) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
pub type PgagainbovrdR = crate::BitReader;
#[doc = "Field `PGAGAINBOVRD` writer - Apply BYPASS and GAIN bits from this register (for channel B) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
pub type PgagainbovrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Channel B0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    pub fn pgachb0gain1sel(&self) -> Pgachb0gain1selR {
        Pgachb0gain1selR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Channel B0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    pub fn pgachb0gain2div2sel(&self) -> Pgachb0gain2div2selR {
        Pgachb0gain2div2selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Channel B0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    pub fn pgachb0gain2sel(&self) -> Pgachb0gain2selR {
        Pgachb0gain2selR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11 - Channel B1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    pub fn pgachb1gain1sel(&self) -> Pgachb1gain1selR {
        Pgachb1gain1selR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Channel B1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    pub fn pgachb1gain2div2sel(&self) -> Pgachb1gain2div2selR {
        Pgachb1gain2div2selR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Channel B1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    pub fn pgachb1gain2sel(&self) -> Pgachb1gain2selR {
        Pgachb1gain2selR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:19 - Bypass enable for Channels B0 and B1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
    #[inline(always)]
    pub fn pgachbbypassen(&self) -> PgachbbypassenR {
        PgachbbypassenR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Channels B0 and B1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHBBYPASSEN = 0."]
    #[inline(always)]
    pub fn pgachbopampinpdnb(&self) -> PgachbopampinpdnbR {
        PgachbopampinpdnbR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Channels B0 and B1 output stage opamp power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgachbopampoutpdnb(&self) -> PgachbopampoutpdnbR {
        PgachbopampoutpdnbR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Channel B VCMGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    pub fn pgachbvcmgenpdnb(&self) -> PgachbvcmgenpdnbR {
        PgachbvcmgenpdnbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel B VCMGEN quick charge enable (pulsed during channel powerup)"]
    #[inline(always)]
    pub fn pgachbvcmgenqchargeen(&self) -> PgachbvcmgenqchargeenR {
        PgachbvcmgenqchargeenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Apply BYPASS and GAIN bits from this register (for channel B) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
    #[inline(always)]
    pub fn pgagainbovrd(&self) -> PgagainbovrdR {
        PgagainbovrdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel B0 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb0gain1sel(&mut self) -> Pgachb0gain1selW<Pgactrl2Spec> {
        Pgachb0gain1selW::new(self, 0)
    }
    #[doc = "Bit 3 - Channel B0 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb0gain2div2sel(&mut self) -> Pgachb0gain2div2selW<Pgactrl2Spec> {
        Pgachb0gain2div2selW::new(self, 3)
    }
    #[doc = "Bits 4:8 - Channel B0 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb0gain2sel(&mut self) -> Pgachb0gain2selW<Pgactrl2Spec> {
        Pgachb0gain2selW::new(self, 4)
    }
    #[doc = "Bits 9:11 - Channel B1 preamp gain (0: 12dB, ..., 7: 33dB in 3 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb1gain1sel(&mut self) -> Pgachb1gain1selW<Pgactrl2Spec> {
        Pgachb1gain1selW::new(self, 9)
    }
    #[doc = "Bit 12 - Channel B1 PGA divide by two select (0: 0 dB, 1: -6dB), needed for fully differential inputs"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb1gain2div2sel(&mut self) -> Pgachb1gain2div2selW<Pgactrl2Spec> {
        Pgachb1gain2div2selW::new(self, 12)
    }
    #[doc = "Bits 13:17 - Channel B1 PGA gain (0: 0dB, ..., 23: 11.5dB in 0.5 dB steps)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachb1gain2sel(&mut self) -> Pgachb1gain2selW<Pgactrl2Spec> {
        Pgachb1gain2selW::new(self, 13)
    }
    #[doc = "Bits 18:19 - Bypass enable for Channels B0 and B1 (1: bypass, when gain LT 12 dB; 0: otherwise)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachbbypassen(&mut self) -> PgachbbypassenW<Pgactrl2Spec> {
        PgachbbypassenW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Channels B0 and B1 input stage opamp power down (0: powered down, 1: powered up). Must be 1 when respective PGACHBBYPASSEN = 0."]
    #[inline(always)]
    #[must_use]
    pub fn pgachbopampinpdnb(&mut self) -> PgachbopampinpdnbW<Pgactrl2Spec> {
        PgachbopampinpdnbW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Channels B0 and B1 output stage opamp power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachbopampoutpdnb(&mut self) -> PgachbopampoutpdnbW<Pgactrl2Spec> {
        PgachbopampoutpdnbW::new(self, 22)
    }
    #[doc = "Bit 24 - Channel B VCMGEN power down (0: powered down, 1: powered up)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachbvcmgenpdnb(&mut self) -> PgachbvcmgenpdnbW<Pgactrl2Spec> {
        PgachbvcmgenpdnbW::new(self, 24)
    }
    #[doc = "Bit 25 - Channel B VCMGEN quick charge enable (pulsed during channel powerup)"]
    #[inline(always)]
    #[must_use]
    pub fn pgachbvcmgenqchargeen(&mut self) -> PgachbvcmgenqchargeenW<Pgactrl2Spec> {
        PgachbvcmgenqchargeenW::new(self, 25)
    }
    #[doc = "Bit 31 - Apply BYPASS and GAIN bits from this register (for channel B) instead of automatically via audio ADC. Note that audio ADC FIFO meta data will not reflect dB gain as used when configuring audio ADC."]
    #[inline(always)]
    #[must_use]
    pub fn pgagainbovrd(&mut self) -> PgagainbovrdW<Pgactrl2Spec> {
        PgagainbovrdW::new(self, 31)
    }
}
#[doc = "PGA control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pgactrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgactrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pgactrl2Spec;
impl crate::RegisterSpec for Pgactrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgactrl2::R`](R) reader structure"]
impl crate::Readable for Pgactrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pgactrl2::W`](W) writer structure"]
impl crate::Writable for Pgactrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PGACTRL2 to value 0"]
impl crate::Resettable for Pgactrl2Spec {
    const RESET_VALUE: u32 = 0;
}
