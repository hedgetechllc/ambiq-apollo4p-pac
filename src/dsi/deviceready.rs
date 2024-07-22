#[doc = "Register `DEVICEREADY` reader"]
pub type R = crate::R<DevicereadySpec>;
#[doc = "Register `DEVICEREADY` writer"]
pub type W = crate::W<DevicereadySpec>;
#[doc = "Ready for programming after all count registers and timeout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "1: Set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed."]
    Programmed = 1,
    #[doc = "0: Set by the processor to inform that device is ready for transmission. This register should be set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed. Note: Reprogramming the registers by resetting the device_ready bit results in re-enumeration of the DSI controller from the power up sequence."]
    Ready = 0,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Ready for programming after all count registers and timeout."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            true => Ready::Programmed,
            false => Ready::Ready,
        }
    }
    #[doc = "Set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed."]
    #[inline(always)]
    pub fn is_programmed(&self) -> bool {
        *self == Ready::Programmed
    }
    #[doc = "Set by the processor to inform that device is ready for transmission. This register should be set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed. Note: Reprogramming the registers by resetting the device_ready bit results in re-enumeration of the DSI controller from the power up sequence."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
}
#[doc = "Field `READY` writer - Ready for programming after all count registers and timeout."]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, Ready>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed."]
    #[inline(always)]
    pub fn programmed(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Programmed)
    }
    #[doc = "Set by the processor to inform that device is ready for transmission. This register should be set to 1 after dphy_parameter register, all the count registers, and timeout and interrupt enable registers are being programmed. Note: Reprogramming the registers by resetting the device_ready bit results in re-enumeration of the DSI controller from the power up sequence."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Ready)
    }
}
#[doc = "ULPS field of the DEVICEREADY register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ulps {
    #[doc = "2: This pattern is set by the processor to inform that entire DSI host is to be put on ultra low power \\[POWER SAVING\\]
mode 01 - This pattern is set by the processor to inform"]
    LowPower = 2,
    #[doc = "1: This pattern is set by the processor to inform that entire DSI host is to be pu on ultr low power EXIT mode"]
    Exit = 1,
    #[doc = "0: pattern is set by the processor to make the DSI host come out of the wakeup time and resume the normal operation if the DSI Host already remains in the ULPS exit state. S/W needs to ensure that there is a minimum of 1ms time available before clearing the UPLS exit State. 1(a). In DPI Only Mode: No DPI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states. 1(b). In Dual Display Mode: DPI Interface has to be disabled and also no DBI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states."]
    This = 0,
}
impl From<Ulps> for u8 {
    #[inline(always)]
    fn from(variant: Ulps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ulps {
    type Ux = u8;
}
impl crate::IsEnum for Ulps {}
#[doc = "Field `ULPS` reader - ULPS field of the DEVICEREADY register."]
pub type UlpsR = crate::FieldReader<Ulps>;
impl UlpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ulps> {
        match self.bits {
            2 => Some(Ulps::LowPower),
            1 => Some(Ulps::Exit),
            0 => Some(Ulps::This),
            _ => None,
        }
    }
    #[doc = "This pattern is set by the processor to inform that entire DSI host is to be put on ultra low power \\[POWER SAVING\\]
mode 01 - This pattern is set by the processor to inform"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Ulps::LowPower
    }
    #[doc = "This pattern is set by the processor to inform that entire DSI host is to be pu on ultr low power EXIT mode"]
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == Ulps::Exit
    }
    #[doc = "pattern is set by the processor to make the DSI host come out of the wakeup time and resume the normal operation if the DSI Host already remains in the ULPS exit state. S/W needs to ensure that there is a minimum of 1ms time available before clearing the UPLS exit State. 1(a). In DPI Only Mode: No DPI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states. 1(b). In Dual Display Mode: DPI Interface has to be disabled and also no DBI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states."]
    #[inline(always)]
    pub fn is_this(&self) -> bool {
        *self == Ulps::This
    }
}
#[doc = "Field `ULPS` writer - ULPS field of the DEVICEREADY register."]
pub type UlpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ulps>;
impl<'a, REG> UlpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This pattern is set by the processor to inform that entire DSI host is to be put on ultra low power \\[POWER SAVING\\]
mode 01 - This pattern is set by the processor to inform"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Ulps::LowPower)
    }
    #[doc = "This pattern is set by the processor to inform that entire DSI host is to be pu on ultr low power EXIT mode"]
    #[inline(always)]
    pub fn exit(self) -> &'a mut crate::W<REG> {
        self.variant(Ulps::Exit)
    }
    #[doc = "pattern is set by the processor to make the DSI host come out of the wakeup time and resume the normal operation if the DSI Host already remains in the ULPS exit state. S/W needs to ensure that there is a minimum of 1ms time available before clearing the UPLS exit State. 1(a). In DPI Only Mode: No DPI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states. 1(b). In Dual Display Mode: DPI Interface has to be disabled and also no DBI traffic should be sent after the above patterns like 10 or 01 is set in this register. Device_ready bit in Device Ready register should not be disturbed or should remain set while the device is subjected to ULPS states."]
    #[inline(always)]
    pub fn this(self) -> &'a mut crate::W<REG> {
        self.variant(Ulps::This)
    }
}
#[doc = "Field `DISPLAYBUSPOSSESSEN` reader - Inform DSI receiver has to be given the bus possession for receiving the tearing effect trigger message; Reset by the processor to stop the bus possession of the DSI receiver; Note: Tearing effect is supported only in Type1; Display Architecture (command mode only) as suggested by Display Command Set Specification; Note1: Even if the processor does not clear the display_bus_possession bit after receiving the interrupt for tearing effect, DSI-tx controller starts the activities on the DSI link once the TE trigger message is received"]
pub type DisplaybuspossessenR = crate::BitReader;
#[doc = "Field `DISPLAYBUSPOSSESSEN` writer - Inform DSI receiver has to be given the bus possession for receiving the tearing effect trigger message; Reset by the processor to stop the bus possession of the DSI receiver; Note: Tearing effect is supported only in Type1; Display Architecture (command mode only) as suggested by Display Command Set Specification; Note1: Even if the processor does not clear the display_bus_possession bit after receiving the interrupt for tearing effect, DSI-tx controller starts the activities on the DSI link once the TE trigger message is received"]
pub type DisplaybuspossessenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ready for programming after all count registers and timeout."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ULPS field of the DEVICEREADY register."]
    #[inline(always)]
    pub fn ulps(&self) -> UlpsR {
        UlpsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inform DSI receiver has to be given the bus possession for receiving the tearing effect trigger message; Reset by the processor to stop the bus possession of the DSI receiver; Note: Tearing effect is supported only in Type1; Display Architecture (command mode only) as suggested by Display Command Set Specification; Note1: Even if the processor does not clear the display_bus_possession bit after receiving the interrupt for tearing effect, DSI-tx controller starts the activities on the DSI link once the TE trigger message is received"]
    #[inline(always)]
    pub fn displaybuspossessen(&self) -> DisplaybuspossessenR {
        DisplaybuspossessenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready for programming after all count registers and timeout."]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> ReadyW<DevicereadySpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ULPS field of the DEVICEREADY register."]
    #[inline(always)]
    #[must_use]
    pub fn ulps(&mut self) -> UlpsW<DevicereadySpec> {
        UlpsW::new(self, 1)
    }
    #[doc = "Bit 3 - Inform DSI receiver has to be given the bus possession for receiving the tearing effect trigger message; Reset by the processor to stop the bus possession of the DSI receiver; Note: Tearing effect is supported only in Type1; Display Architecture (command mode only) as suggested by Display Command Set Specification; Note1: Even if the processor does not clear the display_bus_possession bit after receiving the interrupt for tearing effect, DSI-tx controller starts the activities on the DSI link once the TE trigger message is received"]
    #[inline(always)]
    #[must_use]
    pub fn displaybuspossessen(&mut self) -> DisplaybuspossessenW<DevicereadySpec> {
        DisplaybuspossessenW::new(self, 3)
    }
}
#[doc = "Devide Ready register\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deviceready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicereadySpec;
impl crate::RegisterSpec for DevicereadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceready::R`](R) reader structure"]
impl crate::Readable for DevicereadySpec {}
#[doc = "`write(|w| ..)` method takes [`deviceready::W`](W) writer structure"]
impl crate::Writable for DevicereadySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVICEREADY to value 0"]
impl crate::Resettable for DevicereadySpec {
    const RESET_VALUE: u32 = 0;
}
