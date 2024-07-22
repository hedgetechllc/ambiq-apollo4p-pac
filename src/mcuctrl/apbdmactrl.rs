#[doc = "Register `APBDMACTRL` reader"]
pub type R = crate::R<ApbdmactrlSpec>;
#[doc = "Register `APBDMACTRL` writer"]
pub type W = crate::W<ApbdmactrlSpec>;
#[doc = "Enable the DMA controller. When disabled, DMA requests will be ignored by the controller\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaenable {
    #[doc = "0: DMA operations disabled"]
    Disable = 0,
    #[doc = "1: DMA operations enabled"]
    Enable = 1,
}
impl From<Dmaenable> for bool {
    #[inline(always)]
    fn from(variant: Dmaenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAENABLE` reader - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
pub type DmaenableR = crate::BitReader<Dmaenable>;
impl DmaenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaenable {
        match self.bits {
            false => Dmaenable::Disable,
            true => Dmaenable::Enable,
        }
    }
    #[doc = "DMA operations disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaenable::Disable
    }
    #[doc = "DMA operations enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaenable::Enable
    }
}
#[doc = "Field `DMAENABLE` writer - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
pub type DmaenableW<'a, REG> = crate::BitWriter<'a, REG, Dmaenable>;
impl<'a, REG> DmaenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA operations disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenable::Disable)
    }
    #[doc = "DMA operations enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenable::Enable)
    }
}
#[doc = "APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decodeabort {
    #[doc = "0: Bus operations to powered down peripherals are quietly discarded"]
    Disable = 0,
    #[doc = "1: Bus operations to powered down peripherals result in a bus fault."]
    Enable = 1,
}
impl From<Decodeabort> for bool {
    #[inline(always)]
    fn from(variant: Decodeabort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECODEABORT` reader - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
pub type DecodeabortR = crate::BitReader<Decodeabort>;
impl DecodeabortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Decodeabort {
        match self.bits {
            false => Decodeabort::Disable,
            true => Decodeabort::Enable,
        }
    }
    #[doc = "Bus operations to powered down peripherals are quietly discarded"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Decodeabort::Disable
    }
    #[doc = "Bus operations to powered down peripherals result in a bus fault."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Decodeabort::Enable
    }
}
#[doc = "Field `DECODEABORT` writer - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
pub type DecodeabortW<'a, REG> = crate::BitWriter<'a, REG, Decodeabort>;
impl<'a, REG> DecodeabortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus operations to powered down peripherals are quietly discarded"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Decodeabort::Disable)
    }
    #[doc = "Bus operations to powered down peripherals result in a bus fault."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Decodeabort::Enable)
    }
}
#[doc = "Field `HYSTERESIS` reader - This field determines how long the DMA engine of apb/disp/gfx will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
pub type HysteresisR = crate::FieldReader;
#[doc = "Field `HYSTERESIS` writer - This field determines how long the DMA engine of apb/disp/gfx will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
pub type HysteresisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DmaenableR {
        DmaenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    pub fn decodeabort(&self) -> DecodeabortR {
        DecodeabortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This field determines how long the DMA engine of apb/disp/gfx will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    pub fn hysteresis(&self) -> HysteresisR {
        HysteresisR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the DMA controller. When disabled, DMA requests will be ignored by the controller"]
    #[inline(always)]
    #[must_use]
    pub fn dmaenable(&mut self) -> DmaenableW<ApbdmactrlSpec> {
        DmaenableW::new(self, 0)
    }
    #[doc = "Bit 1 - APB Decode Abort. When set, the APB bridge will issue a data abort (bus fault) on transactions to peripherals that are powered down. When set to 0, writes are quietly discarded and reads return 0."]
    #[inline(always)]
    #[must_use]
    pub fn decodeabort(&mut self) -> DecodeabortW<ApbdmactrlSpec> {
        DecodeabortW::new(self, 1)
    }
    #[doc = "Bits 8:15 - This field determines how long the DMA engine of apb/disp/gfx will remain active during deep sleep before shutting down and returning the system to full deep sleep. Values are based on a 94KHz clock and are roughly 10us increments for a range of ~10us to 2.55ms"]
    #[inline(always)]
    #[must_use]
    pub fn hysteresis(&mut self) -> HysteresisW<ApbdmactrlSpec> {
        HysteresisW::new(self, 8)
    }
}
#[doc = "DMA Control Register. Determines misc settings for DMA operation\n\nYou can [`read`](crate::Reg::read) this register and get [`apbdmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbdmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbdmactrlSpec;
impl crate::RegisterSpec for ApbdmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbdmactrl::R`](R) reader structure"]
impl crate::Readable for ApbdmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`apbdmactrl::W`](W) writer structure"]
impl crate::Writable for ApbdmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBDMACTRL to value 0x0203"]
impl crate::Resettable for ApbdmactrlSpec {
    const RESET_VALUE: u32 = 0x0203;
}
