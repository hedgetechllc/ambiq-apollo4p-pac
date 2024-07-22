#[doc = "Register `PKASTATUS` reader"]
pub type R = crate::R<PkastatusSpec>;
#[doc = "Register `PKASTATUS` writer"]
pub type W = crate::W<PkastatusSpec>;
#[doc = "Field `ALUMSB4BITS` reader - The most significant 4-bits of the operand updated in shift operation."]
pub type Alumsb4bitsR = crate::FieldReader;
#[doc = "Field `ALUMSB4BITS` writer - The most significant 4-bits of the operand updated in shift operation."]
pub type Alumsb4bitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALULSB4BITS` reader - The least significant 4-bits of the operand updated in shift operation."]
pub type Alulsb4bitsR = crate::FieldReader;
#[doc = "Field `ALULSB4BITS` writer - The least significant 4-bits of the operand updated in shift operation."]
pub type Alulsb4bitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALUSIGNOUT` reader - Indicates the last operations sign (MSB)."]
pub type AlusignoutR = crate::BitReader;
#[doc = "Field `ALUSIGNOUT` writer - Indicates the last operations sign (MSB)."]
pub type AlusignoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALUCARRY` reader - Holds the carry of the last ALU operation."]
pub type AlucarryR = crate::BitReader;
#[doc = "Field `ALUCARRY` writer - Holds the carry of the last ALU operation."]
pub type AlucarryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALUCARRYMOD` reader - holds the carry of the last Modular operation."]
pub type AlucarrymodR = crate::BitReader;
#[doc = "Field `ALUCARRYMOD` writer - holds the carry of the last Modular operation."]
pub type AlucarrymodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALUSUBISZERO` reader - Indicates the last subtraction operations sign ."]
pub type AlusubiszeroR = crate::BitReader;
#[doc = "Field `ALUSUBISZERO` writer - Indicates the last subtraction operations sign ."]
pub type AlusubiszeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALUOUTZERO` reader - Indicates if the result of ALU OUT is zero."]
pub type AluoutzeroR = crate::BitReader;
#[doc = "Field `ALUOUTZERO` writer - Indicates if the result of ALU OUT is zero."]
pub type AluoutzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALUMODOVRFLW` reader - Modular overflow flag."]
pub type AlumodovrflwR = crate::BitReader;
#[doc = "Field `ALUMODOVRFLW` writer - Modular overflow flag."]
pub type AlumodovrflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVBYZERO` reader - Indication if the division is done by zero."]
pub type DivbyzeroR = crate::BitReader;
#[doc = "Field `DIVBYZERO` writer - Indication if the division is done by zero."]
pub type DivbyzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODINVOFZERO` reader - Indicates the Modular inverse of zero."]
pub type ModinvofzeroR = crate::BitReader;
#[doc = "Field `MODINVOFZERO` writer - Indicates the Modular inverse of zero."]
pub type ModinvofzeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPCODE` reader - Opcode of the last operation"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode of the last operation"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - The most significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    pub fn alumsb4bits(&self) -> Alumsb4bitsR {
        Alumsb4bitsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The least significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    pub fn alulsb4bits(&self) -> Alulsb4bitsR {
        Alulsb4bitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Indicates the last operations sign (MSB)."]
    #[inline(always)]
    pub fn alusignout(&self) -> AlusignoutR {
        AlusignoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Holds the carry of the last ALU operation."]
    #[inline(always)]
    pub fn alucarry(&self) -> AlucarryR {
        AlucarryR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - holds the carry of the last Modular operation."]
    #[inline(always)]
    pub fn alucarrymod(&self) -> AlucarrymodR {
        AlucarrymodR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates the last subtraction operations sign ."]
    #[inline(always)]
    pub fn alusubiszero(&self) -> AlusubiszeroR {
        AlusubiszeroR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates if the result of ALU OUT is zero."]
    #[inline(always)]
    pub fn aluoutzero(&self) -> AluoutzeroR {
        AluoutzeroR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Modular overflow flag."]
    #[inline(always)]
    pub fn alumodovrflw(&self) -> AlumodovrflwR {
        AlumodovrflwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indication if the division is done by zero."]
    #[inline(always)]
    pub fn divbyzero(&self) -> DivbyzeroR {
        DivbyzeroR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates the Modular inverse of zero."]
    #[inline(always)]
    pub fn modinvofzero(&self) -> ModinvofzeroR {
        ModinvofzeroR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Opcode of the last operation"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The most significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn alumsb4bits(&mut self) -> Alumsb4bitsW<PkastatusSpec> {
        Alumsb4bitsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The least significant 4-bits of the operand updated in shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn alulsb4bits(&mut self) -> Alulsb4bitsW<PkastatusSpec> {
        Alulsb4bitsW::new(self, 4)
    }
    #[doc = "Bit 8 - Indicates the last operations sign (MSB)."]
    #[inline(always)]
    #[must_use]
    pub fn alusignout(&mut self) -> AlusignoutW<PkastatusSpec> {
        AlusignoutW::new(self, 8)
    }
    #[doc = "Bit 9 - Holds the carry of the last ALU operation."]
    #[inline(always)]
    #[must_use]
    pub fn alucarry(&mut self) -> AlucarryW<PkastatusSpec> {
        AlucarryW::new(self, 9)
    }
    #[doc = "Bit 10 - holds the carry of the last Modular operation."]
    #[inline(always)]
    #[must_use]
    pub fn alucarrymod(&mut self) -> AlucarrymodW<PkastatusSpec> {
        AlucarrymodW::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates the last subtraction operations sign ."]
    #[inline(always)]
    #[must_use]
    pub fn alusubiszero(&mut self) -> AlusubiszeroW<PkastatusSpec> {
        AlusubiszeroW::new(self, 11)
    }
    #[doc = "Bit 12 - Indicates if the result of ALU OUT is zero."]
    #[inline(always)]
    #[must_use]
    pub fn aluoutzero(&mut self) -> AluoutzeroW<PkastatusSpec> {
        AluoutzeroW::new(self, 12)
    }
    #[doc = "Bit 13 - Modular overflow flag."]
    #[inline(always)]
    #[must_use]
    pub fn alumodovrflw(&mut self) -> AlumodovrflwW<PkastatusSpec> {
        AlumodovrflwW::new(self, 13)
    }
    #[doc = "Bit 14 - Indication if the division is done by zero."]
    #[inline(always)]
    #[must_use]
    pub fn divbyzero(&mut self) -> DivbyzeroW<PkastatusSpec> {
        DivbyzeroW::new(self, 14)
    }
    #[doc = "Bit 15 - Indicates the Modular inverse of zero."]
    #[inline(always)]
    #[must_use]
    pub fn modinvofzero(&mut self) -> ModinvofzeroW<PkastatusSpec> {
        ModinvofzeroW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Opcode of the last operation"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OpcodeW<PkastatusSpec> {
        OpcodeW::new(self, 16)
    }
}
#[doc = "This register holds the PKA pipe status.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkastatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkastatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkastatusSpec;
impl crate::RegisterSpec for PkastatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkastatus::R`](R) reader structure"]
impl crate::Readable for PkastatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pkastatus::W`](W) writer structure"]
impl crate::Writable for PkastatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKASTATUS to value 0x1000"]
impl crate::Resettable for PkastatusSpec {
    const RESET_VALUE: u32 = 0x1000;
}
