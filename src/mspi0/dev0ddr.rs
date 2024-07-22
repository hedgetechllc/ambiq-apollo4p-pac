#[doc = "Register `DEV0DDR` reader"]
pub type R = crate::R<Dev0ddrSpec>;
#[doc = "Register `DEV0DDR` writer"]
pub type W = crate::W<Dev0ddrSpec>;
#[doc = "Field `EMULATEDDR0` reader - Drive external clock at 1/2 rate to emulate DDR mode"]
pub type Emulateddr0R = crate::BitReader;
#[doc = "Field `EMULATEDDR0` writer - Drive external clock at 1/2 rate to emulate DDR mode"]
pub type Emulateddr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUADDDR0` reader - Deprecated. No effect on RevC."]
pub type Quadddr0R = crate::BitReader;
#[doc = "Field `QUADDDR0` writer - Deprecated. No effect on RevC."]
pub type Quadddr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEDQS0` reader - In EMULATEDDR mode, enable DQS for read capture"]
pub type Enabledqs0R = crate::BitReader;
#[doc = "Field `ENABLEDQS0` writer - In EMULATEDDR mode, enable DQS for read capture"]
pub type Enabledqs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSSYNCNEG0` reader - Use negative edge of clock for DDR data sync"]
pub type Dqssyncneg0R = crate::BitReader;
#[doc = "Field `DQSSYNCNEG0` writer - Use negative edge of clock for DDR data sync"]
pub type Dqssyncneg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLEFINEDELAY0` reader - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Enablefinedelay0R = crate::BitReader;
#[doc = "Field `ENABLEFINEDELAY0` writer - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Enablefinedelay0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDQSDELAY0` reader - This acts as an offset to the computed value (should be set to 0 by default)"]
pub type Txdqsdelay0R = crate::FieldReader;
#[doc = "Field `TXDQSDELAY0` writer - This acts as an offset to the computed value (should be set to 0 by default)"]
pub type Txdqsdelay0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDQSDELAY0` reader - This acts as an offset to the computed value (should be set to 0 by default)"]
pub type Rxdqsdelay0R = crate::FieldReader;
#[doc = "Field `RXDQSDELAY0` writer - This acts as an offset to the computed value (should be set to 0 by default)"]
pub type Rxdqsdelay0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDQSDELAYNEG0` reader - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelayneg0R = crate::FieldReader;
#[doc = "Field `RXDQSDELAYNEG0` writer - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelayneg0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDQSDELAYNEGEN0` reader - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelaynegen0R = crate::BitReader;
#[doc = "Field `RXDQSDELAYNEGEN0` writer - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelaynegen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDQSDELAYHI0` reader - This acts as an offset to the computed value (should be set to 0 by default) for 2nd DQS on HEX mode."]
pub type Rxdqsdelayhi0R = crate::FieldReader;
#[doc = "Field `RXDQSDELAYHI0` writer - This acts as an offset to the computed value (should be set to 0 by default) for 2nd DQS on HEX mode."]
pub type Rxdqsdelayhi0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDQSDELAYNEGHI0` reader - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelayneghi0R = crate::FieldReader;
#[doc = "Field `RXDQSDELAYNEGHI0` writer - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
pub type Rxdqsdelayneghi0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RXDQSDELAYHIEN0` reader - When 1, RXDQSDELAYHI and RXDQSDELAYNEGHI is used for falling edge of the clock."]
pub type Rxdqsdelayhien0R = crate::BitReader;
#[doc = "Field `RXDQSDELAYHIEN0` writer - When 1, RXDQSDELAYHI and RXDQSDELAYNEGHI is used for falling edge of the clock."]
pub type Rxdqsdelayhien0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drive external clock at 1/2 rate to emulate DDR mode"]
    #[inline(always)]
    pub fn emulateddr0(&self) -> Emulateddr0R {
        Emulateddr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deprecated. No effect on RevC."]
    #[inline(always)]
    pub fn quadddr0(&self) -> Quadddr0R {
        Quadddr0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In EMULATEDDR mode, enable DQS for read capture"]
    #[inline(always)]
    pub fn enabledqs0(&self) -> Enabledqs0R {
        Enabledqs0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use negative edge of clock for DDR data sync"]
    #[inline(always)]
    pub fn dqssyncneg0(&self) -> Dqssyncneg0R {
        Dqssyncneg0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    pub fn enablefinedelay0(&self) -> Enablefinedelay0R {
        Enablefinedelay0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - This acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn txdqsdelay0(&self) -> Txdqsdelay0R {
        Txdqsdelay0R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - This acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn rxdqsdelay0(&self) -> Rxdqsdelay0R {
        Rxdqsdelay0R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    pub fn rxdqsdelayneg0(&self) -> Rxdqsdelayneg0R {
        Rxdqsdelayneg0R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    pub fn rxdqsdelaynegen0(&self) -> Rxdqsdelaynegen0R {
        Rxdqsdelaynegen0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:25 - This acts as an offset to the computed value (should be set to 0 by default) for 2nd DQS on HEX mode."]
    #[inline(always)]
    pub fn rxdqsdelayhi0(&self) -> Rxdqsdelayhi0R {
        Rxdqsdelayhi0R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    pub fn rxdqsdelayneghi0(&self) -> Rxdqsdelayneghi0R {
        Rxdqsdelayneghi0R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - When 1, RXDQSDELAYHI and RXDQSDELAYNEGHI is used for falling edge of the clock."]
    #[inline(always)]
    pub fn rxdqsdelayhien0(&self) -> Rxdqsdelayhien0R {
        Rxdqsdelayhien0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive external clock at 1/2 rate to emulate DDR mode"]
    #[inline(always)]
    #[must_use]
    pub fn emulateddr0(&mut self) -> Emulateddr0W<Dev0ddrSpec> {
        Emulateddr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Deprecated. No effect on RevC."]
    #[inline(always)]
    #[must_use]
    pub fn quadddr0(&mut self) -> Quadddr0W<Dev0ddrSpec> {
        Quadddr0W::new(self, 1)
    }
    #[doc = "Bit 2 - In EMULATEDDR mode, enable DQS for read capture"]
    #[inline(always)]
    #[must_use]
    pub fn enabledqs0(&mut self) -> Enabledqs0W<Dev0ddrSpec> {
        Enabledqs0W::new(self, 2)
    }
    #[doc = "Bit 3 - Use negative edge of clock for DDR data sync"]
    #[inline(always)]
    #[must_use]
    pub fn dqssyncneg0(&mut self) -> Dqssyncneg0W<Dev0ddrSpec> {
        Dqssyncneg0W::new(self, 3)
    }
    #[doc = "Bit 4 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    #[must_use]
    pub fn enablefinedelay0(&mut self) -> Enablefinedelay0W<Dev0ddrSpec> {
        Enablefinedelay0W::new(self, 4)
    }
    #[doc = "Bits 5:9 - This acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    #[must_use]
    pub fn txdqsdelay0(&mut self) -> Txdqsdelay0W<Dev0ddrSpec> {
        Txdqsdelay0W::new(self, 5)
    }
    #[doc = "Bits 10:14 - This acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelay0(&mut self) -> Rxdqsdelay0W<Dev0ddrSpec> {
        Rxdqsdelay0W::new(self, 10)
    }
    #[doc = "Bits 15:19 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelayneg0(&mut self) -> Rxdqsdelayneg0W<Dev0ddrSpec> {
        Rxdqsdelayneg0W::new(self, 15)
    }
    #[doc = "Bit 20 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelaynegen0(&mut self) -> Rxdqsdelaynegen0W<Dev0ddrSpec> {
        Rxdqsdelaynegen0W::new(self, 20)
    }
    #[doc = "Bits 21:25 - This acts as an offset to the computed value (should be set to 0 by default) for 2nd DQS on HEX mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelayhi0(&mut self) -> Rxdqsdelayhi0W<Dev0ddrSpec> {
        Rxdqsdelayhi0W::new(self, 21)
    }
    #[doc = "Bits 26:30 - ERR109 - Field has no effect in the Apollo4 Plus and should remain at the default value (0)."]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelayneghi0(&mut self) -> Rxdqsdelayneghi0W<Dev0ddrSpec> {
        Rxdqsdelayneghi0W::new(self, 26)
    }
    #[doc = "Bit 31 - When 1, RXDQSDELAYHI and RXDQSDELAYNEGHI is used for falling edge of the clock."]
    #[inline(always)]
    #[must_use]
    pub fn rxdqsdelayhien0(&mut self) -> Rxdqsdelayhien0W<Dev0ddrSpec> {
        Rxdqsdelayhien0W::new(self, 31)
    }
}
#[doc = "Timing configuration bits for DDR operation of the MSPI module.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0ddrSpec;
impl crate::RegisterSpec for Dev0ddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0ddr::R`](R) reader structure"]
impl crate::Readable for Dev0ddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0ddr::W`](W) writer structure"]
impl crate::Writable for Dev0ddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0DDR to value 0"]
impl crate::Resettable for Dev0ddrSpec {
    const RESET_VALUE: u32 = 0;
}
