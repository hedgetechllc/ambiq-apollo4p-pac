#[doc = "Register `DEV0XIPMISC` reader"]
pub type R = crate::R<Dev0xipmiscSpec>;
#[doc = "Register `DEV0XIPMISC` writer"]
pub type W = crate::W<Dev0xipmiscSpec>;
#[doc = "Field `CEBREAK0` reader - CEBREAK0 field description needed."]
pub type Cebreak0R = crate::FieldReader<u16>;
#[doc = "Field `CEBREAK0` writer - CEBREAK0 field description needed."]
pub type Cebreak0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Convert odd starting address to word-aligned starting address with byte-enables for holes. For example, an AXI transaction with wstrb of 0x0600 results in mspi transaction of addr=8 and BE=0b1001 ( active low ).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xipodd0 {
    #[doc = "0: No conversion"]
    Dis = 0,
    #[doc = "1: Enable the conversion"]
    En = 1,
}
impl From<Xipodd0> for bool {
    #[inline(always)]
    fn from(variant: Xipodd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIPODD0` reader - Convert odd starting address to word-aligned starting address with byte-enables for holes. For example, an AXI transaction with wstrb of 0x0600 results in mspi transaction of addr=8 and BE=0b1001 ( active low )."]
pub type Xipodd0R = crate::BitReader<Xipodd0>;
impl Xipodd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xipodd0 {
        match self.bits {
            false => Xipodd0::Dis,
            true => Xipodd0::En,
        }
    }
    #[doc = "No conversion"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Xipodd0::Dis
    }
    #[doc = "Enable the conversion"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Xipodd0::En
    }
}
#[doc = "Field `XIPODD0` writer - Convert odd starting address to word-aligned starting address with byte-enables for holes. For example, an AXI transaction with wstrb of 0x0600 results in mspi transaction of addr=8 and BE=0b1001 ( active low )."]
pub type Xipodd0W<'a, REG> = crate::BitWriter<'a, REG, Xipodd0>;
impl<'a, REG> Xipodd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No conversion"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Xipodd0::Dis)
    }
    #[doc = "Enable the conversion"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Xipodd0::En)
    }
}
#[doc = "Field `BEPOL0` reader - byte mask polarity to MSPI xfer"]
pub type Bepol0R = crate::BitReader;
#[doc = "Field `BEPOL0` writer - byte mask polarity to MSPI xfer"]
pub type Bepol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Byte enable always on for all lanes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beon0 {
    #[doc = "0: Byte enable is calculated on the fly"]
    Dis = 0,
    #[doc = "1: Byte enable of all bytes are always on"]
    En = 1,
}
impl From<Beon0> for bool {
    #[inline(always)]
    fn from(variant: Beon0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEON0` reader - Byte enable always on for all lanes"]
pub type Beon0R = crate::BitReader<Beon0>;
impl Beon0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Beon0 {
        match self.bits {
            false => Beon0::Dis,
            true => Beon0::En,
        }
    }
    #[doc = "Byte enable is calculated on the fly"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Beon0::Dis
    }
    #[doc = "Byte enable of all bytes are always on"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Beon0::En
    }
}
#[doc = "Field `BEON0` writer - Byte enable always on for all lanes"]
pub type Beon0W<'a, REG> = crate::BitWriter<'a, REG, Beon0>;
impl<'a, REG> Beon0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Byte enable is calculated on the fly"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Beon0::Dis)
    }
    #[doc = "Byte enable of all bytes are always on"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Beon0::En)
    }
}
#[doc = "Deprecated. No effect on RevC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xipboundary0 {
    #[doc = "0: ERROR: desc VALUE MISSING"]
    Dis = 0,
    #[doc = "1: ERROR: desc VALUE MISSING"]
    En = 1,
}
impl From<Xipboundary0> for bool {
    #[inline(always)]
    fn from(variant: Xipboundary0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIPBOUNDARY0` reader - Deprecated. No effect on RevC."]
pub type Xipboundary0R = crate::BitReader<Xipboundary0>;
impl Xipboundary0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xipboundary0 {
        match self.bits {
            false => Xipboundary0::Dis,
            true => Xipboundary0::En,
        }
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Xipboundary0::Dis
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Xipboundary0::En
    }
}
#[doc = "Field `XIPBOUNDARY0` writer - Deprecated. No effect on RevC."]
pub type Xipboundary0W<'a, REG> = crate::BitWriter<'a, REG, Xipboundary0>;
impl<'a, REG> Xipboundary0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Xipboundary0::Dis)
    }
    #[doc = "ERROR: desc VALUE MISSING"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Xipboundary0::En)
    }
}
#[doc = "Append dummy byte to odd number of write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apndodd0 {
    #[doc = "0: No appending byte"]
    Dis = 0,
    #[doc = "1: Append one dummy byte"]
    En = 1,
}
impl From<Apndodd0> for bool {
    #[inline(always)]
    fn from(variant: Apndodd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APNDODD0` reader - Append dummy byte to odd number of write"]
pub type Apndodd0R = crate::BitReader<Apndodd0>;
impl Apndodd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Apndodd0 {
        match self.bits {
            false => Apndodd0::Dis,
            true => Apndodd0::En,
        }
    }
    #[doc = "No appending byte"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Apndodd0::Dis
    }
    #[doc = "Append one dummy byte"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Apndodd0::En
    }
}
#[doc = "Field `APNDODD0` writer - Append dummy byte to odd number of write"]
pub type Apndodd0W<'a, REG> = crate::BitWriter<'a, REG, Apndodd0>;
impl<'a, REG> Apndodd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No appending byte"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Apndodd0::Dis)
    }
    #[doc = "Append one dummy byte"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Apndodd0::En)
    }
}
impl R {
    #[doc = "Bits 0:11 - CEBREAK0 field description needed."]
    #[inline(always)]
    pub fn cebreak0(&self) -> Cebreak0R {
        Cebreak0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Convert odd starting address to word-aligned starting address with byte-enables for holes. For example, an AXI transaction with wstrb of 0x0600 results in mspi transaction of addr=8 and BE=0b1001 ( active low )."]
    #[inline(always)]
    pub fn xipodd0(&self) -> Xipodd0R {
        Xipodd0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - byte mask polarity to MSPI xfer"]
    #[inline(always)]
    pub fn bepol0(&self) -> Bepol0R {
        Bepol0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Byte enable always on for all lanes"]
    #[inline(always)]
    pub fn beon0(&self) -> Beon0R {
        Beon0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deprecated. No effect on RevC."]
    #[inline(always)]
    pub fn xipboundary0(&self) -> Xipboundary0R {
        Xipboundary0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Append dummy byte to odd number of write"]
    #[inline(always)]
    pub fn apndodd0(&self) -> Apndodd0R {
        Apndodd0R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - CEBREAK0 field description needed."]
    #[inline(always)]
    #[must_use]
    pub fn cebreak0(&mut self) -> Cebreak0W<Dev0xipmiscSpec> {
        Cebreak0W::new(self, 0)
    }
    #[doc = "Bit 12 - Convert odd starting address to word-aligned starting address with byte-enables for holes. For example, an AXI transaction with wstrb of 0x0600 results in mspi transaction of addr=8 and BE=0b1001 ( active low )."]
    #[inline(always)]
    #[must_use]
    pub fn xipodd0(&mut self) -> Xipodd0W<Dev0xipmiscSpec> {
        Xipodd0W::new(self, 12)
    }
    #[doc = "Bit 13 - byte mask polarity to MSPI xfer"]
    #[inline(always)]
    #[must_use]
    pub fn bepol0(&mut self) -> Bepol0W<Dev0xipmiscSpec> {
        Bepol0W::new(self, 13)
    }
    #[doc = "Bit 14 - Byte enable always on for all lanes"]
    #[inline(always)]
    #[must_use]
    pub fn beon0(&mut self) -> Beon0W<Dev0xipmiscSpec> {
        Beon0W::new(self, 14)
    }
    #[doc = "Bit 15 - Deprecated. No effect on RevC."]
    #[inline(always)]
    #[must_use]
    pub fn xipboundary0(&mut self) -> Xipboundary0W<Dev0xipmiscSpec> {
        Xipboundary0W::new(self, 15)
    }
    #[doc = "Bit 21 - Append dummy byte to odd number of write"]
    #[inline(always)]
    #[must_use]
    pub fn apndodd0(&mut self) -> Apndodd0W<Dev0xipmiscSpec> {
        Apndodd0W::new(self, 21)
    }
}
#[doc = "Miscellaneous XIP control registers for AXI logic\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0xipmisc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0xipmisc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0xipmiscSpec;
impl crate::RegisterSpec for Dev0xipmiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0xipmisc::R`](R) reader structure"]
impl crate::Readable for Dev0xipmiscSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0xipmisc::W`](W) writer structure"]
impl crate::Writable for Dev0xipmiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0XIPMISC to value 0x900a"]
impl crate::Resettable for Dev0xipmiscSpec {
    const RESET_VALUE: u32 = 0x900a;
}
