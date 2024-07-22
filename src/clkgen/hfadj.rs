#[doc = "Register `HFADJ` reader"]
pub type R = crate::R<HfadjSpec>;
#[doc = "Register `HFADJ` writer"]
pub type W = crate::W<HfadjSpec>;
#[doc = "HFRC adjustment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfadjen {
    #[doc = "0: Disable the HFRC adjustment"]
    Dis = 0,
    #[doc = "1: Enable the HFRC adjustment"]
    En = 1,
}
impl From<Hfadjen> for bool {
    #[inline(always)]
    fn from(variant: Hfadjen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFADJEN` reader - HFRC adjustment control"]
pub type HfadjenR = crate::BitReader<Hfadjen>;
impl HfadjenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfadjen {
        match self.bits {
            false => Hfadjen::Dis,
            true => Hfadjen::En,
        }
    }
    #[doc = "Disable the HFRC adjustment"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hfadjen::Dis
    }
    #[doc = "Enable the HFRC adjustment"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Hfadjen::En
    }
}
#[doc = "Field `HFADJEN` writer - HFRC adjustment control"]
pub type HfadjenW<'a, REG> = crate::BitWriter<'a, REG, Hfadjen>;
impl<'a, REG> HfadjenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the HFRC adjustment"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjen::Dis)
    }
    #[doc = "Enable the HFRC adjustment"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjen::En)
    }
}
#[doc = "Repeat period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfadjck {
    #[doc = "0: Autoadjust repeat period = 4 seconds"]
    _4sec = 0,
    #[doc = "1: Autoadjust repeat period = 16 seconds"]
    _16sec = 1,
    #[doc = "2: Autoadjust repeat period = 32 seconds"]
    _32sec = 2,
    #[doc = "3: Autoadjust repeat period = 64 seconds"]
    _64sec = 3,
    #[doc = "4: Autoadjust repeat period = 128 seconds"]
    _128sec = 4,
    #[doc = "5: Autoadjust repeat period = 256 seconds"]
    _256sec = 5,
    #[doc = "6: Autoadjust repeat period = 512 seconds"]
    _512sec = 6,
    #[doc = "7: Autoadjust repeat period = 1024 seconds"]
    _1024sec = 7,
}
impl From<Hfadjck> for u8 {
    #[inline(always)]
    fn from(variant: Hfadjck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfadjck {
    type Ux = u8;
}
impl crate::IsEnum for Hfadjck {}
#[doc = "Field `HFADJCK` reader - Repeat period for HFRC adjustment"]
pub type HfadjckR = crate::FieldReader<Hfadjck>;
impl HfadjckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfadjck {
        match self.bits {
            0 => Hfadjck::_4sec,
            1 => Hfadjck::_16sec,
            2 => Hfadjck::_32sec,
            3 => Hfadjck::_64sec,
            4 => Hfadjck::_128sec,
            5 => Hfadjck::_256sec,
            6 => Hfadjck::_512sec,
            7 => Hfadjck::_1024sec,
            _ => unreachable!(),
        }
    }
    #[doc = "Autoadjust repeat period = 4 seconds"]
    #[inline(always)]
    pub fn is_4sec(&self) -> bool {
        *self == Hfadjck::_4sec
    }
    #[doc = "Autoadjust repeat period = 16 seconds"]
    #[inline(always)]
    pub fn is_16sec(&self) -> bool {
        *self == Hfadjck::_16sec
    }
    #[doc = "Autoadjust repeat period = 32 seconds"]
    #[inline(always)]
    pub fn is_32sec(&self) -> bool {
        *self == Hfadjck::_32sec
    }
    #[doc = "Autoadjust repeat period = 64 seconds"]
    #[inline(always)]
    pub fn is_64sec(&self) -> bool {
        *self == Hfadjck::_64sec
    }
    #[doc = "Autoadjust repeat period = 128 seconds"]
    #[inline(always)]
    pub fn is_128sec(&self) -> bool {
        *self == Hfadjck::_128sec
    }
    #[doc = "Autoadjust repeat period = 256 seconds"]
    #[inline(always)]
    pub fn is_256sec(&self) -> bool {
        *self == Hfadjck::_256sec
    }
    #[doc = "Autoadjust repeat period = 512 seconds"]
    #[inline(always)]
    pub fn is_512sec(&self) -> bool {
        *self == Hfadjck::_512sec
    }
    #[doc = "Autoadjust repeat period = 1024 seconds"]
    #[inline(always)]
    pub fn is_1024sec(&self) -> bool {
        *self == Hfadjck::_1024sec
    }
}
#[doc = "Field `HFADJCK` writer - Repeat period for HFRC adjustment"]
pub type HfadjckW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hfadjck, crate::Safe>;
impl<'a, REG> HfadjckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Autoadjust repeat period = 4 seconds"]
    #[inline(always)]
    pub fn _4sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_4sec)
    }
    #[doc = "Autoadjust repeat period = 16 seconds"]
    #[inline(always)]
    pub fn _16sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_16sec)
    }
    #[doc = "Autoadjust repeat period = 32 seconds"]
    #[inline(always)]
    pub fn _32sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_32sec)
    }
    #[doc = "Autoadjust repeat period = 64 seconds"]
    #[inline(always)]
    pub fn _64sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_64sec)
    }
    #[doc = "Autoadjust repeat period = 128 seconds"]
    #[inline(always)]
    pub fn _128sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_128sec)
    }
    #[doc = "Autoadjust repeat period = 256 seconds"]
    #[inline(always)]
    pub fn _256sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_256sec)
    }
    #[doc = "Autoadjust repeat period = 512 seconds"]
    #[inline(always)]
    pub fn _512sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_512sec)
    }
    #[doc = "Autoadjust repeat period = 1024 seconds"]
    #[inline(always)]
    pub fn _1024sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjck::_1024sec)
    }
}
#[doc = "Field `HFXTADJ` reader - Target HFRC adjustment value."]
pub type HfxtadjR = crate::FieldReader<u16>;
#[doc = "Field `HFXTADJ` writer - Target HFRC adjustment value."]
pub type HfxtadjW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "XT warmup period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfwarmup {
    #[doc = "0: Autoadjust XT warmup period = 1-2 seconds"]
    _1sec = 0,
    #[doc = "1: Autoadjust XT warmup period = 2-4 seconds"]
    _2sec = 1,
}
impl From<Hfwarmup> for bool {
    #[inline(always)]
    fn from(variant: Hfwarmup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFWARMUP` reader - XT warmup period for HFRC adjustment"]
pub type HfwarmupR = crate::BitReader<Hfwarmup>;
impl HfwarmupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfwarmup {
        match self.bits {
            false => Hfwarmup::_1sec,
            true => Hfwarmup::_2sec,
        }
    }
    #[doc = "Autoadjust XT warmup period = 1-2 seconds"]
    #[inline(always)]
    pub fn is_1sec(&self) -> bool {
        *self == Hfwarmup::_1sec
    }
    #[doc = "Autoadjust XT warmup period = 2-4 seconds"]
    #[inline(always)]
    pub fn is_2sec(&self) -> bool {
        *self == Hfwarmup::_2sec
    }
}
#[doc = "Field `HFWARMUP` writer - XT warmup period for HFRC adjustment"]
pub type HfwarmupW<'a, REG> = crate::BitWriter<'a, REG, Hfwarmup>;
impl<'a, REG> HfwarmupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Autoadjust XT warmup period = 1-2 seconds"]
    #[inline(always)]
    pub fn _1sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfwarmup::_1sec)
    }
    #[doc = "Autoadjust XT warmup period = 2-4 seconds"]
    #[inline(always)]
    pub fn _2sec(self) -> &'a mut crate::W<REG> {
        self.variant(Hfwarmup::_2sec)
    }
}
#[doc = "Gain control for HFRC adjustment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfadjgain {
    #[doc = "0: HF Adjust with Gain of 1"]
    GainOf1 = 0,
    #[doc = "1: HF Adjust with Gain of 0.5"]
    GainOf1In2 = 1,
    #[doc = "2: HF Adjust with Gain of 0.25"]
    GainOf1In4 = 2,
    #[doc = "3: HF Adjust with Gain of 0.125"]
    GainOf1In8 = 3,
    #[doc = "4: HF Adjust with Gain of 0.0625"]
    GainOf1In16 = 4,
    #[doc = "5: HF Adjust with Gain of 0.03125"]
    GainOf1In32 = 5,
}
impl From<Hfadjgain> for u8 {
    #[inline(always)]
    fn from(variant: Hfadjgain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfadjgain {
    type Ux = u8;
}
impl crate::IsEnum for Hfadjgain {}
#[doc = "Field `HFADJGAIN` reader - Gain control for HFRC adjustment"]
pub type HfadjgainR = crate::FieldReader<Hfadjgain>;
impl HfadjgainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfadjgain> {
        match self.bits {
            0 => Some(Hfadjgain::GainOf1),
            1 => Some(Hfadjgain::GainOf1In2),
            2 => Some(Hfadjgain::GainOf1In4),
            3 => Some(Hfadjgain::GainOf1In8),
            4 => Some(Hfadjgain::GainOf1In16),
            5 => Some(Hfadjgain::GainOf1In32),
            _ => None,
        }
    }
    #[doc = "HF Adjust with Gain of 1"]
    #[inline(always)]
    pub fn is_gain_of_1(&self) -> bool {
        *self == Hfadjgain::GainOf1
    }
    #[doc = "HF Adjust with Gain of 0.5"]
    #[inline(always)]
    pub fn is_gain_of_1_in_2(&self) -> bool {
        *self == Hfadjgain::GainOf1In2
    }
    #[doc = "HF Adjust with Gain of 0.25"]
    #[inline(always)]
    pub fn is_gain_of_1_in_4(&self) -> bool {
        *self == Hfadjgain::GainOf1In4
    }
    #[doc = "HF Adjust with Gain of 0.125"]
    #[inline(always)]
    pub fn is_gain_of_1_in_8(&self) -> bool {
        *self == Hfadjgain::GainOf1In8
    }
    #[doc = "HF Adjust with Gain of 0.0625"]
    #[inline(always)]
    pub fn is_gain_of_1_in_16(&self) -> bool {
        *self == Hfadjgain::GainOf1In16
    }
    #[doc = "HF Adjust with Gain of 0.03125"]
    #[inline(always)]
    pub fn is_gain_of_1_in_32(&self) -> bool {
        *self == Hfadjgain::GainOf1In32
    }
}
#[doc = "Field `HFADJGAIN` writer - Gain control for HFRC adjustment"]
pub type HfadjgainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hfadjgain>;
impl<'a, REG> HfadjgainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HF Adjust with Gain of 1"]
    #[inline(always)]
    pub fn gain_of_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1)
    }
    #[doc = "HF Adjust with Gain of 0.5"]
    #[inline(always)]
    pub fn gain_of_1_in_2(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1In2)
    }
    #[doc = "HF Adjust with Gain of 0.25"]
    #[inline(always)]
    pub fn gain_of_1_in_4(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1In4)
    }
    #[doc = "HF Adjust with Gain of 0.125"]
    #[inline(always)]
    pub fn gain_of_1_in_8(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1In8)
    }
    #[doc = "HF Adjust with Gain of 0.0625"]
    #[inline(always)]
    pub fn gain_of_1_in_16(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1In16)
    }
    #[doc = "HF Adjust with Gain of 0.03125"]
    #[inline(always)]
    pub fn gain_of_1_in_32(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjgain::GainOf1In32)
    }
}
#[doc = "Maximum delta for HF Adjustments. 0=Disabled, 1-31=maximum delta step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hfadjmaxdelta {
    #[doc = "0: Maximum Delta function is disabled"]
    Disabled = 0,
    #[doc = "1: Maximum Delta function is enabled"]
    Enabled = 1,
}
impl From<Hfadjmaxdelta> for u8 {
    #[inline(always)]
    fn from(variant: Hfadjmaxdelta) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hfadjmaxdelta {
    type Ux = u8;
}
impl crate::IsEnum for Hfadjmaxdelta {}
#[doc = "Field `HFADJMAXDELTA` reader - Maximum delta for HF Adjustments. 0=Disabled, 1-31=maximum delta step"]
pub type HfadjmaxdeltaR = crate::FieldReader<Hfadjmaxdelta>;
impl HfadjmaxdeltaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hfadjmaxdelta> {
        match self.bits {
            0 => Some(Hfadjmaxdelta::Disabled),
            1 => Some(Hfadjmaxdelta::Enabled),
            _ => None,
        }
    }
    #[doc = "Maximum Delta function is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hfadjmaxdelta::Disabled
    }
    #[doc = "Maximum Delta function is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hfadjmaxdelta::Enabled
    }
}
#[doc = "Field `HFADJMAXDELTA` writer - Maximum delta for HF Adjustments. 0=Disabled, 1-31=maximum delta step"]
pub type HfadjmaxdeltaW<'a, REG> = crate::FieldWriter<'a, REG, 5, Hfadjmaxdelta>;
impl<'a, REG> HfadjmaxdeltaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum Delta function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjmaxdelta::Disabled)
    }
    #[doc = "Maximum Delta function is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hfadjmaxdelta::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    pub fn hfadjen(&self) -> HfadjenR {
        HfadjenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjck(&self) -> HfadjckR {
        HfadjckR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline(always)]
    pub fn hfxtadj(&self) -> HfxtadjR {
        HfxtadjR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfwarmup(&self) -> HfwarmupR {
        HfwarmupR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjgain(&self) -> HfadjgainR {
        HfadjgainR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - Maximum delta for HF Adjustments. 0=Disabled, 1-31=maximum delta step"]
    #[inline(always)]
    pub fn hfadjmaxdelta(&self) -> HfadjmaxdeltaR {
        HfadjmaxdeltaR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    #[must_use]
    pub fn hfadjen(&mut self) -> HfadjenW<HfadjSpec> {
        HfadjenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn hfadjck(&mut self) -> HfadjckW<HfadjSpec> {
        HfadjckW::new(self, 1)
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline(always)]
    #[must_use]
    pub fn hfxtadj(&mut self) -> HfxtadjW<HfadjSpec> {
        HfxtadjW::new(self, 8)
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn hfwarmup(&mut self) -> HfwarmupW<HfadjSpec> {
        HfwarmupW::new(self, 20)
    }
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn hfadjgain(&mut self) -> HfadjgainW<HfadjSpec> {
        HfadjgainW::new(self, 21)
    }
    #[doc = "Bits 24:28 - Maximum delta for HF Adjustments. 0=Disabled, 1-31=maximum delta step"]
    #[inline(always)]
    #[must_use]
    pub fn hfadjmaxdelta(&mut self) -> HfadjmaxdeltaW<HfadjSpec> {
        HfadjmaxdeltaW::new(self, 24)
    }
}
#[doc = "This register controls the HFRC adjustment. The HFRC clock can change with temperature and process corners, and this register controls the HFRC adjustment logic which reduces the fluctuations to the clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfadj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfadj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfadjSpec;
impl crate::RegisterSpec for HfadjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfadj::R`](R) reader structure"]
impl crate::Readable for HfadjSpec {}
#[doc = "`write(|w| ..)` method takes [`hfadj::W`](W) writer structure"]
impl crate::Writable for HfadjSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFADJ to value 0x0025_b800"]
impl crate::Resettable for HfadjSpec {
    const RESET_VALUE: u32 = 0x0025_b800;
}
