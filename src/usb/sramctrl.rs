#[doc = "Register `SRAMCTRL` reader"]
pub type R = crate::R<SramctrlSpec>;
#[doc = "Register `SRAMCTRL` writer"]
pub type W = crate::W<SramctrlSpec>;
#[doc = "Field `RET1N` reader - Retention mode 1 enable, active-LOW"]
pub type Ret1nR = crate::BitReader;
#[doc = "Field `RET1N` writer - Retention mode 1 enable, active-LOW"]
pub type Ret1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMA` reader - Extra margin adjustment"]
pub type EmaR = crate::FieldReader;
#[doc = "Field `EMA` writer - Extra margin adjustment"]
pub type EmaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMAS` reader - Extra margin adjustment sense amplifier pulse"]
pub type EmasR = crate::BitReader;
#[doc = "Field `EMAS` writer - Extra margin adjustment sense amplifier pulse"]
pub type EmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAW` reader - Extra margin adjustment for write operations"]
pub type EmawR = crate::FieldReader;
#[doc = "Field `EMAW` writer - Extra margin adjustment for write operations"]
pub type EmawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SRAM Adjustment for margin for this read assist scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rawlm {
    #[doc = "3: Increased margin adjustment, increased delay for enabling write assist."]
    Incdly = 3,
    #[doc = "2: Minimum boost level with increased delay for enabling write assist."]
    Mbincdly = 2,
    #[doc = "1: Increased margin adjustment with more negative boost."]
    Imnb = 1,
    #[doc = "0: Minimum margin adjustment with lowest negative boost level."]
    Mmnb = 0,
}
impl From<Rawlm> for u8 {
    #[inline(always)]
    fn from(variant: Rawlm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rawlm {
    type Ux = u8;
}
impl crate::IsEnum for Rawlm {}
#[doc = "Field `RAWLM` reader - SRAM Adjustment for margin for this read assist scheme"]
pub type RawlmR = crate::FieldReader<Rawlm>;
impl RawlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rawlm {
        match self.bits {
            3 => Rawlm::Incdly,
            2 => Rawlm::Mbincdly,
            1 => Rawlm::Imnb,
            0 => Rawlm::Mmnb,
            _ => unreachable!(),
        }
    }
    #[doc = "Increased margin adjustment, increased delay for enabling write assist."]
    #[inline(always)]
    pub fn is_incdly(&self) -> bool {
        *self == Rawlm::Incdly
    }
    #[doc = "Minimum boost level with increased delay for enabling write assist."]
    #[inline(always)]
    pub fn is_mbincdly(&self) -> bool {
        *self == Rawlm::Mbincdly
    }
    #[doc = "Increased margin adjustment with more negative boost."]
    #[inline(always)]
    pub fn is_imnb(&self) -> bool {
        *self == Rawlm::Imnb
    }
    #[doc = "Minimum margin adjustment with lowest negative boost level."]
    #[inline(always)]
    pub fn is_mmnb(&self) -> bool {
        *self == Rawlm::Mmnb
    }
}
#[doc = "Field `RAWLM` writer - SRAM Adjustment for margin for this read assist scheme"]
pub type RawlmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rawlm, crate::Safe>;
impl<'a, REG> RawlmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increased margin adjustment, increased delay for enabling write assist."]
    #[inline(always)]
    pub fn incdly(self) -> &'a mut crate::W<REG> {
        self.variant(Rawlm::Incdly)
    }
    #[doc = "Minimum boost level with increased delay for enabling write assist."]
    #[inline(always)]
    pub fn mbincdly(self) -> &'a mut crate::W<REG> {
        self.variant(Rawlm::Mbincdly)
    }
    #[doc = "Increased margin adjustment with more negative boost."]
    #[inline(always)]
    pub fn imnb(self) -> &'a mut crate::W<REG> {
        self.variant(Rawlm::Imnb)
    }
    #[doc = "Minimum margin adjustment with lowest negative boost level."]
    #[inline(always)]
    pub fn mmnb(self) -> &'a mut crate::W<REG> {
        self.variant(Rawlm::Mmnb)
    }
}
#[doc = "Field `RAWL` reader - SRAM Read assist enable"]
pub type RawlR = crate::BitReader;
#[doc = "Field `RAWL` writer - SRAM Read assist enable"]
pub type RawlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WABLM` reader - SRAM No margin adjustment"]
pub type WablmR = crate::FieldReader;
#[doc = "Field `WABLM` writer - SRAM No margin adjustment"]
pub type WablmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WABL` reader - SRAM write assist enable"]
pub type WablR = crate::BitReader;
#[doc = "Field `WABL` writer - SRAM write assist enable"]
pub type WablW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOV` reader - SRAM self-timed override"]
pub type StovR = crate::BitReader;
#[doc = "Field `STOV` writer - SRAM self-timed override"]
pub type StovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Retention mode 1 enable, active-LOW"]
    #[inline(always)]
    pub fn ret1n(&self) -> Ret1nR {
        Ret1nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Extra margin adjustment"]
    #[inline(always)]
    pub fn ema(&self) -> EmaR {
        EmaR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Extra margin adjustment sense amplifier pulse"]
    #[inline(always)]
    pub fn emas(&self) -> EmasR {
        EmasR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Extra margin adjustment for write operations"]
    #[inline(always)]
    pub fn emaw(&self) -> EmawR {
        EmawR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - SRAM Adjustment for margin for this read assist scheme"]
    #[inline(always)]
    pub fn rawlm(&self) -> RawlmR {
        RawlmR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - SRAM Read assist enable"]
    #[inline(always)]
    pub fn rawl(&self) -> RawlR {
        RawlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - SRAM No margin adjustment"]
    #[inline(always)]
    pub fn wablm(&self) -> WablmR {
        WablmR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - SRAM write assist enable"]
    #[inline(always)]
    pub fn wabl(&self) -> WablR {
        WablR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM self-timed override"]
    #[inline(always)]
    pub fn stov(&self) -> StovR {
        StovR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention mode 1 enable, active-LOW"]
    #[inline(always)]
    #[must_use]
    pub fn ret1n(&mut self) -> Ret1nW<SramctrlSpec> {
        Ret1nW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Extra margin adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn ema(&mut self) -> EmaW<SramctrlSpec> {
        EmaW::new(self, 1)
    }
    #[doc = "Bit 4 - Extra margin adjustment sense amplifier pulse"]
    #[inline(always)]
    #[must_use]
    pub fn emas(&mut self) -> EmasW<SramctrlSpec> {
        EmasW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Extra margin adjustment for write operations"]
    #[inline(always)]
    #[must_use]
    pub fn emaw(&mut self) -> EmawW<SramctrlSpec> {
        EmawW::new(self, 5)
    }
    #[doc = "Bits 7:8 - SRAM Adjustment for margin for this read assist scheme"]
    #[inline(always)]
    #[must_use]
    pub fn rawlm(&mut self) -> RawlmW<SramctrlSpec> {
        RawlmW::new(self, 7)
    }
    #[doc = "Bit 9 - SRAM Read assist enable"]
    #[inline(always)]
    #[must_use]
    pub fn rawl(&mut self) -> RawlW<SramctrlSpec> {
        RawlW::new(self, 9)
    }
    #[doc = "Bits 10:12 - SRAM No margin adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn wablm(&mut self) -> WablmW<SramctrlSpec> {
        WablmW::new(self, 10)
    }
    #[doc = "Bit 13 - SRAM write assist enable"]
    #[inline(always)]
    #[must_use]
    pub fn wabl(&mut self) -> WablW<SramctrlSpec> {
        WablW::new(self, 13)
    }
    #[doc = "Bit 14 - SRAM self-timed override"]
    #[inline(always)]
    #[must_use]
    pub fn stov(&mut self) -> StovW<SramctrlSpec> {
        StovW::new(self, 14)
    }
}
#[doc = "Provides optional SRAM tuning control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sramctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramctrlSpec;
impl crate::RegisterSpec for SramctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramctrl::R`](R) reader structure"]
impl crate::Readable for SramctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sramctrl::W`](W) writer structure"]
impl crate::Writable for SramctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMCTRL to value 0x7f"]
impl crate::Resettable for SramctrlSpec {
    const RESET_VALUE: u32 = 0x7f;
}
