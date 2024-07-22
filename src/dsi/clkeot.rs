#[doc = "Register `CLKEOT` reader"]
pub type R = crate::R<ClkeotSpec>;
#[doc = "Register `CLKEOT` writer"]
pub type W = crate::W<ClkeotSpec>;
#[doc = "Field `EOT` reader - Set by the processor to enable or disable EOT short disable_register packet transmission; vy default this register value is 0; for backward compatibility of earlier DSI systems, EOT short packet transmission can be disabled; 0 EOT short packet transmission enabled, 1 EOT short packet transmission disabled"]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - Set by the processor to enable or disable EOT short disable_register packet transmission; vy default this register value is 0; for backward compatibility of earlier DSI systems, EOT short packet transmission can be disabled; 0 EOT short packet transmission enabled, 1 EOT short packet transmission disabled"]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCK` reader - Set by the processor to enable or disable clock; Stopping feature during BLLP timing in a DPI transfer in dual channel mode or during DPI only mode and also when there is no traffic in the DBI interface in DBI only enabled mode. By default this register value is 0."]
pub type ClockR = crate::BitReader;
#[doc = "Field `CLOCK` writer - Set by the processor to enable or disable clock; Stopping feature during BLLP timing in a DPI transfer in dual channel mode or during DPI only mode and also when there is no traffic in the DBI interface in DBI only enabled mode. By default this register value is 0."]
pub type ClockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTA` reader - Disable video; Set by the processor to inform the DSI controller to disable the BTA sent at the last blanking line of VFP. By default, this bit is set to 0; 0 BTA sending at the last blanking line of VFP is enabled; 1 BTA sending at the last blanking line of VFP is disabled"]
pub type BtaR = crate::BitReader;
#[doc = "Field `BTA` writer - Disable video; Set by the processor to inform the DSI controller to disable the BTA sent at the last blanking line of VFP. By default, this bit is set to 0; 0 BTA sending at the last blanking line of VFP is enabled; 1 BTA sending at the last blanking line of VFP is disabled"]
pub type BtaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set by the processor to enable or disable EOT short disable_register packet transmission; vy default this register value is 0; for backward compatibility of earlier DSI systems, EOT short packet transmission can be disabled; 0 EOT short packet transmission enabled, 1 EOT short packet transmission disabled"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set by the processor to enable or disable clock; Stopping feature during BLLP timing in a DPI transfer in dual channel mode or during DPI only mode and also when there is no traffic in the DBI interface in DBI only enabled mode. By default this register value is 0."]
    #[inline(always)]
    pub fn clock(&self) -> ClockR {
        ClockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable video; Set by the processor to inform the DSI controller to disable the BTA sent at the last blanking line of VFP. By default, this bit is set to 0; 0 BTA sending at the last blanking line of VFP is enabled; 1 BTA sending at the last blanking line of VFP is disabled"]
    #[inline(always)]
    pub fn bta(&self) -> BtaR {
        BtaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set by the processor to enable or disable EOT short disable_register packet transmission; vy default this register value is 0; for backward compatibility of earlier DSI systems, EOT short packet transmission can be disabled; 0 EOT short packet transmission enabled, 1 EOT short packet transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EotW<ClkeotSpec> {
        EotW::new(self, 0)
    }
    #[doc = "Bit 1 - Set by the processor to enable or disable clock; Stopping feature during BLLP timing in a DPI transfer in dual channel mode or during DPI only mode and also when there is no traffic in the DBI interface in DBI only enabled mode. By default this register value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn clock(&mut self) -> ClockW<ClkeotSpec> {
        ClockW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable video; Set by the processor to inform the DSI controller to disable the BTA sent at the last blanking line of VFP. By default, this bit is set to 0; 0 BTA sending at the last blanking line of VFP is enabled; 1 BTA sending at the last blanking line of VFP is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn bta(&mut self) -> BtaW<ClkeotSpec> {
        BtaW::new(self, 2)
    }
}
#[doc = "The EOT clock register disables the video.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkeot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkeot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkeotSpec;
impl crate::RegisterSpec for ClkeotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkeot::R`](R) reader structure"]
impl crate::Readable for ClkeotSpec {}
#[doc = "`write(|w| ..)` method takes [`clkeot::W`](W) writer structure"]
impl crate::Writable for ClkeotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKEOT to value 0"]
impl crate::Resettable for ClkeotSpec {
    const RESET_VALUE: u32 = 0;
}
