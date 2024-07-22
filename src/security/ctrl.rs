#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    #[doc = "0: Perform CRC32 operation"]
    Crc32 = 0,
    #[doc = "1: DMA pseudo-random number stream based on CRC value"]
    Rand = 1,
    #[doc = "2: Generate DMA stream based on address"]
    Genaddr = 2,
}
impl From<Function> for u8 {
    #[inline(always)]
    fn from(variant: Function) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Function {
    type Ux = u8;
}
impl crate::IsEnum for Function {}
#[doc = "Field `FUNCTION` reader - Function Select"]
pub type FunctionR = crate::FieldReader<Function>;
impl FunctionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Function> {
        match self.bits {
            0 => Some(Function::Crc32),
            1 => Some(Function::Rand),
            2 => Some(Function::Genaddr),
            _ => None,
        }
    }
    #[doc = "Perform CRC32 operation"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Function::Crc32
    }
    #[doc = "DMA pseudo-random number stream based on CRC value"]
    #[inline(always)]
    pub fn is_rand(&self) -> bool {
        *self == Function::Rand
    }
    #[doc = "Generate DMA stream based on address"]
    #[inline(always)]
    pub fn is_genaddr(&self) -> bool {
        *self == Function::Genaddr
    }
}
#[doc = "Field `FUNCTION` writer - Function Select"]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 4, Function>;
impl<'a, REG> FunctionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Perform CRC32 operation"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Function::Crc32)
    }
    #[doc = "DMA pseudo-random number stream based on CRC value"]
    #[inline(always)]
    pub fn rand(self) -> &'a mut crate::W<REG> {
        self.variant(Function::Rand)
    }
    #[doc = "Generate DMA stream based on address"]
    #[inline(always)]
    pub fn genaddr(self) -> &'a mut crate::W<REG> {
        self.variant(Function::Genaddr)
    }
}
#[doc = "Field `CRCERROR` reader - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
pub type CrcerrorR = crate::BitReader;
#[doc = "Field `CRCERROR` writer - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
pub type CrcerrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline(always)]
    pub fn crcerror(&self) -> CrcerrorR {
        CrcerrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function Enable. Software should set the ENABLE bit to initiate a CRC operation. Hardware will clear the ENABLE bit upon completion."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<CtrlSpec> {
        FunctionW::new(self, 4)
    }
    #[doc = "Bit 31 - CRC Error Status - Set to 1 if an error occurs during a CRC operation. Cleared when CTRL register is written (with any value). Usually indicates an invalid address range."]
    #[inline(always)]
    #[must_use]
    pub fn crcerror(&mut self) -> CrcerrorW<CtrlSpec> {
        CrcerrorW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
