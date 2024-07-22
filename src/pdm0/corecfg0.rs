#[doc = "Register `CORECFG0` reader"]
pub type R = crate::R<Corecfg0Spec>;
#[doc = "Register `CORECFG0` writer"]
pub type W = crate::W<Corecfg0Spec>;
#[doc = "Left/Right channel swap when = 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrswap {
    #[doc = "0: Disable left/right channel swapping."]
    Dis = 0,
    #[doc = "1: Enable left/right channel swapping."]
    En = 1,
}
impl From<Lrswap> for bool {
    #[inline(always)]
    fn from(variant: Lrswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRSWAP` reader - Left/Right channel swap when = 1"]
pub type LrswapR = crate::BitReader<Lrswap>;
impl LrswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrswap {
        match self.bits {
            false => Lrswap::Dis,
            true => Lrswap::En,
        }
    }
    #[doc = "Disable left/right channel swapping."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lrswap::Dis
    }
    #[doc = "Enable left/right channel swapping."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Lrswap::En
    }
}
#[doc = "Field `LRSWAP` writer - Left/Right channel swap when = 1"]
pub type LrswapW<'a, REG> = crate::BitWriter<'a, REG, Lrswap>;
impl<'a, REG> LrswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable left/right channel swapping."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lrswap::Dis)
    }
    #[doc = "Enable left/right channel swapping."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Lrswap::En)
    }
}
#[doc = "Field `SOFTMUTE` reader - Soft mute enable when = 1"]
pub type SoftmuteR = crate::BitReader;
#[doc = "Field `SOFTMUTE` writer - Soft mute enable when = 1"]
pub type SoftmuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Set number of PDMA_CKO cycles during gain setting changes or soft mute\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scycles {
    #[doc = "0: Zero PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _0cycles = 0,
    #[doc = "1: One PDMA_CK0 clock cycle during gain setting changes or soft mute."]
    _1cycles = 1,
    #[doc = "2: Two PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _2cycles = 2,
    #[doc = "3: Three PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _3cycles = 3,
    #[doc = "4: Four PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _4cycles = 4,
    #[doc = "5: Five PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _5cycles = 5,
    #[doc = "6: Six PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _6cycles = 6,
    #[doc = "7: Seven PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    _7cycles = 7,
}
impl From<Scycles> for u8 {
    #[inline(always)]
    fn from(variant: Scycles) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scycles {
    type Ux = u8;
}
impl crate::IsEnum for Scycles {}
#[doc = "Field `SCYCLES` reader - Set number of PDMA_CKO cycles during gain setting changes or soft mute"]
pub type ScyclesR = crate::FieldReader<Scycles>;
impl ScyclesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scycles {
        match self.bits {
            0 => Scycles::_0cycles,
            1 => Scycles::_1cycles,
            2 => Scycles::_2cycles,
            3 => Scycles::_3cycles,
            4 => Scycles::_4cycles,
            5 => Scycles::_5cycles,
            6 => Scycles::_6cycles,
            7 => Scycles::_7cycles,
            _ => unreachable!(),
        }
    }
    #[doc = "Zero PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_0cycles(&self) -> bool {
        *self == Scycles::_0cycles
    }
    #[doc = "One PDMA_CK0 clock cycle during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_1cycles(&self) -> bool {
        *self == Scycles::_1cycles
    }
    #[doc = "Two PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == Scycles::_2cycles
    }
    #[doc = "Three PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == Scycles::_3cycles
    }
    #[doc = "Four PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == Scycles::_4cycles
    }
    #[doc = "Five PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_5cycles(&self) -> bool {
        *self == Scycles::_5cycles
    }
    #[doc = "Six PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_6cycles(&self) -> bool {
        *self == Scycles::_6cycles
    }
    #[doc = "Seven PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn is_7cycles(&self) -> bool {
        *self == Scycles::_7cycles
    }
}
#[doc = "Field `SCYCLES` writer - Set number of PDMA_CKO cycles during gain setting changes or soft mute"]
pub type ScyclesW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scycles, crate::Safe>;
impl<'a, REG> ScyclesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _0cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_0cycles)
    }
    #[doc = "One PDMA_CK0 clock cycle during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _1cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_1cycles)
    }
    #[doc = "Two PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_2cycles)
    }
    #[doc = "Three PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_3cycles)
    }
    #[doc = "Four PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_4cycles)
    }
    #[doc = "Five PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _5cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_5cycles)
    }
    #[doc = "Six PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _6cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_6cycles)
    }
    #[doc = "Seven PDMA_CK0 clock cycles during gain setting changes or soft mute."]
    #[inline(always)]
    pub fn _7cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Scycles::_7cycles)
    }
}
#[doc = "Field `HPGAIN` reader - Adjust High Pass filter coefficients"]
pub type HpgainR = crate::FieldReader;
#[doc = "Field `HPGAIN` writer - Adjust High Pass filter coefficients"]
pub type HpgainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Disable high pass filter when = 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adchpd {
    #[doc = "0: Disable high pass filter."]
    Dis = 0,
    #[doc = "1: Enable high pass filter."]
    En = 1,
}
impl From<Adchpd> for bool {
    #[inline(always)]
    fn from(variant: Adchpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCHPD` reader - Disable high pass filter when = 1"]
pub type AdchpdR = crate::BitReader<Adchpd>;
impl AdchpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adchpd {
        match self.bits {
            false => Adchpd::Dis,
            true => Adchpd::En,
        }
    }
    #[doc = "Disable high pass filter."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Adchpd::Dis
    }
    #[doc = "Enable high pass filter."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Adchpd::En
    }
}
#[doc = "Field `ADCHPD` writer - Disable high pass filter when = 1"]
pub type AdchpdW<'a, REG> = crate::BitWriter<'a, REG, Adchpd>;
impl<'a, REG> AdchpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable high pass filter."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Adchpd::Dis)
    }
    #[doc = "Enable high pass filter."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Adchpd::En)
    }
}
#[doc = "Field `MCLKDIV` reader - PDMA_CKO frequency divisor. MCLKDIV > 0. MCLKDIV = 0 PROHIBITED. MCLKDIV = (PDM_CLK /Fsin / (DIVMCLKQ + 1)) -1"]
pub type MclkdivR = crate::FieldReader;
#[doc = "Field `MCLKDIV` writer - PDMA_CKO frequency divisor. MCLKDIV > 0. MCLKDIV = 0 PROHIBITED. MCLKDIV = (PDM_CLK /Fsin / (DIVMCLKQ + 1)) -1"]
pub type MclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINCRATE` reader - Sinc decimation rate. SINC_RATE = OSR /2. OSR = Fsin / Fsout. Must be even. 16 to 64 allowed. 96 allowed for special configuration."]
pub type SincrateR = crate::FieldReader;
#[doc = "Field `SINCRATE` writer - Sinc decimation rate. SINC_RATE = OSR /2. OSR = Fsin / Fsout. Must be even. 16 to 64 allowed. 96 allowed for special configuration."]
pub type SincrateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Left Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB; enum name = M12_0DB value = 0x0 desc = Left channel PGA gain = -12.0 dB\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgal {
    #[doc = "1: Left channel PGA gain = -10.5 dB"]
    M10_5db = 1,
    #[doc = "2: Left channel PGA gain = -9.0 dB"]
    M9_0db = 2,
    #[doc = "3: Left channel PGA gain = -7.5 dB"]
    M7_5db = 3,
    #[doc = "4: Left channel PGA gain = -6.0 dB"]
    M6_0db = 4,
    #[doc = "5: Left channel PGA gain = -4.5 dB"]
    M4_5db = 5,
    #[doc = "6: Left channel PGA gain = -3.0 dB"]
    M3_0db = 6,
    #[doc = "7: Left channel PGA gain = -1.5 dB"]
    M1_5db = 7,
    #[doc = "8: Left channel PGA gain = 0 DB"]
    _0db = 8,
    #[doc = "9: Left channel PGA gain = 1.5 dB"]
    P1_5db = 9,
    #[doc = "10: Left channel PGA gain = 3.0 dB"]
    P3_0db = 10,
    #[doc = "11: Left channel PGA gain = 4.5 dB"]
    P4_5db = 11,
    #[doc = "12: Left channel PGA gain = 6.0 DB"]
    P6_0db = 12,
    #[doc = "13: Left channel PGA gain = 7.5 dB"]
    P7_5db = 13,
    #[doc = "14: Left channel PGA gain = 9.0 dB"]
    P9_0db = 14,
    #[doc = "15: Left channel PGA gain = 10.5 dB"]
    P10_5db = 15,
    #[doc = "16: Left channel PGA gain = 12.0 DB"]
    P12_0db = 16,
    #[doc = "17: Left channel PGA gain = 13.5 dB"]
    P13_5db = 17,
    #[doc = "18: Left channel PGA gain = 15.0 dB"]
    P15_0db = 18,
    #[doc = "19: Left channel PGA gain = 16.5 dB"]
    P16_5db = 19,
    #[doc = "20: Left channel PGA gain = 18.0 DB"]
    P18_0db = 20,
    #[doc = "21: Left channel PGA gain = 19.5 dB"]
    P19_5db = 21,
    #[doc = "22: Left channel PGA gain = 21.0 dB"]
    P21_0db = 22,
    #[doc = "23: Left channel PGA gain = 22.5 dB"]
    P22_5db = 23,
    #[doc = "24: Left channel PGA gain = 24.0 DB"]
    P24_0db = 24,
    #[doc = "25: Left channel PGA gain = 25.5 dB"]
    P25_5db = 25,
    #[doc = "26: Left channel PGA gain = 27.0 dB"]
    P27_0db = 26,
    #[doc = "27: Left channel PGA gain = 28.5 dB"]
    P28_5db = 27,
    #[doc = "28: Left channel PGA gain = 30.0 DB"]
    P30_0db = 28,
    #[doc = "29: Left channel PGA gain = 31.5 dB"]
    P31_5db = 29,
    #[doc = "30: Left channel PGA gain = 33.0 dB"]
    P33_0db = 30,
    #[doc = "31: Left channel PGA gain = 34.5 dB"]
    P34_5db = 31,
}
impl From<Pgal> for u8 {
    #[inline(always)]
    fn from(variant: Pgal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgal {
    type Ux = u8;
}
impl crate::IsEnum for Pgal {}
#[doc = "Field `PGAL` reader - Left Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB; enum name = M12_0DB value = 0x0 desc = Left channel PGA gain = -12.0 dB"]
pub type PgalR = crate::FieldReader<Pgal>;
impl PgalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pgal> {
        match self.bits {
            1 => Some(Pgal::M10_5db),
            2 => Some(Pgal::M9_0db),
            3 => Some(Pgal::M7_5db),
            4 => Some(Pgal::M6_0db),
            5 => Some(Pgal::M4_5db),
            6 => Some(Pgal::M3_0db),
            7 => Some(Pgal::M1_5db),
            8 => Some(Pgal::_0db),
            9 => Some(Pgal::P1_5db),
            10 => Some(Pgal::P3_0db),
            11 => Some(Pgal::P4_5db),
            12 => Some(Pgal::P6_0db),
            13 => Some(Pgal::P7_5db),
            14 => Some(Pgal::P9_0db),
            15 => Some(Pgal::P10_5db),
            16 => Some(Pgal::P12_0db),
            17 => Some(Pgal::P13_5db),
            18 => Some(Pgal::P15_0db),
            19 => Some(Pgal::P16_5db),
            20 => Some(Pgal::P18_0db),
            21 => Some(Pgal::P19_5db),
            22 => Some(Pgal::P21_0db),
            23 => Some(Pgal::P22_5db),
            24 => Some(Pgal::P24_0db),
            25 => Some(Pgal::P25_5db),
            26 => Some(Pgal::P27_0db),
            27 => Some(Pgal::P28_5db),
            28 => Some(Pgal::P30_0db),
            29 => Some(Pgal::P31_5db),
            30 => Some(Pgal::P33_0db),
            31 => Some(Pgal::P34_5db),
            _ => None,
        }
    }
    #[doc = "Left channel PGA gain = -10.5 dB"]
    #[inline(always)]
    pub fn is_m10_5db(&self) -> bool {
        *self == Pgal::M10_5db
    }
    #[doc = "Left channel PGA gain = -9.0 dB"]
    #[inline(always)]
    pub fn is_m9_0db(&self) -> bool {
        *self == Pgal::M9_0db
    }
    #[doc = "Left channel PGA gain = -7.5 dB"]
    #[inline(always)]
    pub fn is_m7_5db(&self) -> bool {
        *self == Pgal::M7_5db
    }
    #[doc = "Left channel PGA gain = -6.0 dB"]
    #[inline(always)]
    pub fn is_m6_0db(&self) -> bool {
        *self == Pgal::M6_0db
    }
    #[doc = "Left channel PGA gain = -4.5 dB"]
    #[inline(always)]
    pub fn is_m4_5db(&self) -> bool {
        *self == Pgal::M4_5db
    }
    #[doc = "Left channel PGA gain = -3.0 dB"]
    #[inline(always)]
    pub fn is_m3_0db(&self) -> bool {
        *self == Pgal::M3_0db
    }
    #[doc = "Left channel PGA gain = -1.5 dB"]
    #[inline(always)]
    pub fn is_m1_5db(&self) -> bool {
        *self == Pgal::M1_5db
    }
    #[doc = "Left channel PGA gain = 0 DB"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == Pgal::_0db
    }
    #[doc = "Left channel PGA gain = 1.5 dB"]
    #[inline(always)]
    pub fn is_p1_5db(&self) -> bool {
        *self == Pgal::P1_5db
    }
    #[doc = "Left channel PGA gain = 3.0 dB"]
    #[inline(always)]
    pub fn is_p3_0db(&self) -> bool {
        *self == Pgal::P3_0db
    }
    #[doc = "Left channel PGA gain = 4.5 dB"]
    #[inline(always)]
    pub fn is_p4_5db(&self) -> bool {
        *self == Pgal::P4_5db
    }
    #[doc = "Left channel PGA gain = 6.0 DB"]
    #[inline(always)]
    pub fn is_p6_0db(&self) -> bool {
        *self == Pgal::P6_0db
    }
    #[doc = "Left channel PGA gain = 7.5 dB"]
    #[inline(always)]
    pub fn is_p7_5db(&self) -> bool {
        *self == Pgal::P7_5db
    }
    #[doc = "Left channel PGA gain = 9.0 dB"]
    #[inline(always)]
    pub fn is_p9_0db(&self) -> bool {
        *self == Pgal::P9_0db
    }
    #[doc = "Left channel PGA gain = 10.5 dB"]
    #[inline(always)]
    pub fn is_p10_5db(&self) -> bool {
        *self == Pgal::P10_5db
    }
    #[doc = "Left channel PGA gain = 12.0 DB"]
    #[inline(always)]
    pub fn is_p12_0db(&self) -> bool {
        *self == Pgal::P12_0db
    }
    #[doc = "Left channel PGA gain = 13.5 dB"]
    #[inline(always)]
    pub fn is_p13_5db(&self) -> bool {
        *self == Pgal::P13_5db
    }
    #[doc = "Left channel PGA gain = 15.0 dB"]
    #[inline(always)]
    pub fn is_p15_0db(&self) -> bool {
        *self == Pgal::P15_0db
    }
    #[doc = "Left channel PGA gain = 16.5 dB"]
    #[inline(always)]
    pub fn is_p16_5db(&self) -> bool {
        *self == Pgal::P16_5db
    }
    #[doc = "Left channel PGA gain = 18.0 DB"]
    #[inline(always)]
    pub fn is_p18_0db(&self) -> bool {
        *self == Pgal::P18_0db
    }
    #[doc = "Left channel PGA gain = 19.5 dB"]
    #[inline(always)]
    pub fn is_p19_5db(&self) -> bool {
        *self == Pgal::P19_5db
    }
    #[doc = "Left channel PGA gain = 21.0 dB"]
    #[inline(always)]
    pub fn is_p21_0db(&self) -> bool {
        *self == Pgal::P21_0db
    }
    #[doc = "Left channel PGA gain = 22.5 dB"]
    #[inline(always)]
    pub fn is_p22_5db(&self) -> bool {
        *self == Pgal::P22_5db
    }
    #[doc = "Left channel PGA gain = 24.0 DB"]
    #[inline(always)]
    pub fn is_p24_0db(&self) -> bool {
        *self == Pgal::P24_0db
    }
    #[doc = "Left channel PGA gain = 25.5 dB"]
    #[inline(always)]
    pub fn is_p25_5db(&self) -> bool {
        *self == Pgal::P25_5db
    }
    #[doc = "Left channel PGA gain = 27.0 dB"]
    #[inline(always)]
    pub fn is_p27_0db(&self) -> bool {
        *self == Pgal::P27_0db
    }
    #[doc = "Left channel PGA gain = 28.5 dB"]
    #[inline(always)]
    pub fn is_p28_5db(&self) -> bool {
        *self == Pgal::P28_5db
    }
    #[doc = "Left channel PGA gain = 30.0 DB"]
    #[inline(always)]
    pub fn is_p30_0db(&self) -> bool {
        *self == Pgal::P30_0db
    }
    #[doc = "Left channel PGA gain = 31.5 dB"]
    #[inline(always)]
    pub fn is_p31_5db(&self) -> bool {
        *self == Pgal::P31_5db
    }
    #[doc = "Left channel PGA gain = 33.0 dB"]
    #[inline(always)]
    pub fn is_p33_0db(&self) -> bool {
        *self == Pgal::P33_0db
    }
    #[doc = "Left channel PGA gain = 34.5 dB"]
    #[inline(always)]
    pub fn is_p34_5db(&self) -> bool {
        *self == Pgal::P34_5db
    }
}
#[doc = "Field `PGAL` writer - Left Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB; enum name = M12_0DB value = 0x0 desc = Left channel PGA gain = -12.0 dB"]
pub type PgalW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pgal>;
impl<'a, REG> PgalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Left channel PGA gain = -10.5 dB"]
    #[inline(always)]
    pub fn m10_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M10_5db)
    }
    #[doc = "Left channel PGA gain = -9.0 dB"]
    #[inline(always)]
    pub fn m9_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M9_0db)
    }
    #[doc = "Left channel PGA gain = -7.5 dB"]
    #[inline(always)]
    pub fn m7_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M7_5db)
    }
    #[doc = "Left channel PGA gain = -6.0 dB"]
    #[inline(always)]
    pub fn m6_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M6_0db)
    }
    #[doc = "Left channel PGA gain = -4.5 dB"]
    #[inline(always)]
    pub fn m4_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M4_5db)
    }
    #[doc = "Left channel PGA gain = -3.0 dB"]
    #[inline(always)]
    pub fn m3_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M3_0db)
    }
    #[doc = "Left channel PGA gain = -1.5 dB"]
    #[inline(always)]
    pub fn m1_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::M1_5db)
    }
    #[doc = "Left channel PGA gain = 0 DB"]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::_0db)
    }
    #[doc = "Left channel PGA gain = 1.5 dB"]
    #[inline(always)]
    pub fn p1_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P1_5db)
    }
    #[doc = "Left channel PGA gain = 3.0 dB"]
    #[inline(always)]
    pub fn p3_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P3_0db)
    }
    #[doc = "Left channel PGA gain = 4.5 dB"]
    #[inline(always)]
    pub fn p4_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P4_5db)
    }
    #[doc = "Left channel PGA gain = 6.0 DB"]
    #[inline(always)]
    pub fn p6_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P6_0db)
    }
    #[doc = "Left channel PGA gain = 7.5 dB"]
    #[inline(always)]
    pub fn p7_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P7_5db)
    }
    #[doc = "Left channel PGA gain = 9.0 dB"]
    #[inline(always)]
    pub fn p9_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P9_0db)
    }
    #[doc = "Left channel PGA gain = 10.5 dB"]
    #[inline(always)]
    pub fn p10_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P10_5db)
    }
    #[doc = "Left channel PGA gain = 12.0 DB"]
    #[inline(always)]
    pub fn p12_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P12_0db)
    }
    #[doc = "Left channel PGA gain = 13.5 dB"]
    #[inline(always)]
    pub fn p13_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P13_5db)
    }
    #[doc = "Left channel PGA gain = 15.0 dB"]
    #[inline(always)]
    pub fn p15_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P15_0db)
    }
    #[doc = "Left channel PGA gain = 16.5 dB"]
    #[inline(always)]
    pub fn p16_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P16_5db)
    }
    #[doc = "Left channel PGA gain = 18.0 DB"]
    #[inline(always)]
    pub fn p18_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P18_0db)
    }
    #[doc = "Left channel PGA gain = 19.5 dB"]
    #[inline(always)]
    pub fn p19_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P19_5db)
    }
    #[doc = "Left channel PGA gain = 21.0 dB"]
    #[inline(always)]
    pub fn p21_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P21_0db)
    }
    #[doc = "Left channel PGA gain = 22.5 dB"]
    #[inline(always)]
    pub fn p22_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P22_5db)
    }
    #[doc = "Left channel PGA gain = 24.0 DB"]
    #[inline(always)]
    pub fn p24_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P24_0db)
    }
    #[doc = "Left channel PGA gain = 25.5 dB"]
    #[inline(always)]
    pub fn p25_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P25_5db)
    }
    #[doc = "Left channel PGA gain = 27.0 dB"]
    #[inline(always)]
    pub fn p27_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P27_0db)
    }
    #[doc = "Left channel PGA gain = 28.5 dB"]
    #[inline(always)]
    pub fn p28_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P28_5db)
    }
    #[doc = "Left channel PGA gain = 30.0 DB"]
    #[inline(always)]
    pub fn p30_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P30_0db)
    }
    #[doc = "Left channel PGA gain = 31.5 dB"]
    #[inline(always)]
    pub fn p31_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P31_5db)
    }
    #[doc = "Left channel PGA gain = 33.0 dB"]
    #[inline(always)]
    pub fn p33_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P33_0db)
    }
    #[doc = "Left channel PGA gain = 34.5 dB"]
    #[inline(always)]
    pub fn p34_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgal::P34_5db)
    }
}
#[doc = "Right Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB;\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pgar {
    #[doc = "0: Right channel PGA gain = -12.0 dB"]
    M12_0db = 0,
    #[doc = "1: Right channel PGA gain = -10.5 dB"]
    M10_5db = 1,
    #[doc = "2: Right channel PGA gain = -9.0 dB"]
    M9_0db = 2,
    #[doc = "3: Right channel PGA gain = -7.5 dB"]
    M7_5db = 3,
    #[doc = "4: Right channel PGA gain = -6.0 dB"]
    M6_0db = 4,
    #[doc = "5: Right channel PGA gain = -4.5 dB"]
    M4_5db = 5,
    #[doc = "6: Right channel PGA gain = -3.0 dB"]
    M3_0db = 6,
    #[doc = "7: Right channel PGA gain = -1.5 dB"]
    M1_5db = 7,
    #[doc = "8: Right channel PGA gain = 0 DB"]
    _0db = 8,
    #[doc = "9: Right channel PGA gain = 1.5 dB"]
    P1_5db = 9,
    #[doc = "10: Right channel PGA gain = 3.0 dB"]
    P3_0db = 10,
    #[doc = "11: Right channel PGA gain = 4.5 dB"]
    P4_5db = 11,
    #[doc = "12: Right channel PGA gain = 6.0 DB"]
    P6_0db = 12,
    #[doc = "13: Right channel PGA gain = 7.5 dB"]
    P7_5db = 13,
    #[doc = "14: Right channel PGA gain = 9.0 dB"]
    P9_0db = 14,
    #[doc = "15: Right channel PGA gain = 10.5 dB"]
    P10_5db = 15,
    #[doc = "16: Right channel PGA gain = 12.0 DB"]
    P12_0db = 16,
    #[doc = "17: Right channel PGA gain = 13.5 dB"]
    P13_5db = 17,
    #[doc = "18: Right channel PGA gain = 15.0 dB"]
    P15_0db = 18,
    #[doc = "19: Right channel PGA gain = 16.5 dB"]
    P16_5db = 19,
    #[doc = "20: Right channel PGA gain = 18.0 DB"]
    P18_0db = 20,
    #[doc = "21: Right channel PGA gain = 19.5 dB"]
    P19_5db = 21,
    #[doc = "22: Right channel PGA gain = 21.0 dB"]
    P21_0db = 22,
    #[doc = "23: Right channel PGA gain = 22.5 dB"]
    P22_5db = 23,
    #[doc = "24: Right channel PGA gain = 24.0 DB"]
    P24_0db = 24,
    #[doc = "25: Right channel PGA gain = 25.5 dB"]
    P25_5db = 25,
    #[doc = "26: Right channel PGA gain = 27.0 dB"]
    P27_0db = 26,
    #[doc = "27: Right channel PGA gain = 28.5 dB"]
    P28_5db = 27,
    #[doc = "28: Right channel PGA gain = 30.0 DB"]
    P30_0db = 28,
    #[doc = "29: Right channel PGA gain = 31.5 dB"]
    P31_5db = 29,
    #[doc = "30: Right channel PGA gain = 33.0 dB"]
    P33_0db = 30,
    #[doc = "31: Right channel PGA gain = 34.5 dB"]
    P34_5db = 31,
}
impl From<Pgar> for u8 {
    #[inline(always)]
    fn from(variant: Pgar) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pgar {
    type Ux = u8;
}
impl crate::IsEnum for Pgar {}
#[doc = "Field `PGAR` reader - Right Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB;"]
pub type PgarR = crate::FieldReader<Pgar>;
impl PgarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pgar {
        match self.bits {
            0 => Pgar::M12_0db,
            1 => Pgar::M10_5db,
            2 => Pgar::M9_0db,
            3 => Pgar::M7_5db,
            4 => Pgar::M6_0db,
            5 => Pgar::M4_5db,
            6 => Pgar::M3_0db,
            7 => Pgar::M1_5db,
            8 => Pgar::_0db,
            9 => Pgar::P1_5db,
            10 => Pgar::P3_0db,
            11 => Pgar::P4_5db,
            12 => Pgar::P6_0db,
            13 => Pgar::P7_5db,
            14 => Pgar::P9_0db,
            15 => Pgar::P10_5db,
            16 => Pgar::P12_0db,
            17 => Pgar::P13_5db,
            18 => Pgar::P15_0db,
            19 => Pgar::P16_5db,
            20 => Pgar::P18_0db,
            21 => Pgar::P19_5db,
            22 => Pgar::P21_0db,
            23 => Pgar::P22_5db,
            24 => Pgar::P24_0db,
            25 => Pgar::P25_5db,
            26 => Pgar::P27_0db,
            27 => Pgar::P28_5db,
            28 => Pgar::P30_0db,
            29 => Pgar::P31_5db,
            30 => Pgar::P33_0db,
            31 => Pgar::P34_5db,
            _ => unreachable!(),
        }
    }
    #[doc = "Right channel PGA gain = -12.0 dB"]
    #[inline(always)]
    pub fn is_m12_0db(&self) -> bool {
        *self == Pgar::M12_0db
    }
    #[doc = "Right channel PGA gain = -10.5 dB"]
    #[inline(always)]
    pub fn is_m10_5db(&self) -> bool {
        *self == Pgar::M10_5db
    }
    #[doc = "Right channel PGA gain = -9.0 dB"]
    #[inline(always)]
    pub fn is_m9_0db(&self) -> bool {
        *self == Pgar::M9_0db
    }
    #[doc = "Right channel PGA gain = -7.5 dB"]
    #[inline(always)]
    pub fn is_m7_5db(&self) -> bool {
        *self == Pgar::M7_5db
    }
    #[doc = "Right channel PGA gain = -6.0 dB"]
    #[inline(always)]
    pub fn is_m6_0db(&self) -> bool {
        *self == Pgar::M6_0db
    }
    #[doc = "Right channel PGA gain = -4.5 dB"]
    #[inline(always)]
    pub fn is_m4_5db(&self) -> bool {
        *self == Pgar::M4_5db
    }
    #[doc = "Right channel PGA gain = -3.0 dB"]
    #[inline(always)]
    pub fn is_m3_0db(&self) -> bool {
        *self == Pgar::M3_0db
    }
    #[doc = "Right channel PGA gain = -1.5 dB"]
    #[inline(always)]
    pub fn is_m1_5db(&self) -> bool {
        *self == Pgar::M1_5db
    }
    #[doc = "Right channel PGA gain = 0 DB"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == Pgar::_0db
    }
    #[doc = "Right channel PGA gain = 1.5 dB"]
    #[inline(always)]
    pub fn is_p1_5db(&self) -> bool {
        *self == Pgar::P1_5db
    }
    #[doc = "Right channel PGA gain = 3.0 dB"]
    #[inline(always)]
    pub fn is_p3_0db(&self) -> bool {
        *self == Pgar::P3_0db
    }
    #[doc = "Right channel PGA gain = 4.5 dB"]
    #[inline(always)]
    pub fn is_p4_5db(&self) -> bool {
        *self == Pgar::P4_5db
    }
    #[doc = "Right channel PGA gain = 6.0 DB"]
    #[inline(always)]
    pub fn is_p6_0db(&self) -> bool {
        *self == Pgar::P6_0db
    }
    #[doc = "Right channel PGA gain = 7.5 dB"]
    #[inline(always)]
    pub fn is_p7_5db(&self) -> bool {
        *self == Pgar::P7_5db
    }
    #[doc = "Right channel PGA gain = 9.0 dB"]
    #[inline(always)]
    pub fn is_p9_0db(&self) -> bool {
        *self == Pgar::P9_0db
    }
    #[doc = "Right channel PGA gain = 10.5 dB"]
    #[inline(always)]
    pub fn is_p10_5db(&self) -> bool {
        *self == Pgar::P10_5db
    }
    #[doc = "Right channel PGA gain = 12.0 DB"]
    #[inline(always)]
    pub fn is_p12_0db(&self) -> bool {
        *self == Pgar::P12_0db
    }
    #[doc = "Right channel PGA gain = 13.5 dB"]
    #[inline(always)]
    pub fn is_p13_5db(&self) -> bool {
        *self == Pgar::P13_5db
    }
    #[doc = "Right channel PGA gain = 15.0 dB"]
    #[inline(always)]
    pub fn is_p15_0db(&self) -> bool {
        *self == Pgar::P15_0db
    }
    #[doc = "Right channel PGA gain = 16.5 dB"]
    #[inline(always)]
    pub fn is_p16_5db(&self) -> bool {
        *self == Pgar::P16_5db
    }
    #[doc = "Right channel PGA gain = 18.0 DB"]
    #[inline(always)]
    pub fn is_p18_0db(&self) -> bool {
        *self == Pgar::P18_0db
    }
    #[doc = "Right channel PGA gain = 19.5 dB"]
    #[inline(always)]
    pub fn is_p19_5db(&self) -> bool {
        *self == Pgar::P19_5db
    }
    #[doc = "Right channel PGA gain = 21.0 dB"]
    #[inline(always)]
    pub fn is_p21_0db(&self) -> bool {
        *self == Pgar::P21_0db
    }
    #[doc = "Right channel PGA gain = 22.5 dB"]
    #[inline(always)]
    pub fn is_p22_5db(&self) -> bool {
        *self == Pgar::P22_5db
    }
    #[doc = "Right channel PGA gain = 24.0 DB"]
    #[inline(always)]
    pub fn is_p24_0db(&self) -> bool {
        *self == Pgar::P24_0db
    }
    #[doc = "Right channel PGA gain = 25.5 dB"]
    #[inline(always)]
    pub fn is_p25_5db(&self) -> bool {
        *self == Pgar::P25_5db
    }
    #[doc = "Right channel PGA gain = 27.0 dB"]
    #[inline(always)]
    pub fn is_p27_0db(&self) -> bool {
        *self == Pgar::P27_0db
    }
    #[doc = "Right channel PGA gain = 28.5 dB"]
    #[inline(always)]
    pub fn is_p28_5db(&self) -> bool {
        *self == Pgar::P28_5db
    }
    #[doc = "Right channel PGA gain = 30.0 DB"]
    #[inline(always)]
    pub fn is_p30_0db(&self) -> bool {
        *self == Pgar::P30_0db
    }
    #[doc = "Right channel PGA gain = 31.5 dB"]
    #[inline(always)]
    pub fn is_p31_5db(&self) -> bool {
        *self == Pgar::P31_5db
    }
    #[doc = "Right channel PGA gain = 33.0 dB"]
    #[inline(always)]
    pub fn is_p33_0db(&self) -> bool {
        *self == Pgar::P33_0db
    }
    #[doc = "Right channel PGA gain = 34.5 dB"]
    #[inline(always)]
    pub fn is_p34_5db(&self) -> bool {
        *self == Pgar::P34_5db
    }
}
#[doc = "Field `PGAR` writer - Right Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB;"]
pub type PgarW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pgar, crate::Safe>;
impl<'a, REG> PgarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Right channel PGA gain = -12.0 dB"]
    #[inline(always)]
    pub fn m12_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M12_0db)
    }
    #[doc = "Right channel PGA gain = -10.5 dB"]
    #[inline(always)]
    pub fn m10_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M10_5db)
    }
    #[doc = "Right channel PGA gain = -9.0 dB"]
    #[inline(always)]
    pub fn m9_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M9_0db)
    }
    #[doc = "Right channel PGA gain = -7.5 dB"]
    #[inline(always)]
    pub fn m7_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M7_5db)
    }
    #[doc = "Right channel PGA gain = -6.0 dB"]
    #[inline(always)]
    pub fn m6_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M6_0db)
    }
    #[doc = "Right channel PGA gain = -4.5 dB"]
    #[inline(always)]
    pub fn m4_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M4_5db)
    }
    #[doc = "Right channel PGA gain = -3.0 dB"]
    #[inline(always)]
    pub fn m3_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M3_0db)
    }
    #[doc = "Right channel PGA gain = -1.5 dB"]
    #[inline(always)]
    pub fn m1_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::M1_5db)
    }
    #[doc = "Right channel PGA gain = 0 DB"]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::_0db)
    }
    #[doc = "Right channel PGA gain = 1.5 dB"]
    #[inline(always)]
    pub fn p1_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P1_5db)
    }
    #[doc = "Right channel PGA gain = 3.0 dB"]
    #[inline(always)]
    pub fn p3_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P3_0db)
    }
    #[doc = "Right channel PGA gain = 4.5 dB"]
    #[inline(always)]
    pub fn p4_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P4_5db)
    }
    #[doc = "Right channel PGA gain = 6.0 DB"]
    #[inline(always)]
    pub fn p6_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P6_0db)
    }
    #[doc = "Right channel PGA gain = 7.5 dB"]
    #[inline(always)]
    pub fn p7_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P7_5db)
    }
    #[doc = "Right channel PGA gain = 9.0 dB"]
    #[inline(always)]
    pub fn p9_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P9_0db)
    }
    #[doc = "Right channel PGA gain = 10.5 dB"]
    #[inline(always)]
    pub fn p10_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P10_5db)
    }
    #[doc = "Right channel PGA gain = 12.0 DB"]
    #[inline(always)]
    pub fn p12_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P12_0db)
    }
    #[doc = "Right channel PGA gain = 13.5 dB"]
    #[inline(always)]
    pub fn p13_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P13_5db)
    }
    #[doc = "Right channel PGA gain = 15.0 dB"]
    #[inline(always)]
    pub fn p15_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P15_0db)
    }
    #[doc = "Right channel PGA gain = 16.5 dB"]
    #[inline(always)]
    pub fn p16_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P16_5db)
    }
    #[doc = "Right channel PGA gain = 18.0 DB"]
    #[inline(always)]
    pub fn p18_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P18_0db)
    }
    #[doc = "Right channel PGA gain = 19.5 dB"]
    #[inline(always)]
    pub fn p19_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P19_5db)
    }
    #[doc = "Right channel PGA gain = 21.0 dB"]
    #[inline(always)]
    pub fn p21_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P21_0db)
    }
    #[doc = "Right channel PGA gain = 22.5 dB"]
    #[inline(always)]
    pub fn p22_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P22_5db)
    }
    #[doc = "Right channel PGA gain = 24.0 DB"]
    #[inline(always)]
    pub fn p24_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P24_0db)
    }
    #[doc = "Right channel PGA gain = 25.5 dB"]
    #[inline(always)]
    pub fn p25_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P25_5db)
    }
    #[doc = "Right channel PGA gain = 27.0 dB"]
    #[inline(always)]
    pub fn p27_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P27_0db)
    }
    #[doc = "Right channel PGA gain = 28.5 dB"]
    #[inline(always)]
    pub fn p28_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P28_5db)
    }
    #[doc = "Right channel PGA gain = 30.0 DB"]
    #[inline(always)]
    pub fn p30_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P30_0db)
    }
    #[doc = "Right channel PGA gain = 31.5 dB"]
    #[inline(always)]
    pub fn p31_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P31_5db)
    }
    #[doc = "Right channel PGA gain = 33.0 dB"]
    #[inline(always)]
    pub fn p33_0db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P33_0db)
    }
    #[doc = "Right channel PGA gain = 34.5 dB"]
    #[inline(always)]
    pub fn p34_5db(self) -> &'a mut crate::W<REG> {
        self.variant(Pgar::P34_5db)
    }
}
impl R {
    #[doc = "Bit 0 - Left/Right channel swap when = 1"]
    #[inline(always)]
    pub fn lrswap(&self) -> LrswapR {
        LrswapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft mute enable when = 1"]
    #[inline(always)]
    pub fn softmute(&self) -> SoftmuteR {
        SoftmuteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Set number of PDMA_CKO cycles during gain setting changes or soft mute"]
    #[inline(always)]
    pub fn scycles(&self) -> ScyclesR {
        ScyclesR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Adjust High Pass filter coefficients"]
    #[inline(always)]
    pub fn hpgain(&self) -> HpgainR {
        HpgainR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Disable high pass filter when = 1"]
    #[inline(always)]
    pub fn adchpd(&self) -> AdchpdR {
        AdchpdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - PDMA_CKO frequency divisor. MCLKDIV > 0. MCLKDIV = 0 PROHIBITED. MCLKDIV = (PDM_CLK /Fsin / (DIVMCLKQ + 1)) -1"]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MclkdivR {
        MclkdivR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:20 - Sinc decimation rate. SINC_RATE = OSR /2. OSR = Fsin / Fsout. Must be even. 16 to 64 allowed. 96 allowed for special configuration."]
    #[inline(always)]
    pub fn sincrate(&self) -> SincrateR {
        SincrateR::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:25 - Left Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB; enum name = M12_0DB value = 0x0 desc = Left channel PGA gain = -12.0 dB"]
    #[inline(always)]
    pub fn pgal(&self) -> PgalR {
        PgalR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - Right Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB;"]
    #[inline(always)]
    pub fn pgar(&self) -> PgarR {
        PgarR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Left/Right channel swap when = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lrswap(&mut self) -> LrswapW<Corecfg0Spec> {
        LrswapW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft mute enable when = 1"]
    #[inline(always)]
    #[must_use]
    pub fn softmute(&mut self) -> SoftmuteW<Corecfg0Spec> {
        SoftmuteW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Set number of PDMA_CKO cycles during gain setting changes or soft mute"]
    #[inline(always)]
    #[must_use]
    pub fn scycles(&mut self) -> ScyclesW<Corecfg0Spec> {
        ScyclesW::new(self, 2)
    }
    #[doc = "Bits 5:8 - Adjust High Pass filter coefficients"]
    #[inline(always)]
    #[must_use]
    pub fn hpgain(&mut self) -> HpgainW<Corecfg0Spec> {
        HpgainW::new(self, 5)
    }
    #[doc = "Bit 9 - Disable high pass filter when = 1"]
    #[inline(always)]
    #[must_use]
    pub fn adchpd(&mut self) -> AdchpdW<Corecfg0Spec> {
        AdchpdW::new(self, 9)
    }
    #[doc = "Bits 10:13 - PDMA_CKO frequency divisor. MCLKDIV > 0. MCLKDIV = 0 PROHIBITED. MCLKDIV = (PDM_CLK /Fsin / (DIVMCLKQ + 1)) -1"]
    #[inline(always)]
    #[must_use]
    pub fn mclkdiv(&mut self) -> MclkdivW<Corecfg0Spec> {
        MclkdivW::new(self, 10)
    }
    #[doc = "Bits 14:20 - Sinc decimation rate. SINC_RATE = OSR /2. OSR = Fsin / Fsout. Must be even. 16 to 64 allowed. 96 allowed for special configuration."]
    #[inline(always)]
    #[must_use]
    pub fn sincrate(&mut self) -> SincrateW<Corecfg0Spec> {
        SincrateW::new(self, 14)
    }
    #[doc = "Bits 21:25 - Left Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB; enum name = M12_0DB value = 0x0 desc = Left channel PGA gain = -12.0 dB"]
    #[inline(always)]
    #[must_use]
    pub fn pgal(&mut self) -> PgalW<Corecfg0Spec> {
        PgalW::new(self, 21)
    }
    #[doc = "Bits 26:30 - Right Channel PGA Gain: +1.5dB/step, -12dB ~ +34.5dB;"]
    #[inline(always)]
    #[must_use]
    pub fn pgar(&mut self) -> PgarW<Corecfg0Spec> {
        PgarW::new(self, 26)
    }
}
#[doc = "PDM to PCM Core Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`corecfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corecfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Corecfg0Spec;
impl crate::RegisterSpec for Corecfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corecfg0::R`](R) reader structure"]
impl crate::Readable for Corecfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`corecfg0::W`](W) writer structure"]
impl crate::Writable for Corecfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORECFG0 to value 0x2108_0f64"]
impl crate::Resettable for Corecfg0Spec {
    const RESET_VALUE: u32 = 0x2108_0f64;
}
