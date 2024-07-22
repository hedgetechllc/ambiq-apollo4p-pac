#[doc = "Register `CHACHACONTROLREG` reader"]
pub type R = crate::R<ChachacontrolregSpec>;
#[doc = "Register `CHACHACONTROLREG` writer"]
pub type W = crate::W<ChachacontrolregSpec>;
#[doc = "Core:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chachaorsalsa {
    #[doc = "0: ChaCha mode,"]
    Chacha = 0,
    #[doc = "1: Salsa mode."]
    Salsa = 1,
}
impl From<Chachaorsalsa> for bool {
    #[inline(always)]
    fn from(variant: Chachaorsalsa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHACHAORSALSA` reader - Core:"]
pub type ChachaorsalsaR = crate::BitReader<Chachaorsalsa>;
impl ChachaorsalsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chachaorsalsa {
        match self.bits {
            false => Chachaorsalsa::Chacha,
            true => Chachaorsalsa::Salsa,
        }
    }
    #[doc = "ChaCha mode,"]
    #[inline(always)]
    pub fn is_chacha(&self) -> bool {
        *self == Chachaorsalsa::Chacha
    }
    #[doc = "Salsa mode."]
    #[inline(always)]
    pub fn is_salsa(&self) -> bool {
        *self == Chachaorsalsa::Salsa
    }
}
#[doc = "Field `CHACHAORSALSA` writer - Core:"]
pub type ChachaorsalsaW<'a, REG> = crate::BitWriter<'a, REG, Chachaorsalsa>;
impl<'a, REG> ChachaorsalsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ChaCha mode,"]
    #[inline(always)]
    pub fn chacha(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaorsalsa::Chacha)
    }
    #[doc = "Salsa mode."]
    #[inline(always)]
    pub fn salsa(self) -> &'a mut crate::W<REG> {
        self.variant(Chachaorsalsa::Salsa)
    }
}
#[doc = "Start init for new Message:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initfromhost {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable."]
    Enable = 1,
}
impl From<Initfromhost> for bool {
    #[inline(always)]
    fn from(variant: Initfromhost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITFROMHOST` reader - Start init for new Message:"]
pub type InitfromhostR = crate::BitReader<Initfromhost>;
impl InitfromhostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initfromhost {
        match self.bits {
            false => Initfromhost::Disable,
            true => Initfromhost::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Initfromhost::Disable
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Initfromhost::Enable
    }
}
#[doc = "Field `INITFROMHOST` writer - Start init for new Message:"]
pub type InitfromhostW<'a, REG> = crate::BitWriter<'a, REG, Initfromhost>;
impl<'a, REG> InitfromhostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Initfromhost::Disable)
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Initfromhost::Enable)
    }
}
#[doc = "Only if ChaCha core:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calckeyforpoly1305 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable."]
    Enable = 1,
}
impl From<Calckeyforpoly1305> for bool {
    #[inline(always)]
    fn from(variant: Calckeyforpoly1305) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCKEYFORPOLY1305` reader - Only if ChaCha core:"]
pub type Calckeyforpoly1305R = crate::BitReader<Calckeyforpoly1305>;
impl Calckeyforpoly1305R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calckeyforpoly1305 {
        match self.bits {
            false => Calckeyforpoly1305::Disable,
            true => Calckeyforpoly1305::Enable,
        }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Calckeyforpoly1305::Disable
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Calckeyforpoly1305::Enable
    }
}
#[doc = "Field `CALCKEYFORPOLY1305` writer - Only if ChaCha core:"]
pub type Calckeyforpoly1305W<'a, REG> = crate::BitWriter<'a, REG, Calckeyforpoly1305>;
impl<'a, REG> Calckeyforpoly1305W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Calckeyforpoly1305::Disable)
    }
    #[doc = "enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Calckeyforpoly1305::Enable)
    }
}
#[doc = "For All Core:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keylen {
    #[doc = "0: 256 bit."]
    _256Bit = 0,
    #[doc = "1: 128 bit."]
    _128Bit = 1,
}
impl From<Keylen> for bool {
    #[inline(always)]
    fn from(variant: Keylen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYLEN` reader - For All Core:"]
pub type KeylenR = crate::BitReader<Keylen>;
impl KeylenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keylen {
        match self.bits {
            false => Keylen::_256Bit,
            true => Keylen::_128Bit,
        }
    }
    #[doc = "256 bit."]
    #[inline(always)]
    pub fn is_256_bit(&self) -> bool {
        *self == Keylen::_256Bit
    }
    #[doc = "128 bit."]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        *self == Keylen::_128Bit
    }
}
#[doc = "Field `KEYLEN` writer - For All Core:"]
pub type KeylenW<'a, REG> = crate::BitWriter<'a, REG, Keylen>;
impl<'a, REG> KeylenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "256 bit."]
    #[inline(always)]
    pub fn _256_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Keylen::_256Bit)
    }
    #[doc = "128 bit."]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Keylen::_128Bit)
    }
}
#[doc = "The core of ChaCha is a hash function which based on rotation operations. The hash function consist in application of 20 rounds (default value). In additional, ChaCha have two variants (they work exactly as the original algorithm): ChaCha20_8 and ChaCha20_12 (using 8 and 12 rounds).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numofrounds {
    #[doc = "0: 20 rounds"]
    _20Rounds = 0,
    #[doc = "1: 12 rounds"]
    _12Rounds = 1,
    #[doc = "2: 8 rounds"]
    _8Rounds = 2,
    #[doc = "3: Not applicable"]
    NA = 3,
}
impl From<Numofrounds> for u8 {
    #[inline(always)]
    fn from(variant: Numofrounds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numofrounds {
    type Ux = u8;
}
impl crate::IsEnum for Numofrounds {}
#[doc = "Field `NUMOFROUNDS` reader - The core of ChaCha is a hash function which based on rotation operations. The hash function consist in application of 20 rounds (default value). In additional, ChaCha have two variants (they work exactly as the original algorithm): ChaCha20_8 and ChaCha20_12 (using 8 and 12 rounds)."]
pub type NumofroundsR = crate::FieldReader<Numofrounds>;
impl NumofroundsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Numofrounds {
        match self.bits {
            0 => Numofrounds::_20Rounds,
            1 => Numofrounds::_12Rounds,
            2 => Numofrounds::_8Rounds,
            3 => Numofrounds::NA,
            _ => unreachable!(),
        }
    }
    #[doc = "20 rounds"]
    #[inline(always)]
    pub fn is_20_rounds(&self) -> bool {
        *self == Numofrounds::_20Rounds
    }
    #[doc = "12 rounds"]
    #[inline(always)]
    pub fn is_12_rounds(&self) -> bool {
        *self == Numofrounds::_12Rounds
    }
    #[doc = "8 rounds"]
    #[inline(always)]
    pub fn is_8_rounds(&self) -> bool {
        *self == Numofrounds::_8Rounds
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn is_n_a(&self) -> bool {
        *self == Numofrounds::NA
    }
}
#[doc = "Field `NUMOFROUNDS` writer - The core of ChaCha is a hash function which based on rotation operations. The hash function consist in application of 20 rounds (default value). In additional, ChaCha have two variants (they work exactly as the original algorithm): ChaCha20_8 and ChaCha20_12 (using 8 and 12 rounds)."]
pub type NumofroundsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Numofrounds, crate::Safe>;
impl<'a, REG> NumofroundsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 rounds"]
    #[inline(always)]
    pub fn _20_rounds(self) -> &'a mut crate::W<REG> {
        self.variant(Numofrounds::_20Rounds)
    }
    #[doc = "12 rounds"]
    #[inline(always)]
    pub fn _12_rounds(self) -> &'a mut crate::W<REG> {
        self.variant(Numofrounds::_12Rounds)
    }
    #[doc = "8 rounds"]
    #[inline(always)]
    pub fn _8_rounds(self) -> &'a mut crate::W<REG> {
        self.variant(Numofrounds::_8Rounds)
    }
    #[doc = "Not applicable"]
    #[inline(always)]
    pub fn n_a(self) -> &'a mut crate::W<REG> {
        self.variant(Numofrounds::NA)
    }
}
#[doc = "Field `RESETBLOCKCNT` reader - For new message"]
pub type ResetblockcntR = crate::BitReader;
#[doc = "Field `RESETBLOCKCNT` writer - For new message"]
pub type ResetblockcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEIV96BIT` reader - If use 96bit IV"]
pub type Useiv96bitR = crate::BitReader;
#[doc = "Field `USEIV96BIT` writer - If use 96bit IV"]
pub type Useiv96bitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core:"]
    #[inline(always)]
    pub fn chachaorsalsa(&self) -> ChachaorsalsaR {
        ChachaorsalsaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start init for new Message:"]
    #[inline(always)]
    pub fn initfromhost(&self) -> InitfromhostR {
        InitfromhostR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only if ChaCha core:"]
    #[inline(always)]
    pub fn calckeyforpoly1305(&self) -> Calckeyforpoly1305R {
        Calckeyforpoly1305R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For All Core:"]
    #[inline(always)]
    pub fn keylen(&self) -> KeylenR {
        KeylenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - The core of ChaCha is a hash function which based on rotation operations. The hash function consist in application of 20 rounds (default value). In additional, ChaCha have two variants (they work exactly as the original algorithm): ChaCha20_8 and ChaCha20_12 (using 8 and 12 rounds)."]
    #[inline(always)]
    pub fn numofrounds(&self) -> NumofroundsR {
        NumofroundsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 9 - For new message"]
    #[inline(always)]
    pub fn resetblockcnt(&self) -> ResetblockcntR {
        ResetblockcntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If use 96bit IV"]
    #[inline(always)]
    pub fn useiv96bit(&self) -> Useiv96bitR {
        Useiv96bitR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core:"]
    #[inline(always)]
    #[must_use]
    pub fn chachaorsalsa(&mut self) -> ChachaorsalsaW<ChachacontrolregSpec> {
        ChachaorsalsaW::new(self, 0)
    }
    #[doc = "Bit 1 - Start init for new Message:"]
    #[inline(always)]
    #[must_use]
    pub fn initfromhost(&mut self) -> InitfromhostW<ChachacontrolregSpec> {
        InitfromhostW::new(self, 1)
    }
    #[doc = "Bit 2 - Only if ChaCha core:"]
    #[inline(always)]
    #[must_use]
    pub fn calckeyforpoly1305(&mut self) -> Calckeyforpoly1305W<ChachacontrolregSpec> {
        Calckeyforpoly1305W::new(self, 2)
    }
    #[doc = "Bit 3 - For All Core:"]
    #[inline(always)]
    #[must_use]
    pub fn keylen(&mut self) -> KeylenW<ChachacontrolregSpec> {
        KeylenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - The core of ChaCha is a hash function which based on rotation operations. The hash function consist in application of 20 rounds (default value). In additional, ChaCha have two variants (they work exactly as the original algorithm): ChaCha20_8 and ChaCha20_12 (using 8 and 12 rounds)."]
    #[inline(always)]
    #[must_use]
    pub fn numofrounds(&mut self) -> NumofroundsW<ChachacontrolregSpec> {
        NumofroundsW::new(self, 4)
    }
    #[doc = "Bit 9 - For new message"]
    #[inline(always)]
    #[must_use]
    pub fn resetblockcnt(&mut self) -> ResetblockcntW<ChachacontrolregSpec> {
        ResetblockcntW::new(self, 9)
    }
    #[doc = "Bit 10 - If use 96bit IV"]
    #[inline(always)]
    #[must_use]
    pub fn useiv96bit(&mut self) -> Useiv96bitW<ChachacontrolregSpec> {
        Useiv96bitW::new(self, 10)
    }
}
#[doc = "CHACHA general configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachacontrolreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachacontrolreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachacontrolregSpec;
impl crate::RegisterSpec for ChachacontrolregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachacontrolreg::R`](R) reader structure"]
impl crate::Readable for ChachacontrolregSpec {}
#[doc = "`write(|w| ..)` method takes [`chachacontrolreg::W`](W) writer structure"]
impl crate::Writable for ChachacontrolregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHACONTROLREG to value 0"]
impl crate::Resettable for ChachacontrolregSpec {
    const RESET_VALUE: u32 = 0;
}
