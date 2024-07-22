#[doc = "Register `DEV0BOUNDARY` reader"]
pub type R = crate::R<Dev0boundarySpec>;
#[doc = "Register `DEV0BOUNDARY` writer"]
pub type W = crate::W<Dev0boundarySpec>;
#[doc = "Field `DMATIMELIMIT0` reader - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in ~100 ns increments for the 96 MHz clock input. A value of 0 disables the counter."]
pub type Dmatimelimit0R = crate::FieldReader<u16>;
#[doc = "Field `DMATIMELIMIT0` writer - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in ~100 ns increments for the 96 MHz clock input. A value of 0 disables the counter."]
pub type Dmatimelimit0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "DMA Address boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmabound0 {
    #[doc = "0: Disable DMA address boundary breaks"]
    None = 0,
    #[doc = "1: Break at 32 byte boundary (0x20 increments)"]
    Break32 = 1,
    #[doc = "2: Break at 64 byte boundary (0x40 increments)"]
    Break64 = 2,
    #[doc = "3: Break at 128 byte boundary (0x80 increments)"]
    Break128 = 3,
    #[doc = "4: Break at 256 byte boundary (0x100 increments)"]
    Break256 = 4,
    #[doc = "5: Break at 512 byte boundary (0x200 increments)"]
    Break512 = 5,
    #[doc = "6: Break at 1KB boundary (0x400 increments)"]
    Break1k = 6,
    #[doc = "7: Break at 2KB boundary (0x800 increments)"]
    Break2k = 7,
    #[doc = "8: Break at 4KB boundary (0x1000 increments)"]
    Break4k = 8,
    #[doc = "9: Break at 8KB boundary (0x2000 increments)"]
    Break8k = 9,
    #[doc = "10: Break at 16KB boundary (0x4000 increments)"]
    Break16k = 10,
}
impl From<Dmabound0> for u8 {
    #[inline(always)]
    fn from(variant: Dmabound0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmabound0 {
    type Ux = u8;
}
impl crate::IsEnum for Dmabound0 {}
#[doc = "Field `DMABOUND0` reader - DMA Address boundary"]
pub type Dmabound0R = crate::FieldReader<Dmabound0>;
impl Dmabound0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmabound0> {
        match self.bits {
            0 => Some(Dmabound0::None),
            1 => Some(Dmabound0::Break32),
            2 => Some(Dmabound0::Break64),
            3 => Some(Dmabound0::Break128),
            4 => Some(Dmabound0::Break256),
            5 => Some(Dmabound0::Break512),
            6 => Some(Dmabound0::Break1k),
            7 => Some(Dmabound0::Break2k),
            8 => Some(Dmabound0::Break4k),
            9 => Some(Dmabound0::Break8k),
            10 => Some(Dmabound0::Break16k),
            _ => None,
        }
    }
    #[doc = "Disable DMA address boundary breaks"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dmabound0::None
    }
    #[doc = "Break at 32 byte boundary (0x20 increments)"]
    #[inline(always)]
    pub fn is_break32(&self) -> bool {
        *self == Dmabound0::Break32
    }
    #[doc = "Break at 64 byte boundary (0x40 increments)"]
    #[inline(always)]
    pub fn is_break64(&self) -> bool {
        *self == Dmabound0::Break64
    }
    #[doc = "Break at 128 byte boundary (0x80 increments)"]
    #[inline(always)]
    pub fn is_break128(&self) -> bool {
        *self == Dmabound0::Break128
    }
    #[doc = "Break at 256 byte boundary (0x100 increments)"]
    #[inline(always)]
    pub fn is_break256(&self) -> bool {
        *self == Dmabound0::Break256
    }
    #[doc = "Break at 512 byte boundary (0x200 increments)"]
    #[inline(always)]
    pub fn is_break512(&self) -> bool {
        *self == Dmabound0::Break512
    }
    #[doc = "Break at 1KB boundary (0x400 increments)"]
    #[inline(always)]
    pub fn is_break1k(&self) -> bool {
        *self == Dmabound0::Break1k
    }
    #[doc = "Break at 2KB boundary (0x800 increments)"]
    #[inline(always)]
    pub fn is_break2k(&self) -> bool {
        *self == Dmabound0::Break2k
    }
    #[doc = "Break at 4KB boundary (0x1000 increments)"]
    #[inline(always)]
    pub fn is_break4k(&self) -> bool {
        *self == Dmabound0::Break4k
    }
    #[doc = "Break at 8KB boundary (0x2000 increments)"]
    #[inline(always)]
    pub fn is_break8k(&self) -> bool {
        *self == Dmabound0::Break8k
    }
    #[doc = "Break at 16KB boundary (0x4000 increments)"]
    #[inline(always)]
    pub fn is_break16k(&self) -> bool {
        *self == Dmabound0::Break16k
    }
}
#[doc = "Field `DMABOUND0` writer - DMA Address boundary"]
pub type Dmabound0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Dmabound0>;
impl<'a, REG> Dmabound0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable DMA address boundary breaks"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::None)
    }
    #[doc = "Break at 32 byte boundary (0x20 increments)"]
    #[inline(always)]
    pub fn break32(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break32)
    }
    #[doc = "Break at 64 byte boundary (0x40 increments)"]
    #[inline(always)]
    pub fn break64(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break64)
    }
    #[doc = "Break at 128 byte boundary (0x80 increments)"]
    #[inline(always)]
    pub fn break128(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break128)
    }
    #[doc = "Break at 256 byte boundary (0x100 increments)"]
    #[inline(always)]
    pub fn break256(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break256)
    }
    #[doc = "Break at 512 byte boundary (0x200 increments)"]
    #[inline(always)]
    pub fn break512(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break512)
    }
    #[doc = "Break at 1KB boundary (0x400 increments)"]
    #[inline(always)]
    pub fn break1k(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break1k)
    }
    #[doc = "Break at 2KB boundary (0x800 increments)"]
    #[inline(always)]
    pub fn break2k(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break2k)
    }
    #[doc = "Break at 4KB boundary (0x1000 increments)"]
    #[inline(always)]
    pub fn break4k(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break4k)
    }
    #[doc = "Break at 8KB boundary (0x2000 increments)"]
    #[inline(always)]
    pub fn break8k(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break8k)
    }
    #[doc = "Break at 16KB boundary (0x4000 increments)"]
    #[inline(always)]
    pub fn break16k(self) -> &'a mut crate::W<REG> {
        self.variant(Dmabound0::Break16k)
    }
}
impl R {
    #[doc = "Bits 0:11 - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in ~100 ns increments for the 96 MHz clock input. A value of 0 disables the counter."]
    #[inline(always)]
    pub fn dmatimelimit0(&self) -> Dmatimelimit0R {
        Dmatimelimit0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - DMA Address boundary"]
    #[inline(always)]
    pub fn dmabound0(&self) -> Dmabound0R {
        Dmabound0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in ~100 ns increments for the 96 MHz clock input. A value of 0 disables the counter."]
    #[inline(always)]
    #[must_use]
    pub fn dmatimelimit0(&mut self) -> Dmatimelimit0W<Dev0boundarySpec> {
        Dmatimelimit0W::new(self, 0)
    }
    #[doc = "Bits 12:15 - DMA Address boundary"]
    #[inline(always)]
    #[must_use]
    pub fn dmabound0(&mut self) -> Dmabound0W<Dev0boundarySpec> {
        Dmabound0W::new(self, 12)
    }
}
#[doc = "Allows large transfers to be broken up into smaller ones in hardware to accommodate needs of external devices and allow XIP/XIPMM. Only applicable for memory-mapped devices (PSRAM, Flash, etc) where address can be retransmitted without side effects.\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0boundary::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0boundary::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0boundarySpec;
impl crate::RegisterSpec for Dev0boundarySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0boundary::R`](R) reader structure"]
impl crate::Readable for Dev0boundarySpec {}
#[doc = "`write(|w| ..)` method takes [`dev0boundary::W`](W) writer structure"]
impl crate::Writable for Dev0boundarySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV0BOUNDARY to value 0"]
impl crate::Resettable for Dev0boundarySpec {
    const RESET_VALUE: u32 = 0;
}
