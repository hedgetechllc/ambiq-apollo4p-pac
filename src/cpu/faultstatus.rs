#[doc = "Register `FAULTSTATUS` reader"]
pub type R = crate::R<FaultstatusSpec>;
#[doc = "Register `FAULTSTATUS` writer"]
pub type W = crate::W<FaultstatusSpec>;
#[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icodefault {
    #[doc = "0: No ICODE fault has been detected."]
    Nofault = 0,
    #[doc = "1: ICODE fault detected."]
    Fault = 1,
}
impl From<Icodefault> for bool {
    #[inline(always)]
    fn from(variant: Icodefault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICODEFAULT` reader - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
pub type IcodefaultR = crate::BitReader<Icodefault>;
impl IcodefaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icodefault {
        match self.bits {
            false => Icodefault::Nofault,
            true => Icodefault::Fault,
        }
    }
    #[doc = "No ICODE fault has been detected."]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == Icodefault::Nofault
    }
    #[doc = "ICODE fault detected."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Icodefault::Fault
    }
}
#[doc = "Field `ICODEFAULT` writer - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
pub type IcodefaultW<'a, REG> = crate::BitWriter<'a, REG, Icodefault>;
impl<'a, REG> IcodefaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ICODE fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut crate::W<REG> {
        self.variant(Icodefault::Nofault)
    }
    #[doc = "ICODE fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Icodefault::Fault)
    }
}
#[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcodefault {
    #[doc = "0: No DCODE fault has been detected."]
    Nofault = 0,
    #[doc = "1: DCODE fault detected."]
    Fault = 1,
}
impl From<Dcodefault> for bool {
    #[inline(always)]
    fn from(variant: Dcodefault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCODEFAULT` reader - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
pub type DcodefaultR = crate::BitReader<Dcodefault>;
impl DcodefaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcodefault {
        match self.bits {
            false => Dcodefault::Nofault,
            true => Dcodefault::Fault,
        }
    }
    #[doc = "No DCODE fault has been detected."]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == Dcodefault::Nofault
    }
    #[doc = "DCODE fault detected."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Dcodefault::Fault
    }
}
#[doc = "Field `DCODEFAULT` writer - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
pub type DcodefaultW<'a, REG> = crate::BitWriter<'a, REG, Dcodefault>;
impl<'a, REG> DcodefaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DCODE fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut crate::W<REG> {
        self.variant(Dcodefault::Nofault)
    }
    #[doc = "DCODE fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Dcodefault::Fault)
    }
}
#[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysfault {
    #[doc = "0: No bus fault has been detected."]
    Nofault = 0,
    #[doc = "1: Bus fault detected."]
    Fault = 1,
}
impl From<Sysfault> for bool {
    #[inline(always)]
    fn from(variant: Sysfault) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSFAULT` reader - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
pub type SysfaultR = crate::BitReader<Sysfault>;
impl SysfaultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysfault {
        match self.bits {
            false => Sysfault::Nofault,
            true => Sysfault::Fault,
        }
    }
    #[doc = "No bus fault has been detected."]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        *self == Sysfault::Nofault
    }
    #[doc = "Bus fault detected."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Sysfault::Fault
    }
}
#[doc = "Field `SYSFAULT` writer - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
pub type SysfaultW<'a, REG> = crate::BitWriter<'a, REG, Sysfault>;
impl<'a, REG> SysfaultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus fault has been detected."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut crate::W<REG> {
        self.variant(Sysfault::Nofault)
    }
    #[doc = "Bus fault detected."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Sysfault::Fault)
    }
}
impl R {
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icodefault(&self) -> IcodefaultR {
        IcodefaultR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcodefault(&self) -> DcodefaultR {
        DcodefaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sysfault(&self) -> SysfaultR {
        SysfaultR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    #[must_use]
    pub fn icodefault(&mut self) -> IcodefaultW<FaultstatusSpec> {
        IcodefaultW::new(self, 0)
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    #[must_use]
    pub fn dcodefault(&mut self) -> DcodefaultW<FaultstatusSpec> {
        DcodefaultW::new(self, 1)
    }
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    #[must_use]
    pub fn sysfault(&mut self) -> SysfaultW<FaultstatusSpec> {
        SysfaultW::new(self, 2)
    }
}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nYou can [`read`](crate::Reg::read) this register and get [`faultstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faultstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultstatusSpec;
impl crate::RegisterSpec for FaultstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`faultstatus::R`](R) reader structure"]
impl crate::Readable for FaultstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`faultstatus::W`](W) writer structure"]
impl crate::Writable for FaultstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAULTSTATUS to value 0"]
impl crate::Resettable for FaultstatusSpec {
    const RESET_VALUE: u32 = 0;
}
