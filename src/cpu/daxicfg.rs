#[doc = "Register `DAXICFG` reader"]
pub type R = crate::R<DaxicfgSpec>;
#[doc = "Register `DAXICFG` writer"]
pub type W = crate::W<DaxicfgSpec>;
#[doc = "Level of free buffers to flush out dirty buffers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flushlevel {
    #[doc = "0: Flush out dirty buffers if 3 or more ane enabled and less than two are free or if 2 are enabled and none are free."]
    Two = 0,
    #[doc = "1: Flush out dirty buffers if 3 or more are enabled less than three are free or if 2 are enabled and less then two are free."]
    Three = 1,
}
impl From<Flushlevel> for bool {
    #[inline(always)]
    fn from(variant: Flushlevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSHLEVEL` reader - Level of free buffers to flush out dirty buffers."]
pub type FlushlevelR = crate::BitReader<Flushlevel>;
impl FlushlevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flushlevel {
        match self.bits {
            false => Flushlevel::Two,
            true => Flushlevel::Three,
        }
    }
    #[doc = "Flush out dirty buffers if 3 or more ane enabled and less than two are free or if 2 are enabled and none are free."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Flushlevel::Two
    }
    #[doc = "Flush out dirty buffers if 3 or more are enabled less than three are free or if 2 are enabled and less then two are free."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Flushlevel::Three
    }
}
#[doc = "Field `FLUSHLEVEL` writer - Level of free buffers to flush out dirty buffers."]
pub type FlushlevelW<'a, REG> = crate::BitWriter<'a, REG, Flushlevel>;
impl<'a, REG> FlushlevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flush out dirty buffers if 3 or more ane enabled and less than two are free or if 2 are enabled and none are free."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Flushlevel::Two)
    }
    #[doc = "Flush out dirty buffers if 3 or more are enabled less than three are free or if 2 are enabled and less then two are free."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Flushlevel::Three)
    }
}
#[doc = "Enables flushing out shared lines using the aging mechanism.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Agingsenable {
    #[doc = "0: Flushing out shared entries using aging mechanism disabled."]
    Dis = 0,
    #[doc = "1: Flushing out shared entries using aging mechanism enabled."]
    En = 1,
}
impl From<Agingsenable> for bool {
    #[inline(always)]
    fn from(variant: Agingsenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGINGSENABLE` reader - Enables flushing out shared lines using the aging mechanism."]
pub type AgingsenableR = crate::BitReader<Agingsenable>;
impl AgingsenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Agingsenable {
        match self.bits {
            false => Agingsenable::Dis,
            true => Agingsenable::En,
        }
    }
    #[doc = "Flushing out shared entries using aging mechanism disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Agingsenable::Dis
    }
    #[doc = "Flushing out shared entries using aging mechanism enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Agingsenable::En
    }
}
#[doc = "Field `AGINGSENABLE` writer - Enables flushing out shared lines using the aging mechanism."]
pub type AgingsenableW<'a, REG> = crate::BitWriter<'a, REG, Agingsenable>;
impl<'a, REG> AgingsenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flushing out shared entries using aging mechanism disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Agingsenable::Dis)
    }
    #[doc = "Flushing out shared entries using aging mechanism enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Agingsenable::En)
    }
}
#[doc = "Passes requests through DAXI logic, disables caching lines in the DAXI line buffers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daxipassthrough {
    #[doc = "0: Disable pass through mode, caching lines in DAXI enabled."]
    Dis = 0,
    #[doc = "1: Enable pass through mode, caching lines in DAXI disabled."]
    En = 1,
}
impl From<Daxipassthrough> for bool {
    #[inline(always)]
    fn from(variant: Daxipassthrough) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAXIPASSTHROUGH` reader - Passes requests through DAXI logic, disables caching lines in the DAXI line buffers."]
pub type DaxipassthroughR = crate::BitReader<Daxipassthrough>;
impl DaxipassthroughR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daxipassthrough {
        match self.bits {
            false => Daxipassthrough::Dis,
            true => Daxipassthrough::En,
        }
    }
    #[doc = "Disable pass through mode, caching lines in DAXI enabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Daxipassthrough::Dis
    }
    #[doc = "Enable pass through mode, caching lines in DAXI disabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Daxipassthrough::En
    }
}
#[doc = "Field `DAXIPASSTHROUGH` writer - Passes requests through DAXI logic, disables caching lines in the DAXI line buffers."]
pub type DaxipassthroughW<'a, REG> = crate::BitWriter<'a, REG, Daxipassthrough>;
impl<'a, REG> DaxipassthroughW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable pass through mode, caching lines in DAXI enabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Daxipassthrough::Dis)
    }
    #[doc = "Enable pass through mode, caching lines in DAXI disabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Daxipassthrough::En)
    }
}
#[doc = "Enables clock gating of DAXI line buffer byte enables.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daxibeclkgateen {
    #[doc = "0: Enable clock gating of DAXI line buffer byte enables."]
    En = 0,
    #[doc = "1: Disable clock gating of DAXI line buffer byte enables."]
    Dis = 1,
}
impl From<Daxibeclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Daxibeclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAXIBECLKGATEEN` reader - Enables clock gating of DAXI line buffer byte enables."]
pub type DaxibeclkgateenR = crate::BitReader<Daxibeclkgateen>;
impl DaxibeclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daxibeclkgateen {
        match self.bits {
            false => Daxibeclkgateen::En,
            true => Daxibeclkgateen::Dis,
        }
    }
    #[doc = "Enable clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Daxibeclkgateen::En
    }
    #[doc = "Disable clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Daxibeclkgateen::Dis
    }
}
#[doc = "Field `DAXIBECLKGATEEN` writer - Enables clock gating of DAXI line buffer byte enables."]
pub type DaxibeclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Daxibeclkgateen>;
impl<'a, REG> DaxibeclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Daxibeclkgateen::En)
    }
    #[doc = "Disable clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Daxibeclkgateen::Dis)
    }
}
#[doc = "Enables clock gating of DAXI line buffer data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daxidataclkgateen {
    #[doc = "0: Enable clock gating of DAXI line buffer data."]
    En = 0,
    #[doc = "1: Disable clock gating of DAXI line buffer data."]
    Dis = 1,
}
impl From<Daxidataclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Daxidataclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAXIDATACLKGATEEN` reader - Enables clock gating of DAXI line buffer data."]
pub type DaxidataclkgateenR = crate::BitReader<Daxidataclkgateen>;
impl DaxidataclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daxidataclkgateen {
        match self.bits {
            false => Daxidataclkgateen::En,
            true => Daxidataclkgateen::Dis,
        }
    }
    #[doc = "Enable clock gating of DAXI line buffer data."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Daxidataclkgateen::En
    }
    #[doc = "Disable clock gating of DAXI line buffer data."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Daxidataclkgateen::Dis
    }
}
#[doc = "Field `DAXIDATACLKGATEEN` writer - Enables clock gating of DAXI line buffer data."]
pub type DaxidataclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Daxidataclkgateen>;
impl<'a, REG> DaxidataclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable clock gating of DAXI line buffer data."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Daxidataclkgateen::En)
    }
    #[doc = "Disable clock gating of DAXI line buffer data."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Daxidataclkgateen::Dis)
    }
}
#[doc = "Enables clock gating of DAXI state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daxistateclkgateen {
    #[doc = "0: Enable clock gating of DAXI state."]
    En = 0,
    #[doc = "1: Disable clock gating of DAXI state."]
    Dis = 1,
}
impl From<Daxistateclkgateen> for bool {
    #[inline(always)]
    fn from(variant: Daxistateclkgateen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAXISTATECLKGATEEN` reader - Enables clock gating of DAXI state."]
pub type DaxistateclkgateenR = crate::BitReader<Daxistateclkgateen>;
impl DaxistateclkgateenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daxistateclkgateen {
        match self.bits {
            false => Daxistateclkgateen::En,
            true => Daxistateclkgateen::Dis,
        }
    }
    #[doc = "Enable clock gating of DAXI state."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Daxistateclkgateen::En
    }
    #[doc = "Disable clock gating of DAXI state."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Daxistateclkgateen::Dis
    }
}
#[doc = "Field `DAXISTATECLKGATEEN` writer - Enables clock gating of DAXI state."]
pub type DaxistateclkgateenW<'a, REG> = crate::BitWriter<'a, REG, Daxistateclkgateen>;
impl<'a, REG> DaxistateclkgateenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable clock gating of DAXI state."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Daxistateclkgateen::En)
    }
    #[doc = "Disable clock gating of DAXI state."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Daxistateclkgateen::Dis)
    }
}
#[doc = "Enables DAXI buffers\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bufferenable {
    #[doc = "0: One buffer mode enabled."]
    One = 0,
    #[doc = "1: Two buffers enabled."]
    Two = 1,
    #[doc = "2: Three buffers enabled."]
    Three = 2,
    #[doc = "3: Four buffers enabled."]
    Four = 3,
    #[doc = "4: Five buffers enabled."]
    Five = 4,
    #[doc = "5: Six buffers enabled."]
    Six = 5,
    #[doc = "6: Seven buffers enabled."]
    Seven = 6,
    #[doc = "7: Eight buffers enabled."]
    Eight = 7,
    #[doc = "8: Eight buffers enabled."]
    Thirteen = 8,
    #[doc = "9: Eight buffers enabled."]
    Fourteen = 9,
    #[doc = "10: Eight buffers enabled."]
    Fifteen = 10,
    #[doc = "11: Eight buffers enabled."]
    Sixteen = 11,
    #[doc = "12: Twenty-nine buffers enabled."]
    Twentynine = 12,
    #[doc = "13: Thirty buffers enabled."]
    Thirty = 13,
    #[doc = "14: Thirty-one buffers enabled."]
    Thirtyone = 14,
    #[doc = "15: Thirty-two buffers enabled."]
    Thirtytwo = 15,
}
impl From<Bufferenable> for u8 {
    #[inline(always)]
    fn from(variant: Bufferenable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bufferenable {
    type Ux = u8;
}
impl crate::IsEnum for Bufferenable {}
#[doc = "Field `BUFFERENABLE` reader - Enables DAXI buffers"]
pub type BufferenableR = crate::FieldReader<Bufferenable>;
impl BufferenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferenable {
        match self.bits {
            0 => Bufferenable::One,
            1 => Bufferenable::Two,
            2 => Bufferenable::Three,
            3 => Bufferenable::Four,
            4 => Bufferenable::Five,
            5 => Bufferenable::Six,
            6 => Bufferenable::Seven,
            7 => Bufferenable::Eight,
            8 => Bufferenable::Thirteen,
            9 => Bufferenable::Fourteen,
            10 => Bufferenable::Fifteen,
            11 => Bufferenable::Sixteen,
            12 => Bufferenable::Twentynine,
            13 => Bufferenable::Thirty,
            14 => Bufferenable::Thirtyone,
            15 => Bufferenable::Thirtytwo,
            _ => unreachable!(),
        }
    }
    #[doc = "One buffer mode enabled."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Bufferenable::One
    }
    #[doc = "Two buffers enabled."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Bufferenable::Two
    }
    #[doc = "Three buffers enabled."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Bufferenable::Three
    }
    #[doc = "Four buffers enabled."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Bufferenable::Four
    }
    #[doc = "Five buffers enabled."]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Bufferenable::Five
    }
    #[doc = "Six buffers enabled."]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Bufferenable::Six
    }
    #[doc = "Seven buffers enabled."]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Bufferenable::Seven
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Bufferenable::Eight
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == Bufferenable::Thirteen
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == Bufferenable::Fourteen
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == Bufferenable::Fifteen
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Bufferenable::Sixteen
    }
    #[doc = "Twenty-nine buffers enabled."]
    #[inline(always)]
    pub fn is_twentynine(&self) -> bool {
        *self == Bufferenable::Twentynine
    }
    #[doc = "Thirty buffers enabled."]
    #[inline(always)]
    pub fn is_thirty(&self) -> bool {
        *self == Bufferenable::Thirty
    }
    #[doc = "Thirty-one buffers enabled."]
    #[inline(always)]
    pub fn is_thirtyone(&self) -> bool {
        *self == Bufferenable::Thirtyone
    }
    #[doc = "Thirty-two buffers enabled."]
    #[inline(always)]
    pub fn is_thirtytwo(&self) -> bool {
        *self == Bufferenable::Thirtytwo
    }
}
#[doc = "Field `BUFFERENABLE` writer - Enables DAXI buffers"]
pub type BufferenableW<'a, REG> = crate::FieldWriter<'a, REG, 4, Bufferenable, crate::Safe>;
impl<'a, REG> BufferenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One buffer mode enabled."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::One)
    }
    #[doc = "Two buffers enabled."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Two)
    }
    #[doc = "Three buffers enabled."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Three)
    }
    #[doc = "Four buffers enabled."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Four)
    }
    #[doc = "Five buffers enabled."]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Five)
    }
    #[doc = "Six buffers enabled."]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Six)
    }
    #[doc = "Seven buffers enabled."]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Seven)
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Eight)
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Thirteen)
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Fourteen)
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Fifteen)
    }
    #[doc = "Eight buffers enabled."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Sixteen)
    }
    #[doc = "Twenty-nine buffers enabled."]
    #[inline(always)]
    pub fn twentynine(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Twentynine)
    }
    #[doc = "Thirty buffers enabled."]
    #[inline(always)]
    pub fn thirty(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Thirty)
    }
    #[doc = "Thirty-one buffers enabled."]
    #[inline(always)]
    pub fn thirtyone(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Thirtyone)
    }
    #[doc = "Thirty-two buffers enabled."]
    #[inline(always)]
    pub fn thirtytwo(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferenable::Thirtytwo)
    }
}
#[doc = "Specifies the relative time that DAXI buffers may remain unused before being flushed. Counter is based on CPU clock cycles and buffers will generally be flushed in 1-2 AGINGCOUNTER timesteps.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Agingcounter {
    #[doc = "0: One step per age"]
    One = 0,
    #[doc = "1: Two steps per age"]
    Two = 1,
    #[doc = "2: Four steps per age"]
    Four = 2,
    #[doc = "3: EIGHT steps per age"]
    Eight = 3,
    #[doc = "4: Sixteen steps per age"]
    Sixteen = 4,
    #[doc = "5: Thirty-two steps per age"]
    Thirtytwo = 5,
    #[doc = "6: 64 steps per age"]
    Sixtyfour = 6,
    #[doc = "7: One hundred twenty-eight steps per age"]
    Onehundredtwentyeight = 7,
    #[doc = "8: Two hundred fifty-six steps per age"]
    Twohunderedfiftysix = 8,
    #[doc = "9: Five hundred twelve steps per age"]
    Fivehundredtwelve = 9,
    #[doc = "10: One thousand twenty-four steps per age"]
    Onek = 10,
    #[doc = "11: Two thousand forty-eight steps per age"]
    Twok = 11,
    #[doc = "12: Four thousand ninety-six steps per age"]
    Fourk = 12,
    #[doc = "13: Eight thousand one hundred ninety-two steps per age"]
    Eightk = 13,
    #[doc = "14: Sixteen thousand three hundred eighty-four steps per age"]
    Sixteenk = 14,
    #[doc = "15: Thirty-two thousand seven hundred sixty-eight steps per age"]
    Thirtytwok = 15,
    #[doc = "16: Sixty-fix thousand five hundred thirty-six steps per age"]
    Sixtyfourk = 16,
}
impl From<Agingcounter> for u8 {
    #[inline(always)]
    fn from(variant: Agingcounter) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Agingcounter {
    type Ux = u8;
}
impl crate::IsEnum for Agingcounter {}
#[doc = "Field `AGINGCOUNTER` reader - Specifies the relative time that DAXI buffers may remain unused before being flushed. Counter is based on CPU clock cycles and buffers will generally be flushed in 1-2 AGINGCOUNTER timesteps."]
pub type AgingcounterR = crate::FieldReader<Agingcounter>;
impl AgingcounterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Agingcounter> {
        match self.bits {
            0 => Some(Agingcounter::One),
            1 => Some(Agingcounter::Two),
            2 => Some(Agingcounter::Four),
            3 => Some(Agingcounter::Eight),
            4 => Some(Agingcounter::Sixteen),
            5 => Some(Agingcounter::Thirtytwo),
            6 => Some(Agingcounter::Sixtyfour),
            7 => Some(Agingcounter::Onehundredtwentyeight),
            8 => Some(Agingcounter::Twohunderedfiftysix),
            9 => Some(Agingcounter::Fivehundredtwelve),
            10 => Some(Agingcounter::Onek),
            11 => Some(Agingcounter::Twok),
            12 => Some(Agingcounter::Fourk),
            13 => Some(Agingcounter::Eightk),
            14 => Some(Agingcounter::Sixteenk),
            15 => Some(Agingcounter::Thirtytwok),
            16 => Some(Agingcounter::Sixtyfourk),
            _ => None,
        }
    }
    #[doc = "One step per age"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Agingcounter::One
    }
    #[doc = "Two steps per age"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Agingcounter::Two
    }
    #[doc = "Four steps per age"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Agingcounter::Four
    }
    #[doc = "EIGHT steps per age"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Agingcounter::Eight
    }
    #[doc = "Sixteen steps per age"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Agingcounter::Sixteen
    }
    #[doc = "Thirty-two steps per age"]
    #[inline(always)]
    pub fn is_thirtytwo(&self) -> bool {
        *self == Agingcounter::Thirtytwo
    }
    #[doc = "64 steps per age"]
    #[inline(always)]
    pub fn is_sixtyfour(&self) -> bool {
        *self == Agingcounter::Sixtyfour
    }
    #[doc = "One hundred twenty-eight steps per age"]
    #[inline(always)]
    pub fn is_onehundredtwentyeight(&self) -> bool {
        *self == Agingcounter::Onehundredtwentyeight
    }
    #[doc = "Two hundred fifty-six steps per age"]
    #[inline(always)]
    pub fn is_twohunderedfiftysix(&self) -> bool {
        *self == Agingcounter::Twohunderedfiftysix
    }
    #[doc = "Five hundred twelve steps per age"]
    #[inline(always)]
    pub fn is_fivehundredtwelve(&self) -> bool {
        *self == Agingcounter::Fivehundredtwelve
    }
    #[doc = "One thousand twenty-four steps per age"]
    #[inline(always)]
    pub fn is_onek(&self) -> bool {
        *self == Agingcounter::Onek
    }
    #[doc = "Two thousand forty-eight steps per age"]
    #[inline(always)]
    pub fn is_twok(&self) -> bool {
        *self == Agingcounter::Twok
    }
    #[doc = "Four thousand ninety-six steps per age"]
    #[inline(always)]
    pub fn is_fourk(&self) -> bool {
        *self == Agingcounter::Fourk
    }
    #[doc = "Eight thousand one hundred ninety-two steps per age"]
    #[inline(always)]
    pub fn is_eightk(&self) -> bool {
        *self == Agingcounter::Eightk
    }
    #[doc = "Sixteen thousand three hundred eighty-four steps per age"]
    #[inline(always)]
    pub fn is_sixteenk(&self) -> bool {
        *self == Agingcounter::Sixteenk
    }
    #[doc = "Thirty-two thousand seven hundred sixty-eight steps per age"]
    #[inline(always)]
    pub fn is_thirtytwok(&self) -> bool {
        *self == Agingcounter::Thirtytwok
    }
    #[doc = "Sixty-fix thousand five hundred thirty-six steps per age"]
    #[inline(always)]
    pub fn is_sixtyfourk(&self) -> bool {
        *self == Agingcounter::Sixtyfourk
    }
}
#[doc = "Field `AGINGCOUNTER` writer - Specifies the relative time that DAXI buffers may remain unused before being flushed. Counter is based on CPU clock cycles and buffers will generally be flushed in 1-2 AGINGCOUNTER timesteps."]
pub type AgingcounterW<'a, REG> = crate::FieldWriter<'a, REG, 5, Agingcounter>;
impl<'a, REG> AgingcounterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One step per age"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::One)
    }
    #[doc = "Two steps per age"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Two)
    }
    #[doc = "Four steps per age"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Four)
    }
    #[doc = "EIGHT steps per age"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Eight)
    }
    #[doc = "Sixteen steps per age"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Sixteen)
    }
    #[doc = "Thirty-two steps per age"]
    #[inline(always)]
    pub fn thirtytwo(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Thirtytwo)
    }
    #[doc = "64 steps per age"]
    #[inline(always)]
    pub fn sixtyfour(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Sixtyfour)
    }
    #[doc = "One hundred twenty-eight steps per age"]
    #[inline(always)]
    pub fn onehundredtwentyeight(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Onehundredtwentyeight)
    }
    #[doc = "Two hundred fifty-six steps per age"]
    #[inline(always)]
    pub fn twohunderedfiftysix(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Twohunderedfiftysix)
    }
    #[doc = "Five hundred twelve steps per age"]
    #[inline(always)]
    pub fn fivehundredtwelve(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Fivehundredtwelve)
    }
    #[doc = "One thousand twenty-four steps per age"]
    #[inline(always)]
    pub fn onek(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Onek)
    }
    #[doc = "Two thousand forty-eight steps per age"]
    #[inline(always)]
    pub fn twok(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Twok)
    }
    #[doc = "Four thousand ninety-six steps per age"]
    #[inline(always)]
    pub fn fourk(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Fourk)
    }
    #[doc = "Eight thousand one hundred ninety-two steps per age"]
    #[inline(always)]
    pub fn eightk(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Eightk)
    }
    #[doc = "Sixteen thousand three hundred eighty-four steps per age"]
    #[inline(always)]
    pub fn sixteenk(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Sixteenk)
    }
    #[doc = "Thirty-two thousand seven hundred sixty-eight steps per age"]
    #[inline(always)]
    pub fn thirtytwok(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Thirtytwok)
    }
    #[doc = "Sixty-fix thousand five hundred thirty-six steps per age"]
    #[inline(always)]
    pub fn sixtyfourk(self) -> &'a mut crate::W<REG> {
        self.variant(Agingcounter::Sixtyfourk)
    }
}
#[doc = "Sets the MRU group population limit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mrugrouplevel {
    #[doc = "0: Maximum level for current number of buffers enabled."]
    Max = 0,
    #[doc = "1: One less than maximum level or zero for current number of buffers enabled."]
    Onelessthanmax = 1,
    #[doc = "2: Two less than maximum level or zero for current number of buffers enabled."]
    Twolessthanmax = 2,
    #[doc = "3: Three less than maximum level or zero for current number of buffers enabled."]
    Threelessthanmax = 3,
}
impl From<Mrugrouplevel> for u8 {
    #[inline(always)]
    fn from(variant: Mrugrouplevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mrugrouplevel {
    type Ux = u8;
}
impl crate::IsEnum for Mrugrouplevel {}
#[doc = "Field `MRUGROUPLEVEL` reader - Sets the MRU group population limit."]
pub type MrugrouplevelR = crate::FieldReader<Mrugrouplevel>;
impl MrugrouplevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrugrouplevel {
        match self.bits {
            0 => Mrugrouplevel::Max,
            1 => Mrugrouplevel::Onelessthanmax,
            2 => Mrugrouplevel::Twolessthanmax,
            3 => Mrugrouplevel::Threelessthanmax,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum level for current number of buffers enabled."]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Mrugrouplevel::Max
    }
    #[doc = "One less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn is_onelessthanmax(&self) -> bool {
        *self == Mrugrouplevel::Onelessthanmax
    }
    #[doc = "Two less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn is_twolessthanmax(&self) -> bool {
        *self == Mrugrouplevel::Twolessthanmax
    }
    #[doc = "Three less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn is_threelessthanmax(&self) -> bool {
        *self == Mrugrouplevel::Threelessthanmax
    }
}
#[doc = "Field `MRUGROUPLEVEL` writer - Sets the MRU group population limit."]
pub type MrugrouplevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mrugrouplevel, crate::Safe>;
impl<'a, REG> MrugrouplevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum level for current number of buffers enabled."]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Mrugrouplevel::Max)
    }
    #[doc = "One less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn onelessthanmax(self) -> &'a mut crate::W<REG> {
        self.variant(Mrugrouplevel::Onelessthanmax)
    }
    #[doc = "Two less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn twolessthanmax(self) -> &'a mut crate::W<REG> {
        self.variant(Mrugrouplevel::Twolessthanmax)
    }
    #[doc = "Three less than maximum level or zero for current number of buffers enabled."]
    #[inline(always)]
    pub fn threelessthanmax(self) -> &'a mut crate::W<REG> {
        self.variant(Mrugrouplevel::Threelessthanmax)
    }
}
impl R {
    #[doc = "Bit 0 - Level of free buffers to flush out dirty buffers."]
    #[inline(always)]
    pub fn flushlevel(&self) -> FlushlevelR {
        FlushlevelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables flushing out shared lines using the aging mechanism."]
    #[inline(always)]
    pub fn agingsenable(&self) -> AgingsenableR {
        AgingsenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Passes requests through DAXI logic, disables caching lines in the DAXI line buffers."]
    #[inline(always)]
    pub fn daxipassthrough(&self) -> DaxipassthroughR {
        DaxipassthroughR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    pub fn daxibeclkgateen(&self) -> DaxibeclkgateenR {
        DaxibeclkgateenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables clock gating of DAXI line buffer data."]
    #[inline(always)]
    pub fn daxidataclkgateen(&self) -> DaxidataclkgateenR {
        DaxidataclkgateenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables clock gating of DAXI state."]
    #[inline(always)]
    pub fn daxistateclkgateen(&self) -> DaxistateclkgateenR {
        DaxistateclkgateenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Enables DAXI buffers"]
    #[inline(always)]
    pub fn bufferenable(&self) -> BufferenableR {
        BufferenableR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Specifies the relative time that DAXI buffers may remain unused before being flushed. Counter is based on CPU clock cycles and buffers will generally be flushed in 1-2 AGINGCOUNTER timesteps."]
    #[inline(always)]
    pub fn agingcounter(&self) -> AgingcounterR {
        AgingcounterR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - Sets the MRU group population limit."]
    #[inline(always)]
    pub fn mrugrouplevel(&self) -> MrugrouplevelR {
        MrugrouplevelR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Level of free buffers to flush out dirty buffers."]
    #[inline(always)]
    #[must_use]
    pub fn flushlevel(&mut self) -> FlushlevelW<DaxicfgSpec> {
        FlushlevelW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables flushing out shared lines using the aging mechanism."]
    #[inline(always)]
    #[must_use]
    pub fn agingsenable(&mut self) -> AgingsenableW<DaxicfgSpec> {
        AgingsenableW::new(self, 1)
    }
    #[doc = "Bit 2 - Passes requests through DAXI logic, disables caching lines in the DAXI line buffers."]
    #[inline(always)]
    #[must_use]
    pub fn daxipassthrough(&mut self) -> DaxipassthroughW<DaxicfgSpec> {
        DaxipassthroughW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables clock gating of DAXI line buffer byte enables."]
    #[inline(always)]
    #[must_use]
    pub fn daxibeclkgateen(&mut self) -> DaxibeclkgateenW<DaxicfgSpec> {
        DaxibeclkgateenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables clock gating of DAXI line buffer data."]
    #[inline(always)]
    #[must_use]
    pub fn daxidataclkgateen(&mut self) -> DaxidataclkgateenW<DaxicfgSpec> {
        DaxidataclkgateenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables clock gating of DAXI state."]
    #[inline(always)]
    #[must_use]
    pub fn daxistateclkgateen(&mut self) -> DaxistateclkgateenW<DaxicfgSpec> {
        DaxistateclkgateenW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Enables DAXI buffers"]
    #[inline(always)]
    #[must_use]
    pub fn bufferenable(&mut self) -> BufferenableW<DaxicfgSpec> {
        BufferenableW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Specifies the relative time that DAXI buffers may remain unused before being flushed. Counter is based on CPU clock cycles and buffers will generally be flushed in 1-2 AGINGCOUNTER timesteps."]
    #[inline(always)]
    #[must_use]
    pub fn agingcounter(&mut self) -> AgingcounterW<DaxicfgSpec> {
        AgingcounterW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Sets the MRU group population limit."]
    #[inline(always)]
    #[must_use]
    pub fn mrugrouplevel(&mut self) -> MrugrouplevelW<DaxicfgSpec> {
        MrugrouplevelW::new(self, 24)
    }
}
#[doc = "DAXI Config\n\nYou can [`read`](crate::Reg::read) this register and get [`daxicfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daxicfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaxicfgSpec;
impl crate::RegisterSpec for DaxicfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daxicfg::R`](R) reader structure"]
impl crate::Readable for DaxicfgSpec {}
#[doc = "`write(|w| ..)` method takes [`daxicfg::W`](W) writer structure"]
impl crate::Writable for DaxicfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAXICFG to value 0x000b_0300"]
impl crate::Resettable for DaxicfgSpec {
    const RESET_VALUE: u32 = 0x000b_0300;
}
