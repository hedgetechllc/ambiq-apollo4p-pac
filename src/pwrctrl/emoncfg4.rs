#[doc = "Register `EMONCFG4` reader"]
pub type R = crate::R<Emoncfg4Spec>;
#[doc = "Register `EMONCFG4` writer"]
pub type W = crate::W<Emoncfg4Spec>;
#[doc = "Power modes for incrementing the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emonsel4 {
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
impl From<Emonsel4> for u8 {
    #[inline(always)]
    fn from(variant: Emonsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emonsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Emonsel4 {}
#[doc = "Field `EMONSEL4` reader - Power modes for incrementing the counter"]
pub type Emonsel4R = crate::FieldReader<Emonsel4>;
impl Emonsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Emonsel4> {
        match self.bits {
            0 => Some(Emonsel4::Never),
            1 => Some(Emonsel4::Always),
            2 => Some(Emonsel4::Mcusleep),
            3 => Some(Emonsel4::Mcudeepsleep),
            4 => Some(Emonsel4::Dsp0on),
            5 => Some(Emonsel4::Dsp1on),
            6 => Some(Emonsel4::Adcon),
            7 => Some(Emonsel4::Audpbon),
            8 => Some(Emonsel4::Audrecon),
            9 => Some(Emonsel4::I2s0on),
            10 => Some(Emonsel4::I2s1on),
            11 => Some(Emonsel4::Pdm0on),
            12 => Some(Emonsel4::Pdm1on),
            13 => Some(Emonsel4::Pdm2on),
            14 => Some(Emonsel4::Pdm3on),
            15 => Some(Emonsel4::Audadcon),
            16 => Some(Emonsel4::Cryptoon),
            17 => Some(Emonsel4::Dbgon),
            18 => Some(Emonsel4::Dispon),
            19 => Some(Emonsel4::Dispphyon),
            20 => Some(Emonsel4::Dspaon),
            21 => Some(Emonsel4::Gfxon),
            22 => Some(Emonsel4::Uart0on),
            23 => Some(Emonsel4::Uart1on),
            24 => Some(Emonsel4::Uart2on),
            25 => Some(Emonsel4::Uart3on),
            26 => Some(Emonsel4::Iom0on),
            27 => Some(Emonsel4::Iom1on),
            28 => Some(Emonsel4::Iom2on),
            29 => Some(Emonsel4::Iom3on),
            30 => Some(Emonsel4::Reserved30),
            31 => Some(Emonsel4::Reserved31),
            32 => Some(Emonsel4::Iom4on),
            33 => Some(Emonsel4::Iom5on),
            34 => Some(Emonsel4::Iom6on),
            35 => Some(Emonsel4::Iom7on),
            36 => Some(Emonsel4::Ioson),
            37 => Some(Emonsel4::Mspi0on),
            38 => Some(Emonsel4::Mspi1on),
            39 => Some(Emonsel4::Mspi2on),
            40 => Some(Emonsel4::Sdioon),
            41 => Some(Emonsel4::Usbon),
            42 => Some(Emonsel4::Usbphyon),
            _ => None,
        }
    }
    #[doc = "Never increment the counter"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == Emonsel4::Never
    }
    #[doc = "Always increment the counter"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == Emonsel4::Always
    }
    #[doc = "Increment the counter for MCU sleep mode"]
    #[inline(always)]
    pub fn is_mcusleep(&self) -> bool {
        *self == Emonsel4::Mcusleep
    }
    #[doc = "Increment the counter for MCU deepsleep mode"]
    #[inline(always)]
    pub fn is_mcudeepsleep(&self) -> bool {
        *self == Emonsel4::Mcudeepsleep
    }
    #[doc = "Increment the counter for DSP0 active mode"]
    #[inline(always)]
    pub fn is_dsp0on(&self) -> bool {
        *self == Emonsel4::Dsp0on
    }
    #[doc = "Increment the counter for DSP1 active mode"]
    #[inline(always)]
    pub fn is_dsp1on(&self) -> bool {
        *self == Emonsel4::Dsp1on
    }
    #[doc = "Increment the counter when ADC is powered on"]
    #[inline(always)]
    pub fn is_adcon(&self) -> bool {
        *self == Emonsel4::Adcon
    }
    #[doc = "Increment the counter when AUDPB is powered on"]
    #[inline(always)]
    pub fn is_audpbon(&self) -> bool {
        *self == Emonsel4::Audpbon
    }
    #[doc = "Increment the counter when AUDREC is powered on"]
    #[inline(always)]
    pub fn is_audrecon(&self) -> bool {
        *self == Emonsel4::Audrecon
    }
    #[doc = "Increment the counter when I2S0 is powered on"]
    #[inline(always)]
    pub fn is_i2s0on(&self) -> bool {
        *self == Emonsel4::I2s0on
    }
    #[doc = "Increment the counter when I2S1 is powered on"]
    #[inline(always)]
    pub fn is_i2s1on(&self) -> bool {
        *self == Emonsel4::I2s1on
    }
    #[doc = "Increment the counter when PDM0 is powered on"]
    #[inline(always)]
    pub fn is_pdm0on(&self) -> bool {
        *self == Emonsel4::Pdm0on
    }
    #[doc = "Increment the counter when PDM1 is powered on"]
    #[inline(always)]
    pub fn is_pdm1on(&self) -> bool {
        *self == Emonsel4::Pdm1on
    }
    #[doc = "Increment the counter when PDM2 is powered on"]
    #[inline(always)]
    pub fn is_pdm2on(&self) -> bool {
        *self == Emonsel4::Pdm2on
    }
    #[doc = "Increment the counter when PDM3 is powered on"]
    #[inline(always)]
    pub fn is_pdm3on(&self) -> bool {
        *self == Emonsel4::Pdm3on
    }
    #[doc = "Increment the counter when AUDADC is powered on"]
    #[inline(always)]
    pub fn is_audadcon(&self) -> bool {
        *self == Emonsel4::Audadcon
    }
    #[doc = "Increment the counter when CRYPTO is powered on"]
    #[inline(always)]
    pub fn is_cryptoon(&self) -> bool {
        *self == Emonsel4::Cryptoon
    }
    #[doc = "Increment the counter when DBG is powered on"]
    #[inline(always)]
    pub fn is_dbgon(&self) -> bool {
        *self == Emonsel4::Dbgon
    }
    #[doc = "Increment the counter when DISP is powered on"]
    #[inline(always)]
    pub fn is_dispon(&self) -> bool {
        *self == Emonsel4::Dispon
    }
    #[doc = "Increment the counter when DISPPHY is powered on"]
    #[inline(always)]
    pub fn is_dispphyon(&self) -> bool {
        *self == Emonsel4::Dispphyon
    }
    #[doc = "Increment the counter when DSPA is powered on"]
    #[inline(always)]
    pub fn is_dspaon(&self) -> bool {
        *self == Emonsel4::Dspaon
    }
    #[doc = "Increment the counter when GFX is powered on"]
    #[inline(always)]
    pub fn is_gfxon(&self) -> bool {
        *self == Emonsel4::Gfxon
    }
    #[doc = "Increment the counter when UART0 is powered on"]
    #[inline(always)]
    pub fn is_uart0on(&self) -> bool {
        *self == Emonsel4::Uart0on
    }
    #[doc = "Increment the counter when UART1 is powered on"]
    #[inline(always)]
    pub fn is_uart1on(&self) -> bool {
        *self == Emonsel4::Uart1on
    }
    #[doc = "Increment the counter when UART2 is powered on"]
    #[inline(always)]
    pub fn is_uart2on(&self) -> bool {
        *self == Emonsel4::Uart2on
    }
    #[doc = "Increment the counter when UART3 is powered on"]
    #[inline(always)]
    pub fn is_uart3on(&self) -> bool {
        *self == Emonsel4::Uart3on
    }
    #[doc = "Increment the counter when IOM0 is powered on"]
    #[inline(always)]
    pub fn is_iom0on(&self) -> bool {
        *self == Emonsel4::Iom0on
    }
    #[doc = "Increment the counter when IOM1 is powered on"]
    #[inline(always)]
    pub fn is_iom1on(&self) -> bool {
        *self == Emonsel4::Iom1on
    }
    #[doc = "Increment the counter when IOM2 is powered on"]
    #[inline(always)]
    pub fn is_iom2on(&self) -> bool {
        *self == Emonsel4::Iom2on
    }
    #[doc = "Increment the counter when IOM3 is powered on"]
    #[inline(always)]
    pub fn is_iom3on(&self) -> bool {
        *self == Emonsel4::Iom3on
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved30(&self) -> bool {
        *self == Emonsel4::Reserved30
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn is_reserved31(&self) -> bool {
        *self == Emonsel4::Reserved31
    }
    #[doc = "Increment the counter when IOM4 is powered on"]
    #[inline(always)]
    pub fn is_iom4on(&self) -> bool {
        *self == Emonsel4::Iom4on
    }
    #[doc = "Increment the counter when IOM5 is powered on"]
    #[inline(always)]
    pub fn is_iom5on(&self) -> bool {
        *self == Emonsel4::Iom5on
    }
    #[doc = "Increment the counter when IOM6 is powered on"]
    #[inline(always)]
    pub fn is_iom6on(&self) -> bool {
        *self == Emonsel4::Iom6on
    }
    #[doc = "Increment the counter when IOM7 is powered on"]
    #[inline(always)]
    pub fn is_iom7on(&self) -> bool {
        *self == Emonsel4::Iom7on
    }
    #[doc = "Increment the counter when IOS is powered on"]
    #[inline(always)]
    pub fn is_ioson(&self) -> bool {
        *self == Emonsel4::Ioson
    }
    #[doc = "Increment the counter when MSPI0 is powered on"]
    #[inline(always)]
    pub fn is_mspi0on(&self) -> bool {
        *self == Emonsel4::Mspi0on
    }
    #[doc = "Increment the counter when MSPI1 is powered on"]
    #[inline(always)]
    pub fn is_mspi1on(&self) -> bool {
        *self == Emonsel4::Mspi1on
    }
    #[doc = "Increment the counter when MSPI2 is powered on"]
    #[inline(always)]
    pub fn is_mspi2on(&self) -> bool {
        *self == Emonsel4::Mspi2on
    }
    #[doc = "Increment the counter when SDIO is powered on"]
    #[inline(always)]
    pub fn is_sdioon(&self) -> bool {
        *self == Emonsel4::Sdioon
    }
    #[doc = "Increment the counter when USB is powered on"]
    #[inline(always)]
    pub fn is_usbon(&self) -> bool {
        *self == Emonsel4::Usbon
    }
    #[doc = "Increment the counter when USBPHY is powered on"]
    #[inline(always)]
    pub fn is_usbphyon(&self) -> bool {
        *self == Emonsel4::Usbphyon
    }
}
#[doc = "Field `EMONSEL4` writer - Power modes for incrementing the counter"]
pub type Emonsel4W<'a, REG> = crate::FieldWriter<'a, REG, 8, Emonsel4>;
impl<'a, REG> Emonsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Never increment the counter"]
    #[inline(always)]
    pub fn never(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Never)
    }
    #[doc = "Always increment the counter"]
    #[inline(always)]
    pub fn always(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Always)
    }
    #[doc = "Increment the counter for MCU sleep mode"]
    #[inline(always)]
    pub fn mcusleep(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Mcusleep)
    }
    #[doc = "Increment the counter for MCU deepsleep mode"]
    #[inline(always)]
    pub fn mcudeepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Mcudeepsleep)
    }
    #[doc = "Increment the counter for DSP0 active mode"]
    #[inline(always)]
    pub fn dsp0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dsp0on)
    }
    #[doc = "Increment the counter for DSP1 active mode"]
    #[inline(always)]
    pub fn dsp1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dsp1on)
    }
    #[doc = "Increment the counter when ADC is powered on"]
    #[inline(always)]
    pub fn adcon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Adcon)
    }
    #[doc = "Increment the counter when AUDPB is powered on"]
    #[inline(always)]
    pub fn audpbon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Audpbon)
    }
    #[doc = "Increment the counter when AUDREC is powered on"]
    #[inline(always)]
    pub fn audrecon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Audrecon)
    }
    #[doc = "Increment the counter when I2S0 is powered on"]
    #[inline(always)]
    pub fn i2s0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::I2s0on)
    }
    #[doc = "Increment the counter when I2S1 is powered on"]
    #[inline(always)]
    pub fn i2s1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::I2s1on)
    }
    #[doc = "Increment the counter when PDM0 is powered on"]
    #[inline(always)]
    pub fn pdm0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Pdm0on)
    }
    #[doc = "Increment the counter when PDM1 is powered on"]
    #[inline(always)]
    pub fn pdm1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Pdm1on)
    }
    #[doc = "Increment the counter when PDM2 is powered on"]
    #[inline(always)]
    pub fn pdm2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Pdm2on)
    }
    #[doc = "Increment the counter when PDM3 is powered on"]
    #[inline(always)]
    pub fn pdm3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Pdm3on)
    }
    #[doc = "Increment the counter when AUDADC is powered on"]
    #[inline(always)]
    pub fn audadcon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Audadcon)
    }
    #[doc = "Increment the counter when CRYPTO is powered on"]
    #[inline(always)]
    pub fn cryptoon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Cryptoon)
    }
    #[doc = "Increment the counter when DBG is powered on"]
    #[inline(always)]
    pub fn dbgon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dbgon)
    }
    #[doc = "Increment the counter when DISP is powered on"]
    #[inline(always)]
    pub fn dispon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dispon)
    }
    #[doc = "Increment the counter when DISPPHY is powered on"]
    #[inline(always)]
    pub fn dispphyon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dispphyon)
    }
    #[doc = "Increment the counter when DSPA is powered on"]
    #[inline(always)]
    pub fn dspaon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Dspaon)
    }
    #[doc = "Increment the counter when GFX is powered on"]
    #[inline(always)]
    pub fn gfxon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Gfxon)
    }
    #[doc = "Increment the counter when UART0 is powered on"]
    #[inline(always)]
    pub fn uart0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Uart0on)
    }
    #[doc = "Increment the counter when UART1 is powered on"]
    #[inline(always)]
    pub fn uart1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Uart1on)
    }
    #[doc = "Increment the counter when UART2 is powered on"]
    #[inline(always)]
    pub fn uart2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Uart2on)
    }
    #[doc = "Increment the counter when UART3 is powered on"]
    #[inline(always)]
    pub fn uart3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Uart3on)
    }
    #[doc = "Increment the counter when IOM0 is powered on"]
    #[inline(always)]
    pub fn iom0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom0on)
    }
    #[doc = "Increment the counter when IOM1 is powered on"]
    #[inline(always)]
    pub fn iom1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom1on)
    }
    #[doc = "Increment the counter when IOM2 is powered on"]
    #[inline(always)]
    pub fn iom2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom2on)
    }
    #[doc = "Increment the counter when IOM3 is powered on"]
    #[inline(always)]
    pub fn iom3on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom3on)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved30(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Reserved30)
    }
    #[doc = "Reserved selection. Operation unknown if selected."]
    #[inline(always)]
    pub fn reserved31(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Reserved31)
    }
    #[doc = "Increment the counter when IOM4 is powered on"]
    #[inline(always)]
    pub fn iom4on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom4on)
    }
    #[doc = "Increment the counter when IOM5 is powered on"]
    #[inline(always)]
    pub fn iom5on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom5on)
    }
    #[doc = "Increment the counter when IOM6 is powered on"]
    #[inline(always)]
    pub fn iom6on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom6on)
    }
    #[doc = "Increment the counter when IOM7 is powered on"]
    #[inline(always)]
    pub fn iom7on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Iom7on)
    }
    #[doc = "Increment the counter when IOS is powered on"]
    #[inline(always)]
    pub fn ioson(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Ioson)
    }
    #[doc = "Increment the counter when MSPI0 is powered on"]
    #[inline(always)]
    pub fn mspi0on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Mspi0on)
    }
    #[doc = "Increment the counter when MSPI1 is powered on"]
    #[inline(always)]
    pub fn mspi1on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Mspi1on)
    }
    #[doc = "Increment the counter when MSPI2 is powered on"]
    #[inline(always)]
    pub fn mspi2on(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Mspi2on)
    }
    #[doc = "Increment the counter when SDIO is powered on"]
    #[inline(always)]
    pub fn sdioon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Sdioon)
    }
    #[doc = "Increment the counter when USB is powered on"]
    #[inline(always)]
    pub fn usbon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Usbon)
    }
    #[doc = "Increment the counter when USBPHY is powered on"]
    #[inline(always)]
    pub fn usbphyon(self) -> &'a mut crate::W<REG> {
        self.variant(Emonsel4::Usbphyon)
    }
}
impl R {
    #[doc = "Bits 0:7 - Power modes for incrementing the counter"]
    #[inline(always)]
    pub fn emonsel4(&self) -> Emonsel4R {
        Emonsel4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Power modes for incrementing the counter"]
    #[inline(always)]
    #[must_use]
    pub fn emonsel4(&mut self) -> Emonsel4W<Emoncfg4Spec> {
        Emonsel4W::new(self, 0)
    }
}
#[doc = "The counter increments when the counter is enabled and the mode selected here matches the power mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`emoncfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emoncfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emoncfg4Spec;
impl crate::RegisterSpec for Emoncfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emoncfg4::R`](R) reader structure"]
impl crate::Readable for Emoncfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`emoncfg4::W`](W) writer structure"]
impl crate::Writable for Emoncfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONCFG4 to value 0"]
impl crate::Resettable for Emoncfg4Spec {
    const RESET_VALUE: u32 = 0;
}
