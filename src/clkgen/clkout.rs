#[doc = "Register `CLKOUT` reader"]
pub type R = crate::R<ClkoutSpec>;
#[doc = "Register `CLKOUT` writer"]
pub type W = crate::W<ClkoutSpec>;
#[doc = "CLKOUT signal select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cksel {
    #[doc = "0: LFRC clock source selection"]
    Lfrc = 0,
    #[doc = "1: XT / 2 clock source selection"]
    XtDiv2 = 1,
    #[doc = "2: XT / 4 clock source selection"]
    XtDiv4 = 2,
    #[doc = "3: XT / 8 clock source selection"]
    XtDiv8 = 3,
    #[doc = "4: XT / 16 clock source selection"]
    XtDiv16 = 4,
    #[doc = "5: XT / 32 clock source selection"]
    XtDiv32 = 5,
    #[doc = "16: 1 Hz as selected in RTC"]
    Rtc1hz = 16,
    #[doc = "22: XT / 2097152 (2^21) clock source selection"]
    XtDiv2m = 22,
    #[doc = "23: XT clock source selection"]
    Xt = 23,
    #[doc = "24: 100 Hz as selected in CLKGEN"]
    Cg100hz = 24,
    #[doc = "25: HFRC / 2 clock source selection"]
    HfrcDiv2 = 25,
    #[doc = "26: HFRC / 8 clock source selection"]
    HfrcDiv8 = 26,
    #[doc = "27: HFRC / 16 clock source selection"]
    HfrcDiv16 = 27,
    #[doc = "28: HFRC / 32 clock source selection"]
    HfrcDiv32 = 28,
    #[doc = "29: HFRC / 128 clock source selection"]
    HfrcDiv128 = 29,
    #[doc = "30: HFRC / 256 clock source selection"]
    HfrcDiv256 = 30,
    #[doc = "31: HFRC / 512 clock source selection"]
    HfrcDiv512 = 31,
    #[doc = "32: HFRC / 1024 clock source selection"]
    HfrcDiv1024 = 32,
    #[doc = "35: LFRC / 2 clock source selection"]
    LfrcDiv2 = 35,
    #[doc = "36: LFRC / 32 clock source selection"]
    LfrcDiv32 = 36,
    #[doc = "37: LFRC / 512 clock source selection"]
    LfrcDiv512 = 37,
    #[doc = "38: LFRC / 32768 clock source selection"]
    LfrcDiv32k = 38,
    #[doc = "39: XT / 256 clock source selection"]
    XtDiv256 = 39,
    #[doc = "40: XT / 8192 clock source selection"]
    XtDiv8k = 40,
    #[doc = "41: XT / 65536 (2^16) clock source selection"]
    XtDiv64k = 41,
    #[doc = "42: Uncal LFRC / 16 clock source selection"]
    UlfrcDiv16 = 42,
    #[doc = "43: Uncal LFRC / 128 clock source selection"]
    UlfrcDiv128 = 43,
    #[doc = "44: Uncal LFRC / 1024 clock source selection"]
    Ulfrc1hz = 44,
    #[doc = "45: Uncal LFRC / 4096 clock source selection"]
    UlfrcDiv4k = 45,
    #[doc = "46: Uncal LFRC / 1048576 (2^20) clock source selection"]
    UlfrcDiv1m = 46,
    #[doc = "47: HFRC / 262144 (2^18) clock source selection"]
    HfrcDiv256k = 47,
    #[doc = "48: HFRC / 67108864 (2^26) clock source selection"]
    HfrcDiv64m = 48,
    #[doc = "49: LFRC / 1048576 (2^20) clock source selection"]
    LfrcDiv1m = 49,
    #[doc = "53: XT (not autoenabled)"]
    Xtne = 53,
    #[doc = "54: XT / 16 (not autoenabled)"]
    XtneDiv16 = 54,
    #[doc = "55: LFRC / 32 (not autoenabled)"]
    LfrcneDiv32 = 55,
    #[doc = "57: LFRC (not autoenabled) - Default for undefined values"]
    Lfrcne = 57,
    #[doc = "58: HFRC2 6MHz clock source selection"]
    Hfrc2_6mhz = 58,
    #[doc = "59: HFRC2 24MHz clock source selection"]
    Hfrc2_12mhz = 59,
    #[doc = "60: HFRC2 24MHz clock source selection"]
    Hfrc2_24mhz = 60,
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cksel {
    type Ux = u8;
}
impl crate::IsEnum for Cksel {}
#[doc = "Field `CKSEL` reader - CLKOUT signal select"]
pub type CkselR = crate::FieldReader<Cksel>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cksel> {
        match self.bits {
            0 => Some(Cksel::Lfrc),
            1 => Some(Cksel::XtDiv2),
            2 => Some(Cksel::XtDiv4),
            3 => Some(Cksel::XtDiv8),
            4 => Some(Cksel::XtDiv16),
            5 => Some(Cksel::XtDiv32),
            16 => Some(Cksel::Rtc1hz),
            22 => Some(Cksel::XtDiv2m),
            23 => Some(Cksel::Xt),
            24 => Some(Cksel::Cg100hz),
            25 => Some(Cksel::HfrcDiv2),
            26 => Some(Cksel::HfrcDiv8),
            27 => Some(Cksel::HfrcDiv16),
            28 => Some(Cksel::HfrcDiv32),
            29 => Some(Cksel::HfrcDiv128),
            30 => Some(Cksel::HfrcDiv256),
            31 => Some(Cksel::HfrcDiv512),
            32 => Some(Cksel::HfrcDiv1024),
            35 => Some(Cksel::LfrcDiv2),
            36 => Some(Cksel::LfrcDiv32),
            37 => Some(Cksel::LfrcDiv512),
            38 => Some(Cksel::LfrcDiv32k),
            39 => Some(Cksel::XtDiv256),
            40 => Some(Cksel::XtDiv8k),
            41 => Some(Cksel::XtDiv64k),
            42 => Some(Cksel::UlfrcDiv16),
            43 => Some(Cksel::UlfrcDiv128),
            44 => Some(Cksel::Ulfrc1hz),
            45 => Some(Cksel::UlfrcDiv4k),
            46 => Some(Cksel::UlfrcDiv1m),
            47 => Some(Cksel::HfrcDiv256k),
            48 => Some(Cksel::HfrcDiv64m),
            49 => Some(Cksel::LfrcDiv1m),
            53 => Some(Cksel::Xtne),
            54 => Some(Cksel::XtneDiv16),
            55 => Some(Cksel::LfrcneDiv32),
            57 => Some(Cksel::Lfrcne),
            58 => Some(Cksel::Hfrc2_6mhz),
            59 => Some(Cksel::Hfrc2_12mhz),
            60 => Some(Cksel::Hfrc2_24mhz),
            _ => None,
        }
    }
    #[doc = "LFRC clock source selection"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Cksel::Lfrc
    }
    #[doc = "XT / 2 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == Cksel::XtDiv2
    }
    #[doc = "XT / 4 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == Cksel::XtDiv4
    }
    #[doc = "XT / 8 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == Cksel::XtDiv8
    }
    #[doc = "XT / 16 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == Cksel::XtDiv16
    }
    #[doc = "XT / 32 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == Cksel::XtDiv32
    }
    #[doc = "1 Hz as selected in RTC"]
    #[inline(always)]
    pub fn is_rtc_1hz(&self) -> bool {
        *self == Cksel::Rtc1hz
    }
    #[doc = "XT / 2097152 (2^21) clock source selection"]
    #[inline(always)]
    pub fn is_xt_div2m(&self) -> bool {
        *self == Cksel::XtDiv2m
    }
    #[doc = "XT clock source selection"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == Cksel::Xt
    }
    #[doc = "100 Hz as selected in CLKGEN"]
    #[inline(always)]
    pub fn is_cg_100hz(&self) -> bool {
        *self == Cksel::Cg100hz
    }
    #[doc = "HFRC / 2 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        *self == Cksel::HfrcDiv2
    }
    #[doc = "HFRC / 8 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        *self == Cksel::HfrcDiv8
    }
    #[doc = "HFRC / 16 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == Cksel::HfrcDiv16
    }
    #[doc = "HFRC / 32 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div32(&self) -> bool {
        *self == Cksel::HfrcDiv32
    }
    #[doc = "HFRC / 128 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        *self == Cksel::HfrcDiv128
    }
    #[doc = "HFRC / 256 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == Cksel::HfrcDiv256
    }
    #[doc = "HFRC / 512 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        *self == Cksel::HfrcDiv512
    }
    #[doc = "HFRC / 1024 clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == Cksel::HfrcDiv1024
    }
    #[doc = "LFRC / 2 clock source selection"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == Cksel::LfrcDiv2
    }
    #[doc = "LFRC / 32 clock source selection"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == Cksel::LfrcDiv32
    }
    #[doc = "LFRC / 512 clock source selection"]
    #[inline(always)]
    pub fn is_lfrc_div512(&self) -> bool {
        *self == Cksel::LfrcDiv512
    }
    #[doc = "LFRC / 32768 clock source selection"]
    #[inline(always)]
    pub fn is_lfrc_div32k(&self) -> bool {
        *self == Cksel::LfrcDiv32k
    }
    #[doc = "XT / 256 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        *self == Cksel::XtDiv256
    }
    #[doc = "XT / 8192 clock source selection"]
    #[inline(always)]
    pub fn is_xt_div8k(&self) -> bool {
        *self == Cksel::XtDiv8k
    }
    #[doc = "XT / 65536 (2^16) clock source selection"]
    #[inline(always)]
    pub fn is_xt_div64k(&self) -> bool {
        *self == Cksel::XtDiv64k
    }
    #[doc = "Uncal LFRC / 16 clock source selection"]
    #[inline(always)]
    pub fn is_ulfrc_div16(&self) -> bool {
        *self == Cksel::UlfrcDiv16
    }
    #[doc = "Uncal LFRC / 128 clock source selection"]
    #[inline(always)]
    pub fn is_ulfrc_div128(&self) -> bool {
        *self == Cksel::UlfrcDiv128
    }
    #[doc = "Uncal LFRC / 1024 clock source selection"]
    #[inline(always)]
    pub fn is_ulfrc_1hz(&self) -> bool {
        *self == Cksel::Ulfrc1hz
    }
    #[doc = "Uncal LFRC / 4096 clock source selection"]
    #[inline(always)]
    pub fn is_ulfrc_div4k(&self) -> bool {
        *self == Cksel::UlfrcDiv4k
    }
    #[doc = "Uncal LFRC / 1048576 (2^20) clock source selection"]
    #[inline(always)]
    pub fn is_ulfrc_div1m(&self) -> bool {
        *self == Cksel::UlfrcDiv1m
    }
    #[doc = "HFRC / 262144 (2^18) clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div256k(&self) -> bool {
        *self == Cksel::HfrcDiv256k
    }
    #[doc = "HFRC / 67108864 (2^26) clock source selection"]
    #[inline(always)]
    pub fn is_hfrc_div64m(&self) -> bool {
        *self == Cksel::HfrcDiv64m
    }
    #[doc = "LFRC / 1048576 (2^20) clock source selection"]
    #[inline(always)]
    pub fn is_lfrc_div1m(&self) -> bool {
        *self == Cksel::LfrcDiv1m
    }
    #[doc = "XT (not autoenabled)"]
    #[inline(always)]
    pub fn is_xtne(&self) -> bool {
        *self == Cksel::Xtne
    }
    #[doc = "XT / 16 (not autoenabled)"]
    #[inline(always)]
    pub fn is_xtne_div16(&self) -> bool {
        *self == Cksel::XtneDiv16
    }
    #[doc = "LFRC / 32 (not autoenabled)"]
    #[inline(always)]
    pub fn is_lfrcne_div32(&self) -> bool {
        *self == Cksel::LfrcneDiv32
    }
    #[doc = "LFRC (not autoenabled) - Default for undefined values"]
    #[inline(always)]
    pub fn is_lfrcne(&self) -> bool {
        *self == Cksel::Lfrcne
    }
    #[doc = "HFRC2 6MHz clock source selection"]
    #[inline(always)]
    pub fn is_hfrc2_6mhz(&self) -> bool {
        *self == Cksel::Hfrc2_6mhz
    }
    #[doc = "HFRC2 24MHz clock source selection"]
    #[inline(always)]
    pub fn is_hfrc2_12mhz(&self) -> bool {
        *self == Cksel::Hfrc2_12mhz
    }
    #[doc = "HFRC2 24MHz clock source selection"]
    #[inline(always)]
    pub fn is_hfrc2_24mhz(&self) -> bool {
        *self == Cksel::Hfrc2_24mhz
    }
}
#[doc = "Field `CKSEL` writer - CLKOUT signal select"]
pub type CkselW<'a, REG> = crate::FieldWriter<'a, REG, 6, Cksel>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFRC clock source selection"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Lfrc)
    }
    #[doc = "XT / 2 clock source selection"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv2)
    }
    #[doc = "XT / 4 clock source selection"]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv4)
    }
    #[doc = "XT / 8 clock source selection"]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv8)
    }
    #[doc = "XT / 16 clock source selection"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv16)
    }
    #[doc = "XT / 32 clock source selection"]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv32)
    }
    #[doc = "1 Hz as selected in RTC"]
    #[inline(always)]
    pub fn rtc_1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Rtc1hz)
    }
    #[doc = "XT / 2097152 (2^21) clock source selection"]
    #[inline(always)]
    pub fn xt_div2m(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv2m)
    }
    #[doc = "XT clock source selection"]
    #[inline(always)]
    pub fn xt(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Xt)
    }
    #[doc = "100 Hz as selected in CLKGEN"]
    #[inline(always)]
    pub fn cg_100hz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Cg100hz)
    }
    #[doc = "HFRC / 2 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv2)
    }
    #[doc = "HFRC / 8 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv8)
    }
    #[doc = "HFRC / 16 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv16)
    }
    #[doc = "HFRC / 32 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv32)
    }
    #[doc = "HFRC / 128 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv128)
    }
    #[doc = "HFRC / 256 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv256)
    }
    #[doc = "HFRC / 512 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv512)
    }
    #[doc = "HFRC / 1024 clock source selection"]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv1024)
    }
    #[doc = "LFRC / 2 clock source selection"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcDiv2)
    }
    #[doc = "LFRC / 32 clock source selection"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcDiv32)
    }
    #[doc = "LFRC / 512 clock source selection"]
    #[inline(always)]
    pub fn lfrc_div512(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcDiv512)
    }
    #[doc = "LFRC / 32768 clock source selection"]
    #[inline(always)]
    pub fn lfrc_div32k(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcDiv32k)
    }
    #[doc = "XT / 256 clock source selection"]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv256)
    }
    #[doc = "XT / 8192 clock source selection"]
    #[inline(always)]
    pub fn xt_div8k(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv8k)
    }
    #[doc = "XT / 65536 (2^16) clock source selection"]
    #[inline(always)]
    pub fn xt_div64k(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtDiv64k)
    }
    #[doc = "Uncal LFRC / 16 clock source selection"]
    #[inline(always)]
    pub fn ulfrc_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::UlfrcDiv16)
    }
    #[doc = "Uncal LFRC / 128 clock source selection"]
    #[inline(always)]
    pub fn ulfrc_div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::UlfrcDiv128)
    }
    #[doc = "Uncal LFRC / 1024 clock source selection"]
    #[inline(always)]
    pub fn ulfrc_1hz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Ulfrc1hz)
    }
    #[doc = "Uncal LFRC / 4096 clock source selection"]
    #[inline(always)]
    pub fn ulfrc_div4k(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::UlfrcDiv4k)
    }
    #[doc = "Uncal LFRC / 1048576 (2^20) clock source selection"]
    #[inline(always)]
    pub fn ulfrc_div1m(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::UlfrcDiv1m)
    }
    #[doc = "HFRC / 262144 (2^18) clock source selection"]
    #[inline(always)]
    pub fn hfrc_div256k(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv256k)
    }
    #[doc = "HFRC / 67108864 (2^26) clock source selection"]
    #[inline(always)]
    pub fn hfrc_div64m(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::HfrcDiv64m)
    }
    #[doc = "LFRC / 1048576 (2^20) clock source selection"]
    #[inline(always)]
    pub fn lfrc_div1m(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcDiv1m)
    }
    #[doc = "XT (not autoenabled)"]
    #[inline(always)]
    pub fn xtne(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Xtne)
    }
    #[doc = "XT / 16 (not autoenabled)"]
    #[inline(always)]
    pub fn xtne_div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::XtneDiv16)
    }
    #[doc = "LFRC / 32 (not autoenabled)"]
    #[inline(always)]
    pub fn lfrcne_div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::LfrcneDiv32)
    }
    #[doc = "LFRC (not autoenabled) - Default for undefined values"]
    #[inline(always)]
    pub fn lfrcne(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Lfrcne)
    }
    #[doc = "HFRC2 6MHz clock source selection"]
    #[inline(always)]
    pub fn hfrc2_6mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Hfrc2_6mhz)
    }
    #[doc = "HFRC2 24MHz clock source selection"]
    #[inline(always)]
    pub fn hfrc2_12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Hfrc2_12mhz)
    }
    #[doc = "HFRC2 24MHz clock source selection"]
    #[inline(always)]
    pub fn hfrc2_24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Hfrc2_24mhz)
    }
}
#[doc = "Enable the CLKOUT signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cken {
    #[doc = "0: Disable CLKOUT"]
    Dis = 0,
    #[doc = "1: Enable CLKOUT"]
    En = 1,
}
impl From<Cken> for bool {
    #[inline(always)]
    fn from(variant: Cken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - Enable the CLKOUT signal"]
pub type CkenR = crate::BitReader<Cken>;
impl CkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cken {
        match self.bits {
            false => Cken::Dis,
            true => Cken::En,
        }
    }
    #[doc = "Disable CLKOUT"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cken::Dis
    }
    #[doc = "Enable CLKOUT"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cken::En
    }
}
#[doc = "Field `CKEN` writer - Enable the CLKOUT signal"]
pub type CkenW<'a, REG> = crate::BitWriter<'a, REG, Cken>;
impl<'a, REG> CkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable CLKOUT"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::Dis)
    }
    #[doc = "Enable CLKOUT"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cken::En)
    }
}
impl R {
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline(always)]
    pub fn cken(&self) -> CkenR {
        CkenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CkselW<ClkoutSpec> {
        CkselW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline(always)]
    #[must_use]
    pub fn cken(&mut self) -> CkenW<ClkoutSpec> {
        CkenW::new(self, 7)
    }
}
#[doc = "This register enables the CLKOUT to the GPIOs, and selects the clock source to that.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutSpec;
impl crate::RegisterSpec for ClkoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkout::R`](R) reader structure"]
impl crate::Readable for ClkoutSpec {}
#[doc = "`write(|w| ..)` method takes [`clkout::W`](W) writer structure"]
impl crate::Writable for ClkoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUT to value 0"]
impl crate::Resettable for ClkoutSpec {
    const RESET_VALUE: u32 = 0;
}
