#[doc = "Register `OPCODE` reader"]
pub type R = crate::R<OpcodeSpec>;
#[doc = "Register `OPCODE` writer"]
pub type W = crate::W<OpcodeSpec>;
#[doc = "Field `TAG` reader - Holds the operations tag or the operand C virtual address."]
pub type TagR = crate::FieldReader;
#[doc = "Field `TAG` writer - Holds the operations tag or the operand C virtual address."]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGR` reader - Result register virtual address 0-15."]
pub type RegrR = crate::FieldReader;
#[doc = "Field `REGR` writer - Result register virtual address 0-15."]
pub type RegrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGB` reader - Operand B virtual address 0-15."]
pub type RegbR = crate::FieldReader;
#[doc = "Field `REGB` writer - Operand B virtual address 0-15."]
pub type RegbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGA` reader - Operand A virtual address 0-15."]
pub type RegaR = crate::FieldReader;
#[doc = "Field `REGA` writer - Operand A virtual address 0-15."]
pub type RegaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LEN` reader - The length of the operation. The value serves as a pointer to PKA length register, for example, if the value is 0, PKA_L0 holds the size of the operation."]
pub type LenR = crate::FieldReader;
#[doc = "Field `LEN` writer - The length of the operation. The value serves as a pointer to PKA length register, for example, if the value is 0, PKA_L0 holds the size of the operation."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Defines the PKA operation:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    #[doc = "4: Add,Inc opcode"]
    Add = 4,
    #[doc = "5: Sub,Dec,Neg opcode"]
    Sub = 5,
    #[doc = "6: ModAdd,ModInc opcode"]
    Modadd = 6,
    #[doc = "7: ModSub,ModDec,ModNeg opcode"]
    Modsub = 7,
    #[doc = "8: AND,TST0,CLR0 opcode"]
    And = 8,
    #[doc = "9: OR,COPY,SET0 opcode"]
    Or = 9,
    #[doc = "10: XOR,FLIP0,INVERT,COMPARE opcode"]
    Xor = 10,
    #[doc = "12: SHR0 opcode"]
    Shr0 = 12,
    #[doc = "13: SHR1 opcode"]
    Shr1 = 13,
    #[doc = "14: SHL0 opcode"]
    Shl0 = 14,
    #[doc = "15: SHL1 opcode"]
    Shl1 = 15,
    #[doc = "16: MulLow opcode"]
    Mullow = 16,
    #[doc = "17: ModMul opcode"]
    Modmul = 17,
    #[doc = "18: ModMulN opcode"]
    Modmuln = 18,
    #[doc = "19: ModExp opcode"]
    Modexp = 19,
    #[doc = "20: Division opcode"]
    Division = 20,
    #[doc = "21: Div opcode"]
    Div = 21,
    #[doc = "22: ModDiv opcode"]
    Moddiv = 22,
    #[doc = "0: Terminate opcode"]
    Terminate = 0,
}
impl From<Opcode> for u8 {
    #[inline(always)]
    fn from(variant: Opcode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opcode {
    type Ux = u8;
}
impl crate::IsEnum for Opcode {}
#[doc = "Field `OPCODE` reader - Defines the PKA operation:"]
pub type OpcodeR = crate::FieldReader<Opcode>;
impl OpcodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Opcode> {
        match self.bits {
            4 => Some(Opcode::Add),
            5 => Some(Opcode::Sub),
            6 => Some(Opcode::Modadd),
            7 => Some(Opcode::Modsub),
            8 => Some(Opcode::And),
            9 => Some(Opcode::Or),
            10 => Some(Opcode::Xor),
            12 => Some(Opcode::Shr0),
            13 => Some(Opcode::Shr1),
            14 => Some(Opcode::Shl0),
            15 => Some(Opcode::Shl1),
            16 => Some(Opcode::Mullow),
            17 => Some(Opcode::Modmul),
            18 => Some(Opcode::Modmuln),
            19 => Some(Opcode::Modexp),
            20 => Some(Opcode::Division),
            21 => Some(Opcode::Div),
            22 => Some(Opcode::Moddiv),
            0 => Some(Opcode::Terminate),
            _ => None,
        }
    }
    #[doc = "Add,Inc opcode"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == Opcode::Add
    }
    #[doc = "Sub,Dec,Neg opcode"]
    #[inline(always)]
    pub fn is_sub(&self) -> bool {
        *self == Opcode::Sub
    }
    #[doc = "ModAdd,ModInc opcode"]
    #[inline(always)]
    pub fn is_modadd(&self) -> bool {
        *self == Opcode::Modadd
    }
    #[doc = "ModSub,ModDec,ModNeg opcode"]
    #[inline(always)]
    pub fn is_modsub(&self) -> bool {
        *self == Opcode::Modsub
    }
    #[doc = "AND,TST0,CLR0 opcode"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == Opcode::And
    }
    #[doc = "OR,COPY,SET0 opcode"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == Opcode::Or
    }
    #[doc = "XOR,FLIP0,INVERT,COMPARE opcode"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == Opcode::Xor
    }
    #[doc = "SHR0 opcode"]
    #[inline(always)]
    pub fn is_shr0(&self) -> bool {
        *self == Opcode::Shr0
    }
    #[doc = "SHR1 opcode"]
    #[inline(always)]
    pub fn is_shr1(&self) -> bool {
        *self == Opcode::Shr1
    }
    #[doc = "SHL0 opcode"]
    #[inline(always)]
    pub fn is_shl0(&self) -> bool {
        *self == Opcode::Shl0
    }
    #[doc = "SHL1 opcode"]
    #[inline(always)]
    pub fn is_shl1(&self) -> bool {
        *self == Opcode::Shl1
    }
    #[doc = "MulLow opcode"]
    #[inline(always)]
    pub fn is_mullow(&self) -> bool {
        *self == Opcode::Mullow
    }
    #[doc = "ModMul opcode"]
    #[inline(always)]
    pub fn is_modmul(&self) -> bool {
        *self == Opcode::Modmul
    }
    #[doc = "ModMulN opcode"]
    #[inline(always)]
    pub fn is_modmuln(&self) -> bool {
        *self == Opcode::Modmuln
    }
    #[doc = "ModExp opcode"]
    #[inline(always)]
    pub fn is_modexp(&self) -> bool {
        *self == Opcode::Modexp
    }
    #[doc = "Division opcode"]
    #[inline(always)]
    pub fn is_division(&self) -> bool {
        *self == Opcode::Division
    }
    #[doc = "Div opcode"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == Opcode::Div
    }
    #[doc = "ModDiv opcode"]
    #[inline(always)]
    pub fn is_moddiv(&self) -> bool {
        *self == Opcode::Moddiv
    }
    #[doc = "Terminate opcode"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == Opcode::Terminate
    }
}
#[doc = "Field `OPCODE` writer - Defines the PKA operation:"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Opcode>;
impl<'a, REG> OpcodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Add,Inc opcode"]
    #[inline(always)]
    pub fn add(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Add)
    }
    #[doc = "Sub,Dec,Neg opcode"]
    #[inline(always)]
    pub fn sub(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Sub)
    }
    #[doc = "ModAdd,ModInc opcode"]
    #[inline(always)]
    pub fn modadd(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Modadd)
    }
    #[doc = "ModSub,ModDec,ModNeg opcode"]
    #[inline(always)]
    pub fn modsub(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Modsub)
    }
    #[doc = "AND,TST0,CLR0 opcode"]
    #[inline(always)]
    pub fn and(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::And)
    }
    #[doc = "OR,COPY,SET0 opcode"]
    #[inline(always)]
    pub fn or(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Or)
    }
    #[doc = "XOR,FLIP0,INVERT,COMPARE opcode"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Xor)
    }
    #[doc = "SHR0 opcode"]
    #[inline(always)]
    pub fn shr0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shr0)
    }
    #[doc = "SHR1 opcode"]
    #[inline(always)]
    pub fn shr1(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shr1)
    }
    #[doc = "SHL0 opcode"]
    #[inline(always)]
    pub fn shl0(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shl0)
    }
    #[doc = "SHL1 opcode"]
    #[inline(always)]
    pub fn shl1(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Shl1)
    }
    #[doc = "MulLow opcode"]
    #[inline(always)]
    pub fn mullow(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Mullow)
    }
    #[doc = "ModMul opcode"]
    #[inline(always)]
    pub fn modmul(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Modmul)
    }
    #[doc = "ModMulN opcode"]
    #[inline(always)]
    pub fn modmuln(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Modmuln)
    }
    #[doc = "ModExp opcode"]
    #[inline(always)]
    pub fn modexp(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Modexp)
    }
    #[doc = "Division opcode"]
    #[inline(always)]
    pub fn division(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Division)
    }
    #[doc = "Div opcode"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Div)
    }
    #[doc = "ModDiv opcode"]
    #[inline(always)]
    pub fn moddiv(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Moddiv)
    }
    #[doc = "Terminate opcode"]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut crate::W<REG> {
        self.variant(Opcode::Terminate)
    }
}
impl R {
    #[doc = "Bits 0:5 - Holds the operations tag or the operand C virtual address."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Result register virtual address 0-15."]
    #[inline(always)]
    pub fn regr(&self) -> RegrR {
        RegrR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Operand B virtual address 0-15."]
    #[inline(always)]
    pub fn regb(&self) -> RegbR {
        RegbR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Operand A virtual address 0-15."]
    #[inline(always)]
    pub fn rega(&self) -> RegaR {
        RegaR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - The length of the operation. The value serves as a pointer to PKA length register, for example, if the value is 0, PKA_L0 holds the size of the operation."]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - Defines the PKA operation:"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Holds the operations tag or the operand C virtual address."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<OpcodeSpec> {
        TagW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Result register virtual address 0-15."]
    #[inline(always)]
    #[must_use]
    pub fn regr(&mut self) -> RegrW<OpcodeSpec> {
        RegrW::new(self, 6)
    }
    #[doc = "Bits 12:17 - Operand B virtual address 0-15."]
    #[inline(always)]
    #[must_use]
    pub fn regb(&mut self) -> RegbW<OpcodeSpec> {
        RegbW::new(self, 12)
    }
    #[doc = "Bits 18:23 - Operand A virtual address 0-15."]
    #[inline(always)]
    #[must_use]
    pub fn rega(&mut self) -> RegaW<OpcodeSpec> {
        RegaW::new(self, 18)
    }
    #[doc = "Bits 24:26 - The length of the operation. The value serves as a pointer to PKA length register, for example, if the value is 0, PKA_L0 holds the size of the operation."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<OpcodeSpec> {
        LenW::new(self, 24)
    }
    #[doc = "Bits 27:31 - Defines the PKA operation:"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OpcodeW<OpcodeSpec> {
        OpcodeW::new(self, 27)
    }
}
#[doc = "This register holds the PKAs OPCODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`opcode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpcodeSpec;
impl crate::RegisterSpec for OpcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcode::R`](R) reader structure"]
impl crate::Readable for OpcodeSpec {}
#[doc = "`write(|w| ..)` method takes [`opcode::W`](W) writer structure"]
impl crate::Writable for OpcodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPCODE to value 0"]
impl crate::Resettable for OpcodeSpec {
    const RESET_VALUE: u32 = 0;
}
