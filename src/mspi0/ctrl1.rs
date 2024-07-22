#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Provides override controls for data operations where instruction, address, and data may transfer in different rates.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piomixed {
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
impl From<Piomixed> for u8 {
    #[inline(always)]
    fn from(variant: Piomixed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Piomixed {
    type Ux = u8;
}
impl crate::IsEnum for Piomixed {}
#[doc = "Field `PIOMIXED` reader - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
pub type PiomixedR = crate::FieldReader<Piomixed>;
impl PiomixedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Piomixed> {
        match self.bits {
            0 => Some(Piomixed::Normal),
            1 => Some(Piomixed::D2),
            3 => Some(Piomixed::Ad2),
            5 => Some(Piomixed::D4),
            7 => Some(Piomixed::Ad4),
            9 => Some(Piomixed::D8),
            11 => Some(Piomixed::Ad8),
            _ => None,
        }
    }
    #[doc = "Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Piomixed::Normal
    }
    #[doc = "Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Piomixed::D2
    }
    #[doc = "Address and Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == Piomixed::Ad2
    }
    #[doc = "Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Piomixed::D4
    }
    #[doc = "Address and Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        *self == Piomixed::Ad4
    }
    #[doc = "Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == Piomixed::D8
    }
    #[doc = "Address and Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn is_ad8(&self) -> bool {
        *self == Piomixed::Ad8
    }
}
#[doc = "Field `PIOMIXED` writer - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
pub type PiomixedW<'a, REG> = crate::FieldWriter<'a, REG, 4, Piomixed>;
impl<'a, REG> PiomixedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::Normal)
    }
    #[doc = "Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::D2)
    }
    #[doc = "Address and Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::Ad2)
    }
    #[doc = "Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::D4)
    }
    #[doc = "Address and Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::Ad4)
    }
    #[doc = "Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::D8)
    }
    #[doc = "Address and Data operations proceed in octal data rate"]
    #[inline(always)]
    pub fn ad8(self) -> &'a mut crate::W<REG> {
        self.variant(Piomixed::Ad8)
    }
}
impl R {
    #[doc = "Bits 0:3 - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
    #[inline(always)]
    pub fn piomixed(&self) -> PiomixedR {
        PiomixedR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Provides override controls for data operations where instruction, address, and data may transfer in different rates."]
    #[inline(always)]
    #[must_use]
    pub fn piomixed(&mut self) -> PiomixedW<Ctrl1Spec> {
        PiomixedW::new(self, 0)
    }
}
#[doc = "These registers are used to enable individual PIO based transactions to a device on the bus. The CFG register must be programmed properly for the transfer, and the ADDR and INSTR registers should be programmed if the SENDI and SENDA fields are enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
