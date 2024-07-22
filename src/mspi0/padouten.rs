#[doc = "Register `PADOUTEN` reader"]
pub type R = crate::R<PadoutenSpec>;
#[doc = "Register `PADOUTEN` writer"]
pub type W = crate::W<PadoutenSpec>;
#[doc = "Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data. Bits \\[7:4\\]
are Quad1 data. Bit \\[8\\]
is clock. Bit \\[9\\]
is BM/DQS. Bit \\[10:17\\]
are data for 16-bit. Bit\\[18\\]
is BM/DQS for 16-bit. Bit\\[19\\]
is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Outen {
    #[doc = "271: Quad0 (4 data + 1 clock)"]
    Quad0 = 271,
    #[doc = "496: Quad1 (4 data + 1 clock)"]
    Quad1 = 496,
    #[doc = "1023: Octal (8 data + 1 clock)"]
    Octal = 1023,
    #[doc = "259: Serial (2 data + 1 clock)"]
    Serial0 = 259,
    #[doc = "304: Serial (2 data + 1 clock)"]
    Serial1 = 304,
    #[doc = "524287: Octal (16 data + 1 clock + 2 bytemask)"]
    Hex = 524287,
}
impl From<Outen> for u32 {
    #[inline(always)]
    fn from(variant: Outen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outen {
    type Ux = u32;
}
impl crate::IsEnum for Outen {}
#[doc = "Field `OUTEN` reader - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data. Bits \\[7:4\\]
are Quad1 data. Bit \\[8\\]
is clock. Bit \\[9\\]
is BM/DQS. Bit \\[10:17\\]
are data for 16-bit. Bit\\[18\\]
is BM/DQS for 16-bit. Bit\\[19\\]
is not used."]
pub type OutenR = crate::FieldReader<Outen>;
impl OutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Outen> {
        match self.bits {
            271 => Some(Outen::Quad0),
            496 => Some(Outen::Quad1),
            1023 => Some(Outen::Octal),
            259 => Some(Outen::Serial0),
            304 => Some(Outen::Serial1),
            524287 => Some(Outen::Hex),
            _ => None,
        }
    }
    #[doc = "Quad0 (4 data + 1 clock)"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        *self == Outen::Quad0
    }
    #[doc = "Quad1 (4 data + 1 clock)"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        *self == Outen::Quad1
    }
    #[doc = "Octal (8 data + 1 clock)"]
    #[inline(always)]
    pub fn is_octal(&self) -> bool {
        *self == Outen::Octal
    }
    #[doc = "Serial (2 data + 1 clock)"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        *self == Outen::Serial0
    }
    #[doc = "Serial (2 data + 1 clock)"]
    #[inline(always)]
    pub fn is_serial1(&self) -> bool {
        *self == Outen::Serial1
    }
    #[doc = "Octal (16 data + 1 clock + 2 bytemask)"]
    #[inline(always)]
    pub fn is_hex(&self) -> bool {
        *self == Outen::Hex
    }
}
#[doc = "Field `OUTEN` writer - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data. Bits \\[7:4\\]
are Quad1 data. Bit \\[8\\]
is clock. Bit \\[9\\]
is BM/DQS. Bit \\[10:17\\]
are data for 16-bit. Bit\\[18\\]
is BM/DQS for 16-bit. Bit\\[19\\]
is not used."]
pub type OutenW<'a, REG> = crate::FieldWriter<'a, REG, 20, Outen>;
impl<'a, REG> OutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Quad0 (4 data + 1 clock)"]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Quad0)
    }
    #[doc = "Quad1 (4 data + 1 clock)"]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Quad1)
    }
    #[doc = "Octal (8 data + 1 clock)"]
    #[inline(always)]
    pub fn octal(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Octal)
    }
    #[doc = "Serial (2 data + 1 clock)"]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Serial0)
    }
    #[doc = "Serial (2 data + 1 clock)"]
    #[inline(always)]
    pub fn serial1(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Serial1)
    }
    #[doc = "Octal (16 data + 1 clock + 2 bytemask)"]
    #[inline(always)]
    pub fn hex(self) -> &'a mut crate::W<REG> {
        self.variant(Outen::Hex)
    }
}
#[doc = "Field `PADSET1` reader - Only applicable on mspi1. When set, use gpio95 .. gpio 104 as the pads. This extra set of pads is to run mspi0 on HEX and mspi1 concurrently. Note that the timing of this extra pads may not be as good as the origianl pads."]
pub type Padset1R = crate::BitReader;
#[doc = "Field `PADSET1` writer - Only applicable on mspi1. When set, use gpio95 .. gpio 104 as the pads. This extra set of pads is to run mspi0 on HEX and mspi1 concurrently. Note that the timing of this extra pads may not be as good as the origianl pads."]
pub type Padset1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOND4` reader - Output clock on MSPI data\\[4\\]"]
pub type Clkond4R = crate::BitReader;
#[doc = "Field `CLKOND4` writer - Output clock on MSPI data\\[4\\]"]
pub type Clkond4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data. Bits \\[7:4\\]
are Quad1 data. Bit \\[8\\]
is clock. Bit \\[9\\]
is BM/DQS. Bit \\[10:17\\]
are data for 16-bit. Bit\\[18\\]
is BM/DQS for 16-bit. Bit\\[19\\]
is not used."]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30 - Only applicable on mspi1. When set, use gpio95 .. gpio 104 as the pads. This extra set of pads is to run mspi0 on HEX and mspi1 concurrently. Note that the timing of this extra pads may not be as good as the origianl pads."]
    #[inline(always)]
    pub fn padset1(&self) -> Padset1R {
        Padset1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output clock on MSPI data\\[4\\]"]
    #[inline(always)]
    pub fn clkond4(&self) -> Clkond4R {
        Clkond4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\]
are Quad0 data. Bits \\[7:4\\]
are Quad1 data. Bit \\[8\\]
is clock. Bit \\[9\\]
is BM/DQS. Bit \\[10:17\\]
are data for 16-bit. Bit\\[18\\]
is BM/DQS for 16-bit. Bit\\[19\\]
is not used."]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OutenW<PadoutenSpec> {
        OutenW::new(self, 0)
    }
    #[doc = "Bit 30 - Only applicable on mspi1. When set, use gpio95 .. gpio 104 as the pads. This extra set of pads is to run mspi0 on HEX and mspi1 concurrently. Note that the timing of this extra pads may not be as good as the origianl pads."]
    #[inline(always)]
    #[must_use]
    pub fn padset1(&mut self) -> Padset1W<PadoutenSpec> {
        Padset1W::new(self, 30)
    }
    #[doc = "Bit 31 - Output clock on MSPI data\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn clkond4(&mut self) -> Clkond4W<PadoutenSpec> {
        Clkond4W::new(self, 31)
    }
}
#[doc = "Enable bits for the MSPI output pads. Each active MSPI line should be set to 1 in the OUTEN field below.\n\nYou can [`read`](crate::Reg::read) this register and get [`padouten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padouten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadoutenSpec;
impl crate::RegisterSpec for PadoutenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padouten::R`](R) reader structure"]
impl crate::Readable for PadoutenSpec {}
#[doc = "`write(|w| ..)` method takes [`padouten::W`](W) writer structure"]
impl crate::Writable for PadoutenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADOUTEN to value 0"]
impl crate::Resettable for PadoutenSpec {
    const RESET_VALUE: u32 = 0;
}
