#[doc = "Register `AUDSSPWREN` reader"]
pub type R = crate::R<AudsspwrenSpec>;
#[doc = "Register `AUDSSPWREN` writer"]
pub type W = crate::W<AudsspwrenSpec>;
#[doc = "Power up Audio Record\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenaudrec {
    #[doc = "1: Power up AUDREC"]
    En = 1,
    #[doc = "0: Power down AUDREC"]
    Dis = 0,
}
impl From<Pwrenaudrec> for bool {
    #[inline(always)]
    fn from(variant: Pwrenaudrec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENAUDREC` reader - Power up Audio Record"]
pub type PwrenaudrecR = crate::BitReader<Pwrenaudrec>;
impl PwrenaudrecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenaudrec {
        match self.bits {
            true => Pwrenaudrec::En,
            false => Pwrenaudrec::Dis,
        }
    }
    #[doc = "Power up AUDREC"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenaudrec::En
    }
    #[doc = "Power down AUDREC"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenaudrec::Dis
    }
}
#[doc = "Field `PWRENAUDREC` writer - Power up Audio Record"]
pub type PwrenaudrecW<'a, REG> = crate::BitWriter<'a, REG, Pwrenaudrec>;
impl<'a, REG> PwrenaudrecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up AUDREC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudrec::En)
    }
    #[doc = "Power down AUDREC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudrec::Dis)
    }
}
#[doc = "Power up Audio Playback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenaudpb {
    #[doc = "1: Power up AUDPB"]
    En = 1,
    #[doc = "0: Power down AUDPB"]
    Dis = 0,
}
impl From<Pwrenaudpb> for bool {
    #[inline(always)]
    fn from(variant: Pwrenaudpb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENAUDPB` reader - Power up Audio Playback"]
pub type PwrenaudpbR = crate::BitReader<Pwrenaudpb>;
impl PwrenaudpbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenaudpb {
        match self.bits {
            true => Pwrenaudpb::En,
            false => Pwrenaudpb::Dis,
        }
    }
    #[doc = "Power up AUDPB"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenaudpb::En
    }
    #[doc = "Power down AUDPB"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenaudpb::Dis
    }
}
#[doc = "Field `PWRENAUDPB` writer - Power up Audio Playback"]
pub type PwrenaudpbW<'a, REG> = crate::BitWriter<'a, REG, Pwrenaudpb>;
impl<'a, REG> PwrenaudpbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up AUDPB"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudpb::En)
    }
    #[doc = "Power down AUDPB"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudpb::Dis)
    }
}
#[doc = "Power up audio subsystem PDM0 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenpdm0 {
    #[doc = "1: Power up PDM0"]
    En = 1,
    #[doc = "0: Power down PDM0"]
    Dis = 0,
}
impl From<Pwrenpdm0> for bool {
    #[inline(always)]
    fn from(variant: Pwrenpdm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENPDM0` reader - Power up audio subsystem PDM0 domain"]
pub type Pwrenpdm0R = crate::BitReader<Pwrenpdm0>;
impl Pwrenpdm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenpdm0 {
        match self.bits {
            true => Pwrenpdm0::En,
            false => Pwrenpdm0::Dis,
        }
    }
    #[doc = "Power up PDM0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenpdm0::En
    }
    #[doc = "Power down PDM0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenpdm0::Dis
    }
}
#[doc = "Field `PWRENPDM0` writer - Power up audio subsystem PDM0 domain"]
pub type Pwrenpdm0W<'a, REG> = crate::BitWriter<'a, REG, Pwrenpdm0>;
impl<'a, REG> Pwrenpdm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up PDM0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm0::En)
    }
    #[doc = "Power down PDM0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm0::Dis)
    }
}
#[doc = "Power up audio subsystem PDM1 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenpdm1 {
    #[doc = "1: Power up PDM1"]
    En = 1,
    #[doc = "0: Power down PDM1"]
    Dis = 0,
}
impl From<Pwrenpdm1> for bool {
    #[inline(always)]
    fn from(variant: Pwrenpdm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENPDM1` reader - Power up audio subsystem PDM1 domain"]
pub type Pwrenpdm1R = crate::BitReader<Pwrenpdm1>;
impl Pwrenpdm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenpdm1 {
        match self.bits {
            true => Pwrenpdm1::En,
            false => Pwrenpdm1::Dis,
        }
    }
    #[doc = "Power up PDM1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenpdm1::En
    }
    #[doc = "Power down PDM1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenpdm1::Dis
    }
}
#[doc = "Field `PWRENPDM1` writer - Power up audio subsystem PDM1 domain"]
pub type Pwrenpdm1W<'a, REG> = crate::BitWriter<'a, REG, Pwrenpdm1>;
impl<'a, REG> Pwrenpdm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up PDM1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm1::En)
    }
    #[doc = "Power down PDM1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm1::Dis)
    }
}
#[doc = "Power up audio subsystem PDM2 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenpdm2 {
    #[doc = "1: Power up PDM2"]
    En = 1,
    #[doc = "0: Power down PDM2"]
    Dis = 0,
}
impl From<Pwrenpdm2> for bool {
    #[inline(always)]
    fn from(variant: Pwrenpdm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENPDM2` reader - Power up audio subsystem PDM2 domain"]
pub type Pwrenpdm2R = crate::BitReader<Pwrenpdm2>;
impl Pwrenpdm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenpdm2 {
        match self.bits {
            true => Pwrenpdm2::En,
            false => Pwrenpdm2::Dis,
        }
    }
    #[doc = "Power up PDM2"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenpdm2::En
    }
    #[doc = "Power down PDM2"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenpdm2::Dis
    }
}
#[doc = "Field `PWRENPDM2` writer - Power up audio subsystem PDM2 domain"]
pub type Pwrenpdm2W<'a, REG> = crate::BitWriter<'a, REG, Pwrenpdm2>;
impl<'a, REG> Pwrenpdm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up PDM2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm2::En)
    }
    #[doc = "Power down PDM2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm2::Dis)
    }
}
#[doc = "Power up audio subsystem PDM3 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenpdm3 {
    #[doc = "1: Power up PDM3"]
    En = 1,
    #[doc = "0: Power down PDM3"]
    Dis = 0,
}
impl From<Pwrenpdm3> for bool {
    #[inline(always)]
    fn from(variant: Pwrenpdm3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENPDM3` reader - Power up audio subsystem PDM3 domain"]
pub type Pwrenpdm3R = crate::BitReader<Pwrenpdm3>;
impl Pwrenpdm3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenpdm3 {
        match self.bits {
            true => Pwrenpdm3::En,
            false => Pwrenpdm3::Dis,
        }
    }
    #[doc = "Power up PDM3"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenpdm3::En
    }
    #[doc = "Power down PDM3"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenpdm3::Dis
    }
}
#[doc = "Field `PWRENPDM3` writer - Power up audio subsystem PDM3 domain"]
pub type Pwrenpdm3W<'a, REG> = crate::BitWriter<'a, REG, Pwrenpdm3>;
impl<'a, REG> Pwrenpdm3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up PDM3"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm3::En)
    }
    #[doc = "Power down PDM3"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenpdm3::Dis)
    }
}
#[doc = "Power up audio subsystem I2S0 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreni2s0 {
    #[doc = "1: Power up I2S0"]
    En = 1,
    #[doc = "0: Power down I2S0"]
    Dis = 0,
}
impl From<Pwreni2s0> for bool {
    #[inline(always)]
    fn from(variant: Pwreni2s0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENI2S0` reader - Power up audio subsystem I2S0 domain"]
pub type Pwreni2s0R = crate::BitReader<Pwreni2s0>;
impl Pwreni2s0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreni2s0 {
        match self.bits {
            true => Pwreni2s0::En,
            false => Pwreni2s0::Dis,
        }
    }
    #[doc = "Power up I2S0"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreni2s0::En
    }
    #[doc = "Power down I2S0"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreni2s0::Dis
    }
}
#[doc = "Field `PWRENI2S0` writer - Power up audio subsystem I2S0 domain"]
pub type Pwreni2s0W<'a, REG> = crate::BitWriter<'a, REG, Pwreni2s0>;
impl<'a, REG> Pwreni2s0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up I2S0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreni2s0::En)
    }
    #[doc = "Power down I2S0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreni2s0::Dis)
    }
}
#[doc = "Power up audio subsystem I2S1 domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwreni2s1 {
    #[doc = "1: Power up I2S1"]
    En = 1,
    #[doc = "0: Power down I2S1"]
    Dis = 0,
}
impl From<Pwreni2s1> for bool {
    #[inline(always)]
    fn from(variant: Pwreni2s1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENI2S1` reader - Power up audio subsystem I2S1 domain"]
pub type Pwreni2s1R = crate::BitReader<Pwreni2s1>;
impl Pwreni2s1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwreni2s1 {
        match self.bits {
            true => Pwreni2s1::En,
            false => Pwreni2s1::Dis,
        }
    }
    #[doc = "Power up I2S1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwreni2s1::En
    }
    #[doc = "Power down I2S1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwreni2s1::Dis
    }
}
#[doc = "Field `PWRENI2S1` writer - Power up audio subsystem I2S1 domain"]
pub type Pwreni2s1W<'a, REG> = crate::BitWriter<'a, REG, Pwreni2s1>;
impl<'a, REG> Pwreni2s1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up I2S1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreni2s1::En)
    }
    #[doc = "Power down I2S1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwreni2s1::Dis)
    }
}
#[doc = "Power up audio subsystem ADC domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrenaudadc {
    #[doc = "1: Power up AUDADC"]
    En = 1,
    #[doc = "0: Power down AUDADC"]
    Dis = 0,
}
impl From<Pwrenaudadc> for bool {
    #[inline(always)]
    fn from(variant: Pwrenaudadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENAUDADC` reader - Power up audio subsystem ADC domain"]
pub type PwrenaudadcR = crate::BitReader<Pwrenaudadc>;
impl PwrenaudadcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrenaudadc {
        match self.bits {
            true => Pwrenaudadc::En,
            false => Pwrenaudadc::Dis,
        }
    }
    #[doc = "Power up AUDADC"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrenaudadc::En
    }
    #[doc = "Power down AUDADC"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrenaudadc::Dis
    }
}
#[doc = "Field `PWRENAUDADC` writer - Power up audio subsystem ADC domain"]
pub type PwrenaudadcW<'a, REG> = crate::BitWriter<'a, REG, Pwrenaudadc>;
impl<'a, REG> PwrenaudadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power up AUDADC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudadc::En)
    }
    #[doc = "Power down AUDADC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrenaudadc::Dis)
    }
}
#[doc = "Enable one or more DSP subsystems\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrendspa {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<Pwrendspa> for bool {
    #[inline(always)]
    fn from(variant: Pwrendspa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRENDSPA` reader - Enable one or more DSP subsystems"]
pub type PwrendspaR = crate::BitReader<Pwrendspa>;
impl PwrendspaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrendspa {
        match self.bits {
            true => Pwrendspa::En,
            false => Pwrendspa::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pwrendspa::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pwrendspa::Dis
    }
}
#[doc = "Field `PWRENDSPA` writer - Enable one or more DSP subsystems"]
pub type PwrendspaW<'a, REG> = crate::BitWriter<'a, REG, Pwrendspa>;
impl<'a, REG> PwrendspaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendspa::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrendspa::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Power up Audio Record"]
    #[inline(always)]
    pub fn pwrenaudrec(&self) -> PwrenaudrecR {
        PwrenaudrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power up Audio Playback"]
    #[inline(always)]
    pub fn pwrenaudpb(&self) -> PwrenaudpbR {
        PwrenaudpbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power up audio subsystem PDM0 domain"]
    #[inline(always)]
    pub fn pwrenpdm0(&self) -> Pwrenpdm0R {
        Pwrenpdm0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power up audio subsystem PDM1 domain"]
    #[inline(always)]
    pub fn pwrenpdm1(&self) -> Pwrenpdm1R {
        Pwrenpdm1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power up audio subsystem PDM2 domain"]
    #[inline(always)]
    pub fn pwrenpdm2(&self) -> Pwrenpdm2R {
        Pwrenpdm2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power up audio subsystem PDM3 domain"]
    #[inline(always)]
    pub fn pwrenpdm3(&self) -> Pwrenpdm3R {
        Pwrenpdm3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power up audio subsystem I2S0 domain"]
    #[inline(always)]
    pub fn pwreni2s0(&self) -> Pwreni2s0R {
        Pwreni2s0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power up audio subsystem I2S1 domain"]
    #[inline(always)]
    pub fn pwreni2s1(&self) -> Pwreni2s1R {
        Pwreni2s1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Power up audio subsystem ADC domain"]
    #[inline(always)]
    pub fn pwrenaudadc(&self) -> PwrenaudadcR {
        PwrenaudadcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable one or more DSP subsystems"]
    #[inline(always)]
    pub fn pwrendspa(&self) -> PwrendspaR {
        PwrendspaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power up Audio Record"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenaudrec(&mut self) -> PwrenaudrecW<AudsspwrenSpec> {
        PwrenaudrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Power up Audio Playback"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenaudpb(&mut self) -> PwrenaudpbW<AudsspwrenSpec> {
        PwrenaudpbW::new(self, 1)
    }
    #[doc = "Bit 2 - Power up audio subsystem PDM0 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenpdm0(&mut self) -> Pwrenpdm0W<AudsspwrenSpec> {
        Pwrenpdm0W::new(self, 2)
    }
    #[doc = "Bit 3 - Power up audio subsystem PDM1 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenpdm1(&mut self) -> Pwrenpdm1W<AudsspwrenSpec> {
        Pwrenpdm1W::new(self, 3)
    }
    #[doc = "Bit 4 - Power up audio subsystem PDM2 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenpdm2(&mut self) -> Pwrenpdm2W<AudsspwrenSpec> {
        Pwrenpdm2W::new(self, 4)
    }
    #[doc = "Bit 5 - Power up audio subsystem PDM3 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenpdm3(&mut self) -> Pwrenpdm3W<AudsspwrenSpec> {
        Pwrenpdm3W::new(self, 5)
    }
    #[doc = "Bit 6 - Power up audio subsystem I2S0 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwreni2s0(&mut self) -> Pwreni2s0W<AudsspwrenSpec> {
        Pwreni2s0W::new(self, 6)
    }
    #[doc = "Bit 7 - Power up audio subsystem I2S1 domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwreni2s1(&mut self) -> Pwreni2s1W<AudsspwrenSpec> {
        Pwreni2s1W::new(self, 7)
    }
    #[doc = "Bit 10 - Power up audio subsystem ADC domain"]
    #[inline(always)]
    #[must_use]
    pub fn pwrenaudadc(&mut self) -> PwrenaudadcW<AudsspwrenSpec> {
        PwrenaudadcW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable one or more DSP subsystems"]
    #[inline(always)]
    #[must_use]
    pub fn pwrendspa(&mut self) -> PwrendspaW<AudsspwrenSpec> {
        PwrendspaW::new(self, 11)
    }
}
#[doc = "This enables various power domains in audio subsystem.\n\nYou can [`read`](crate::Reg::read) this register and get [`audsspwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audsspwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudsspwrenSpec;
impl crate::RegisterSpec for AudsspwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audsspwren::R`](R) reader structure"]
impl crate::Readable for AudsspwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`audsspwren::W`](W) writer structure"]
impl crate::Writable for AudsspwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDSSPWREN to value 0"]
impl crate::Resettable for AudsspwrenSpec {
    const RESET_VALUE: u32 = 0;
}
