#[doc = "Register `EMONCFG6` reader"]
pub type R = crate::R<Emoncfg6Spec>;
#[doc = "Register `EMONCFG6` writer"]
pub type W = crate::W<Emoncfg6Spec>;
#[doc = "Power modes for incrementing the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emonsel6 {
    #[doc = "0: Never increment the counter"]
    Never = 0,
    #[doc = "1: Always increment the counter"]
    Always = 1,
    #[doc = "2: Increment the counter for MCU sleep mode"]
    Mcusleep = 2,
    #[doc = "3: Increment the counter for MCU deepsleep mode"]
    Mcudeepsleep = 3,
    #[doc = "4: Increment the counter for DSP0 active mode"]
    Dsp0on = 4,
    #[doc = "5: Increment the counter for DSP1 active mode"]
    Dsp1on = 5,
    #[doc = "6: Increment the counter when ADC is powered on"]
    Adcon = 6,
    #[doc = "7: Increment the counter when AUDPB is powered on"]
    Audpbon = 7,
    #[doc = "8: Increment the counter when AUDREC is powered on"]
    Audrecon = 8,
    #[doc = "9: Increment the counter when I2S0 is powered on"]
    I2s0on = 9,
    #[doc = "10: Increment the counter when I2S1 is powered on"]
    I2s1on = 10,
    #[doc = "11: Increment the counter when PDM0 is powered on"]
    Pdm0on = 11,
    #[doc = "12: Increment the counter when PDM1 is powered on"]
    Pdm1on = 12,
    #[doc = "13: Increment the counter when PDM2 is powered on"]
    Pdm2on = 13,
    #[doc = "14: Increment the counter when PDM3 is powered on"]
    Pdm3on = 14,
    #[doc = "15: Increment the counter when AUDADC is powered on"]
    Audadcon = 15,
    #[doc = "16: Increment the counter when CRYPTO is powered on"]
    Cryptoon = 16,
    #[doc = "17: Increment the counter when DBG is powered on"]
    Dbgon = 17,
    #[doc = "18: Increment the counter when DISP is powered on"]
    Dispon = 18,
    #[doc = "19: Increment the counter when DISPPHY is powered on"]
    Dispphyon = 19,
    #[doc = "20: Increment the counter when DSPA is powered on"]
    Dspaon = 20,
    #[doc = "21: Increment the counter when GFX is powered on"]
    Gfxon = 21,
    #[doc = "22: Increment the counter when UART0 is powered on"]
    Uart0on = 22,
    #[doc = "23: Increment the counter when UART1 is powered on"]
    Uart1on = 23,
    #[doc = "24: Increment the counter when UART2 is powered on"]
    Uart2on = 24,
    #[doc = "25: Increment the counter when UART3 is powered on"]
    Uart3on = 25,
    #[doc = "26: Increment the counter when IOM0 is powered on"]
    Iom0on = 26,
    #[doc = "27: Increment the counter when IOM1 is powered on"]
    Iom1on = 27,
    #[doc = "28: Increment the counter when IOM2 is powered on"]
    Iom2on = 28,
    #[doc = "29: Increment the counter when IOM3 is powered on"]
    Iom3on = 29,
    #[doc = "30: Reserved selection. Operation unknown if selected."]
    Reserved30 = 30,
    #[doc = "31: Reserved selection. Operation unknown if selected."]
    Reserved31 = 31,
    #[doc = "32: Increment the counter when IOM4 is powered on"]
    Iom4on = 32,
    #[doc = "33: Increment the counter when IOM5 is powered on"]
    Iom5on = 33,
    #[doc = "34: Increment the counter when IOM6 is powered on"]
    Iom6on = 34,
    #[doc = "35: Increment the counter when IOM7 is powered on"]
    Iom7on = 35,
    #[doc = "36: Increment the counter when IOS is powered on"]
    Ioson = 36,
    #[doc = "37: Increment the counter when MSPI0 is powered on"]
    Mspi0on = 37,
    #[doc = "38: Increment the counter when MSPI1 is powered on"]
    Mspi1on = 38,
    #[doc = "39: Increment the counter when MSPI2 is powered on"]
    Mspi2on = 39,
    #[doc = "40: Increment the counter when SDIO is powered on"]
    Sdioon = 40,
    #[doc = "41: Increment the counter when USB is powered on"]
    Usbon = 41,
    #[doc = "42: Increment the counter when USBPHY is powered on"]
    Usbphyon = 42,
}
impl From<Emonsel6> for u8 {
    #[inline(always)]
    fn from(variant: Emonsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emonsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Emonsel6 {}
#[doc = "Field `EMONSEL6` reader - Power modes for incrementing the counter"]
pub type Emonsel6R = crate::FieldReader<Emonsel6>;
impl Emonsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Emonsel6> {
        match self.bits {
            0 => Some(Emonsel6::Never),
            1 => Some(Emonsel6::Always),
            2 => Some(Emonsel6::Mcusleep),
            3 => Some(Emonsel6::Mcudeepsleep),
            4 => Some(Emonsel6::Dsp0on),
            5 => Some(Emonsel6::Dsp1on),
            6 => Some(Emonsel6::Adcon),
            7 => Some(Emonsel6::Audpbon),
            8 => Some(Emonsel6::Audrecon),
            9 => Some(Emonsel6::I2s0on),
            10 => Some(Emonsel6::I2s1on),
            11 => Some(Emonsel6::Pdm0on),
            12 => Some(Emonsel6::Pdm1on),
            13 => Some(Emonsel6::Pdm2on),
            14 => Some(Emonsel6::Pdm3on),
            15 => Some(Emonsel6::Audadcon),
            16 => Some(Emonsel6::Cryptoon),
            17 => Some(Emonsel6::Dbgon),
            18 => Some(Emonsel6::Dispon),
            19 => Some(Emonsel6::Dispphyon),
            20 => Some(Emonsel6::Dspaon),
            21 => Some(Emonsel6::Gfxon),
            22 => Some(Emonsel6::Uart0on),
            23 => Some(Emonsel6::Uart1on),
            24 => Some(Emonsel6::Uart2on),
            25 => Some(Emonsel6::Uart3on),
            26 => Some(Emonsel6::Iom0on),
            27 => Some(Emonsel6::Iom1on),
            28 => Some(Emonsel6::Iom2on),
            29 => Some(Emonsel6::Iom3on),
            30 => Some(Emonsel6::Reserved30),
            31 => Some(Emonsel6::Reserved31),
            32 => Some(Emonsel6::Iom4on),
            33 => Some(Emonsel6::Iom5on),
            34 => Some(Emonsel6::Iom6on),
            35 => Some(Emonsel6::Iom7on),
            36 => Some(Emonsel6::Ioson),
            37 => Some(Emonsel6::Mspi0on),
            38 => Some(Emonsel6::Mspi1on),
            39 => Some(Emonsel6::Mspi2on),
            40 => Some(Emonsel6::Sdioon),
            41 => Some(Emonsel6::Usbon),
            42 => Some(Emonsel6::Usbphyon),
            _ => None,
        }
    }
    #[doc = "Never increment the counter"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == Emonsel6::Never
    }
    #[doc = "Always increment the counter"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == Emonsel6::Always
    }
    #[doc = "Increment the counter for MCU sleep mode"]
    #[inline(always)]
    pub fn is_mcusleep(&self) -> bool {
        *self == Emonsel6::Mcusleep
    }
    #[doc = "Increment the counter for MCU deepsleep mode"]
    #[inline(always)]
    pub fn is_mcudeepsleep(&self) -> bool {
        *self == Emonsel6::Mcudeepsleep
    }
    #[doc = "Increment the counter for DSP0 active mode"]
    #[inline(always)]
    pub fn is_dsp0on(&self) -> bool {
        *self == Emonsel6::Dsp0on
    }
    #[doc = "Increment the counter for DSP1 active mode"]
    #[inline(always)]
    pub fn is_dsp1on(&self) -> bool {
        *self == Emonsel6::Dsp1on
    }
    #[doc = "Increment the counter when ADC is powered on"]
    #[inline(always)]
    pub fn is_adcon(&self) -> bool {
        *self == Emonsel6::Adcon
    }
    #[doc = "Increment the counter when AUDPB is powered on"]
    #[inline(always)]
    pub fn is_audpbon(&self) -> bool {
        *self == Emonsel6::Audpbon
    }
    #[doc = "Increment the counter when AUDREC is powered on"]
    #[inline(always)]
    pub fn is_audrecon(&self) -> bool {
        *self == Emonsel6::Audrecon
    }
    #[doc = "Increment the counter when I2S0 is powered on"]
    #[inline(always)]
    pub fn is_i2s0on(&self) -> bool {
        *self == Emonsel6::I2s0on
    }
    #[doc = "Increment the counter when I2S1 is powered on"]
    #[inline(always)]
    pub fn is_i2s1on(&self) -> bool {
        *self == Emonsel6::I2s1on
    }
    #[doc = "Increment the counter when PDM0 is powered on"]
    #[inline(always)]
    pub fn is_pdm0on(&self) -> bool {
        *self == Emonsel6::Pdm0on
    }
    #[doc = "Increment the counter when PDM1 is powered on"]
    #[inline(always)]
    pub fn is_pdm1on(&self) -> bool {
        *self == Emonsel6::Pdm1on
    }
    #[doc = "Increment the counter when PDM2 is powered on"]
    #[inline(always)]
    pub fn is_pdm2on(&self) -> bool {
        *self == Emonsel6::Pdm2on
    }
    #[doc = "Increment the counter when PDM3 is powered on"]
    #[inline(always)]
    pub fn is_pdm3on(&self) -> bool {
        *self == Emonsel6::Pdm3on
    }
    #[doc = "Increment the counter when AUDADC is powered on"]
    #[inline(always)]
    pub fn is_audadcon(&self) -> bool {
        *self == Emonsel6::Audadcon
    }
    #[doc = "Increment the counter when CRYPTO is powered on"]
    #[inline(always)]
    pub fn is_cryptoon(&self) -> bool {
        *self == Emonsel6::Cryptoon
    }
    #[doc = "Increment the counter when DBG is powered on"]
    #[inline(always)]
    pub fn is_dbgon(&self) -> bool {
        *self == Emonsel6::Dbgon
    }
    #[doc = "Increment the counter when DISP is powered on"]
    #[inline(always)]
    pub fn is_dispon(&self) -> bool {
        *self == Emonsel6::Dispon
    }
    #[doc = "Increment the counter when DISPPHY is powered on"]
    #[inline(always)]
    pub fn is_dispphyon(&self) -> bool {
        *self == Emonsel6::Dispphyon
    }
    #[doc = "Increment the counter when DSPA is powered on"]
    #[inline(always)]
    pub fn is_dspaon(&self) -> bool {
        *self == Emonsel6::Dspaon
    }
    #[doc = "Increment the counter when GFX is powered on"]
    #[inline(always)]
    pub fn is_gfxon(&self) -> bool {
        *self == Emonsel6::Gfxon
    }
    #[doc = "Increment the counter when UART0 is powered on"]
    #[inline(always)]
    pub fn is_uart0on(&self) -> bool {
        *self == Emonsel6::Uart0on
    }
    #[doc = "Increment the counter when UART1 is powered on"]
    #[inline(always)]
    pub fn is_uart1on(&self) -> bool {
        *self == Emonsel6::Uart1on
    }
    #[doc = "Increment the counter when UART2 is powered on"]
    #[inline(always)]
    pub fn is_uart2on(&self) -> bool {
        *self == Emonsel6::Uart2on
    }
    #[doc = "Increment the counter when UART3 is powered on"]
    #[inline(always)]
    pub fn is_uart3on(&self) -> bool {
        *self == Emonsel6::Uart3on
    }
    #[doc = "Increment the counter when IOM0 is powered on"]
    #[inline(always)]
    pub fn is_iom0on(&self) -> bool {
        *self == Emonsel6::Iom0on
    }
    #[doc = "Increment the counter when IOM1 is powered on"]
    #[inline(always)]
    pub fn is_iom1on(&self) -> bool {
        *self == Emonsel6::Iom1on
    }
    #[doc = "Increment the counter when IOM2 is powered on"]
    #[inline(always)]
    pub fn is_iom2on(&self) -> bool {
        *self == Emonsel6::Iom2on
    }
    #[doc = "Increment the counter when IOM3 is powered on"]
    #[inline(always)]
    pub fn is_iom3on(&self) -> bool {
        *self == Emonsel6::Iom3on
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved30(&self) -> bool {
        *self == Emonsel6::Reserved30
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved31(&self) -> bool {
        *self == Emonsel6::Reserved31
    }
    #[doc = "Increment the counter when IOM4 is powered on"]
    #[inline(always)]
    pub fn is_iom4on(&self) -> bool {
        *self == Emonsel6::Iom4on
    }
    #[doc = "Increment the counter when IOM5 is powered on"]
    #[inline(always)]
    pub fn is_iom5on(&self) -> bool {
        *self == Emonsel6::Iom5on
    }
    #[doc = "Increment the counter when IOM6 is powered on"]
    #[inline(always)]
    pub fn is_iom6on(&self) -> bool {
        *self == Emonsel6::Iom6on
    }
    #[doc = "Increment the counter when IOM7 is powered on"]
    #[inline(always)]
    pub fn is_iom7on(&self) -> bool {
        *self == Emonsel6::Iom7on
    }
    #[doc = "Increment the counter when IOS is powered on"]
    #[inline(always)]
    pub fn is_ioson(&self) -> bool {
        *self == Emonsel6::Ioson
    }
    #[doc = "Increment the counter when MSPI0 is powered on"]
    #[inline(always)]
    pub fn is_mspi0on(&self) -> bool {
        *self == Emonsel6::Mspi0on
    }
    #[doc = "Increment the counter when MSPI1 is powered on"]
    #[inline(always)]
    pub fn is_mspi1on(&self) -> bool {
        *self == Emonsel6::Mspi1on
    }
    #[doc = "Increment the counter when MSPI2 is powered on"]
    #[inline(always)]
    pub fn is_mspi2on(&self) -> bool {
        *self == Emonsel6::Mspi2on
    }
    #[doc = "Increment the counter when SDIO is powered on"]
    #[inline(always)]
    pub fn is_sdioon(&self) -> bool {
        *self == Emonsel6::Sdioon
    }
    #[doc = "Increment the counter when USB is powered on"]
    #[inline(always)]
    pub fn is_usbon(&self) -> bool {
        *self == Emonsel6::Usbon
    }
    #[doc = "Increment the counter when USBPHY is powered on"]
    #[inline(always)]
    pub fn is_usbphyon(&self) -> bool {
        *self == Emonsel6::Usbphyon
    }
}
#[doc = "Field `EMONSEL6` writer - Power modes for incrementing the counter"]
pub type Emonsel6W<'a, REG> = crate::FieldWriter<'a, REG, 8, Emonsel6>;
impl<'a, REG> Emonsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never increment the counter"]
    #[inline(always)]
    pub fn never(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Never)
    }
    #[doc = "Always increment the counter"]
    #[inline(always)]
    pub fn always(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Always)
    }
    #[doc = "Increment the counter for MCU sleep mode"]
    #[inline(always)]
    pub fn mcusleep(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Mcusleep)
    }
    #[doc = "Increment the counter for MCU deepsleep mode"]
    #[inline(always)]
    pub fn mcudeepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Mcudeepsleep)
    }
    #[doc = "Increment the counter for DSP0 active mode"]
    #[inline(always)]
    pub fn dsp0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dsp0on)
    }
    #[doc = "Increment the counter for DSP1 active mode"]
    #[inline(always)]
    pub fn dsp1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dsp1on)
    }
    #[doc = "Increment the counter when ADC is powered on"]
    #[inline(always)]
    pub fn adcon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Adcon)
    }
    #[doc = "Increment the counter when AUDPB is powered on"]
    #[inline(always)]
    pub fn audpbon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Audpbon)
    }
    #[doc = "Increment the counter when AUDREC is powered on"]
    #[inline(always)]
    pub fn audrecon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Audrecon)
    }
    #[doc = "Increment the counter when I2S0 is powered on"]
    #[inline(always)]
    pub fn i2s0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::I2s0on)
    }
    #[doc = "Increment the counter when I2S1 is powered on"]
    #[inline(always)]
    pub fn i2s1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::I2s1on)
    }
    #[doc = "Increment the counter when PDM0 is powered on"]
    #[inline(always)]
    pub fn pdm0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Pdm0on)
    }
    #[doc = "Increment the counter when PDM1 is powered on"]
    #[inline(always)]
    pub fn pdm1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Pdm1on)
    }
    #[doc = "Increment the counter when PDM2 is powered on"]
    #[inline(always)]
    pub fn pdm2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Pdm2on)
    }
    #[doc = "Increment the counter when PDM3 is powered on"]
    #[inline(always)]
    pub fn pdm3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Pdm3on)
    }
    #[doc = "Increment the counter when AUDADC is powered on"]
    #[inline(always)]
    pub fn audadcon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Audadcon)
    }
    #[doc = "Increment the counter when CRYPTO is powered on"]
    #[inline(always)]
    pub fn cryptoon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Cryptoon)
    }
    #[doc = "Increment the counter when DBG is powered on"]
    #[inline(always)]
    pub fn dbgon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dbgon)
    }
    #[doc = "Increment the counter when DISP is powered on"]
    #[inline(always)]
    pub fn dispon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dispon)
    }
    #[doc = "Increment the counter when DISPPHY is powered on"]
    #[inline(always)]
    pub fn dispphyon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dispphyon)
    }
    #[doc = "Increment the counter when DSPA is powered on"]
    #[inline(always)]
    pub fn dspaon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Dspaon)
    }
    #[doc = "Increment the counter when GFX is powered on"]
    #[inline(always)]
    pub fn gfxon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Gfxon)
    }
    #[doc = "Increment the counter when UART0 is powered on"]
    #[inline(always)]
    pub fn uart0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Uart0on)
    }
    #[doc = "Increment the counter when UART1 is powered on"]
    #[inline(always)]
    pub fn uart1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Uart1on)
    }
    #[doc = "Increment the counter when UART2 is powered on"]
    #[inline(always)]
    pub fn uart2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Uart2on)
    }
    #[doc = "Increment the counter when UART3 is powered on"]
    #[inline(always)]
    pub fn uart3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Uart3on)
    }
    #[doc = "Increment the counter when IOM0 is powered on"]
    #[inline(always)]
    pub fn iom0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom0on)
    }
    #[doc = "Increment the counter when IOM1 is powered on"]
    #[inline(always)]
    pub fn iom1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom1on)
    }
    #[doc = "Increment the counter when IOM2 is powered on"]
    #[inline(always)]
    pub fn iom2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom2on)
    }
    #[doc = "Increment the counter when IOM3 is powered on"]
    #[inline(always)]
    pub fn iom3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom3on)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved30(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Reserved30)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved31(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Reserved31)
    }
    #[doc = "Increment the counter when IOM4 is powered on"]
    #[inline(always)]
    pub fn iom4on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom4on)
    }
    #[doc = "Increment the counter when IOM5 is powered on"]
    #[inline(always)]
    pub fn iom5on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom5on)
    }
    #[doc = "Increment the counter when IOM6 is powered on"]
    #[inline(always)]
    pub fn iom6on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom6on)
    }
    #[doc = "Increment the counter when IOM7 is powered on"]
    #[inline(always)]
    pub fn iom7on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Iom7on)
    }
    #[doc = "Increment the counter when IOS is powered on"]
    #[inline(always)]
    pub fn ioson(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Ioson)
    }
    #[doc = "Increment the counter when MSPI0 is powered on"]
    #[inline(always)]
    pub fn mspi0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Mspi0on)
    }
    #[doc = "Increment the counter when MSPI1 is powered on"]
    #[inline(always)]
    pub fn mspi1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Mspi1on)
    }
    #[doc = "Increment the counter when MSPI2 is powered on"]
    #[inline(always)]
    pub fn mspi2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Mspi2on)
    }
    #[doc = "Increment the counter when SDIO is powered on"]
    #[inline(always)]
    pub fn sdioon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Sdioon)
    }
    #[doc = "Increment the counter when USB is powered on"]
    #[inline(always)]
    pub fn usbon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Usbon)
    }
    #[doc = "Increment the counter when USBPHY is powered on"]
    #[inline(always)]
    pub fn usbphyon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel6::Usbphyon)
    }
}
impl R {
    #[doc = "Bits 0:7 - Power modes for incrementing the counter"]
    #[inline(always)]
    pub fn emonsel6(&self) -> Emonsel6R {
        Emonsel6R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Power modes for incrementing the counter"]
    #[inline(always)]
    #[must_use]
    pub fn emonsel6(&mut self) -> Emonsel6W<Emoncfg6Spec> {
        Emonsel6W::new(self, 0)
    }
}
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncfg6Spec;
impl crate::RegisterSpec for Emoncfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncfg6::R`](R) reader structure"]
impl crate::Readable for Emoncfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncfg6::W`](W) writer structure"]
impl crate::Writable for Emoncfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCFG6 to value 0"]
impl crate::Resettable for Emoncfg6Spec {
    const RESET_VALUE: u32 = 0;
}
