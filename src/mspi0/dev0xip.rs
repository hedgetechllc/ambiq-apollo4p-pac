#[doc = "Register `DEV0XIP` reader"]
pub type R = crate::R<Dev0xipSpec>;
#[doc = "Register `DEV0XIP` writer"]
pub type W = crate::W<Dev0xipSpec>;
#[doc = "Field `XIPEN0` reader - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
pub type Xipen0R = crate::BitReader;
#[doc = "Field `XIPEN0` writer - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
pub type Xipen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xipack0 {
    #[doc = "0: No acknowledege sent. Data IOs are tristated the first turnaround cycle"]
    Noack = 0,
    #[doc = "2: Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode"]
    Ack = 2,
    #[doc = "3: Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer"]
    Terminate = 3,
}
impl From<Xipack0> for u8 {
    #[inline(always)]
    fn from(variant: Xipack0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xipack0 {
    type Ux = u8;
}
impl crate::IsEnum for Xipack0 {}
#[doc = "Field `XIPACK0` reader - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
pub type Xipack0R = crate::FieldReader<Xipack0>;
impl Xipack0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xipack0> {
        match self.bits {
            0 => Some(Xipack0::Noack),
            2 => Some(Xipack0::Ack),
            3 => Some(Xipack0::Terminate),
            _ => None,
        }
    }
    #[doc = "No acknowledege sent. Data IOs are tristated the first turnaround cycle"]
    #[inline(always)]
    pub fn is_noack(&self) -> bool {
        *self == Xipack0::Noack
    }
    #[doc = "Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Xipack0::Ack
    }
    #[doc = "Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == Xipack0::Terminate
    }
}
#[doc = "Field `XIPACK0` writer - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
pub type Xipack0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Xipack0>;
impl<'a, REG> Xipack0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No acknowledege sent. Data IOs are tristated the first turnaround cycle"]
    #[inline(always)]
    pub fn noack(self) -> &'a mut crate::W<REG> {
        self.variant(Xipack0::Noack)
    }
    #[doc = "Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Xipack0::Ack)
    }
    #[doc = "Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer"]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut crate::W<REG> {
        self.variant(Xipack0::Terminate)
    }
}
#[doc = "Field `XIPBIGENDIAN0` reader - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
pub type Xipbigendian0R = crate::BitReader;
#[doc = "Field `XIPBIGENDIAN0` writer - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
pub type Xipbigendian0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPENTURN0` reader - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
pub type Xipenturn0R = crate::BitReader;
#[doc = "Field `XIPENTURN0` writer - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
pub type Xipenturn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPSENDA0` reader - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
pub type Xipsenda0R = crate::BitReader;
#[doc = "Field `XIPSENDA0` writer - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
pub type Xipsenda0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPSENDI0` reader - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
pub type Xipsendi0R = crate::BitReader;
#[doc = "Field `XIPSENDI0` writer - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
pub type Xipsendi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Provides override controls for data operations where instruction, address, and data may transfer in different rates.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xipmixed0 {
    #[doc = "0: Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    Normal = 0,
    #[doc = "1: Data operations proceed in dual data rate"]
    D2 = 1,
    #[doc = "3: Address and Data operations proceed in dual data rate"]
    Ad2 = 3,
    #[doc = "5: Data operations proceed in quad data rate"]
    D4 = 5,
    #[doc = "7: Address and Data operations proceed in quad data rate"]
    Ad4 = 7,
    #[doc = "9: Data operations proceed in octal data rate"]
    D8 = 9,
    #[doc = "11: Address and Data operations proceed in octal data rate"]
    Ad8 = 11,
}
impl From<Xipmixed0> for u8 {
    #[inline(always)]
    fn from(variant: Xipmixed0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xipmixed0 {
    type Ux = u8;
}
impl crate::IsEnum for Xipmixed0 {}
#[doc = "Field `XIPMIXED0` reader - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
pub type Xipmixed0R = crate::FieldReader<Xipmixed0>;
impl Xipmixed0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xipmixed0> {
        match self.bits {
            0 => Some(Xipmixed0::Normal),
            1 => Some(Xipmixed0::D2),
            3 => Some(Xipmixed0::Ad2),
            5 => Some(Xipmixed0::D4),
            7 => Some(Xipmixed0::Ad4),
            9 => Some(Xipmixed0::D8),
            11 => Some(Xipmixed0::Ad8),
            _ => None,
        }
    }
    #[doc = "Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Xipmixed0::Normal
    }
    #[doc = "Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Xipmixed0::D2
    }
    #[doc = "Address and Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == Xipmixed0::Ad2
    }
    #[doc = "Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Xipmixed0::D4
    }
    #[doc = "Address and Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        *self == Xipmixed0::Ad4
    }
    #[doc = "Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == Xipmixed0::D8
    }
    #[doc = "Address and Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn is_ad8(&self) -> bool {
        *self == Xipmixed0::Ad8
    }
}
#[doc = "Field `XIPMIXED0` writer - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
pub type Xipmixed0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Xipmixed0>;
impl<'a, REG> Xipmixed0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::Normal)
    }
    #[doc = "Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::D2)
    }
    #[doc = "Address and Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::Ad2)
    }
    #[doc = "Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::D4)
    }
    #[doc = "Address and Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::Ad4)
    }
    #[doc = "Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::D8)
    }
    #[doc = "Address and Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn ad8(self) -> &'a mut crate::W<REG> {
        self.variant(Xipmixed0::Ad8)
    }
}
#[doc = "Field `XIPENDCX0` reader - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
pub type Xipendcx0R = crate::BitReader;
#[doc = "Field `XIPENDCX0` writer - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
pub type Xipendcx0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPENWLAT0` reader - Enable Write Latency counter for XIP write transactions"]
pub type Xipenwlat0R = crate::BitReader;
#[doc = "Field `XIPENWLAT0` writer - Enable Write Latency counter for XIP write transactions"]
pub type Xipenwlat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIPTURNAROUND0` reader - Number of turnaound cycles (for TX->RX transitions). Qualified by XIPENTURN bit field."]
pub type Xipturnaround0R = crate::FieldReader;
#[doc = "Field `XIPTURNAROUND0` writer - Number of turnaound cycles (for TX->RX transitions). Qualified by XIPENTURN bit field."]
pub type Xipturnaround0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `XIPWRITELATENCY0` reader - Number of write Latency cycles. Qualified by XIPENWLAT bit field."]
pub type Xipwritelatency0R = crate::FieldReader;
#[doc = "Field `XIPWRITELATENCY0` writer - Number of write Latency cycles. Qualified by XIPENWLAT bit field."]
pub type Xipwritelatency0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub fn xipen0(&self) -> Xipen0R {
        Xipen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    pub fn xipack0(&self) -> Xipack0R {
        Xipack0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    pub fn xipbigendian0(&self) -> Xipbigendian0R {
        Xipbigendian0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    pub fn xipenturn0(&self) -> Xipenturn0R {
        Xipenturn0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsenda0(&self) -> Xipsenda0R {
        Xipsenda0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsendi0(&self) -> Xipsendi0R {
        Xipsendi0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
    #[inline(always)]
    pub fn xipmixed0(&self) -> Xipmixed0R {
        Xipmixed0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
    #[inline(always)]
    pub fn xipendcx0(&self) -> Xipendcx0R {
        Xipendcx0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Latency counter for XIP write transactions"]
    #[inline(always)]
    pub fn xipenwlat0(&self) -> Xipenwlat0R {
        Xipenwlat0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - Number of turnaound cycles (for TX->RX transitions). Qualified by XIPENTURN bit field."]
    #[inline(always)]
    pub fn xipturnaround0(&self) -> Xipturnaround0R {
        Xipturnaround0R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Number of write Latency cycles. Qualified by XIPENWLAT bit field."]
    #[inline(always)]
    pub fn xipwritelatency0(&self) -> Xipwritelatency0R {
        Xipwritelatency0R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    #[must_use]
    pub fn xipen0(&mut self) -> Xipen0W<Dev0xipSpec> {
        Xipen0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    #[must_use]
    pub fn xipack0(&mut self) -> Xipack0W<Dev0xipSpec> {
        Xipack0W::new(self, 2)
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    #[must_use]
    pub fn xipbigendian0(&mut self) -> Xipbigendian0W<Dev0xipSpec> {
        Xipbigendian0W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    #[must_use]
    pub fn xipenturn0(&mut self) -> Xipenturn0W<Dev0xipSpec> {
        Xipenturn0W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    #[must_use]
    pub fn xipsenda0(&mut self) -> Xipsenda0W<Dev0xipSpec> {
        Xipsenda0W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    #[must_use]
    pub fn xipsendi0(&mut self) -> Xipsendi0W<Dev0xipSpec> {
        Xipsendi0W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
    #[inline(always)]
    #[must_use]
    pub fn xipmixed0(&mut self) -> Xipmixed0W<Dev0xipSpec> {
        Xipmixed0W::new(self, 8)
    }
    #[doc = "Bit 12 - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
    #[inline(always)]
    #[must_use]
    pub fn xipendcx0(&mut self) -> Xipendcx0W<Dev0xipSpec> {
        Xipendcx0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Latency counter for XIP write transactions"]
    #[inline(always)]
    #[must_use]
    pub fn xipenwlat0(&mut self) -> Xipenwlat0W<Dev0xipSpec> {
        Xipenwlat0W::new(self, 13)
    }
    #[doc = "Bits 14:19 - Number of turnaound cycles (for TX->RX transitions). Qualified by XIPENTURN bit field."]
    #[inline(always)]
    #[must_use]
    pub fn xipturnaround0(&mut self) -> Xipturnaround0W<Dev0xipSpec> {
        Xipturnaround0W::new(self, 14)
    }
    #[doc = "Bits 20:25 - Number of write Latency cycles. Qualified by XIPENWLAT bit field."]
    #[inline(always)]
    #[must_use]
    pub fn xipwritelatency0(&mut self) -> Xipwritelatency0W<Dev0xipSpec> {
        Xipwritelatency0W::new(self, 20)
    }
}
#[doc = "When any SPI flash is configured, this register must be properly programmed before XIP or AUTO DMA operations commence.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0xip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0xip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0xipSpec;
impl crate::RegisterSpec for Dev0xipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0xip::R`](R) reader structure"]
impl crate::Readable for Dev0xipSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0xip::W`](W) writer structure"]
impl crate::Writable for Dev0xipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0XIP to value 0"]
impl crate::Resettable for Dev0xipSpec {
    const RESET_VALUE: u32 = 0;
}
