#[doc = "Register `VENDOR` reader"]
pub type R = crate::R<VendorSpec>;
#[doc = "Register `VENDOR` writer"]
pub type W = crate::W<VendorSpec>;
#[doc = "If this bit is 0, SD_CLK to card will not be gated automatically, when there is no transfer. If this bit set to 1, SD_CLK to card will be gated automatically,when there is no transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gatesdclken {
    #[doc = "1: SD_CLK to card will be gated automatically when there is no transfer."]
    Gate = 1,
    #[doc = "0: SD_CLK to card will NOT be gated automatically when there is no transfer."]
    Nogate = 0,
}
impl From<Gatesdclken> for bool {
    #[inline(always)]
    fn from(variant: Gatesdclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATESDCLKEN` reader - If this bit is 0, SD_CLK to card will not be gated automatically, when there is no transfer. If this bit set to 1, SD_CLK to card will be gated automatically,when there is no transfer."]
pub type GatesdclkenR = crate::BitReader<Gatesdclken>;
impl GatesdclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gatesdclken {
        match self.bits {
            true => Gatesdclken::Gate,
            false => Gatesdclken::Nogate,
        }
    }
    #[doc = "SD_CLK to card will be gated automatically when there is no transfer."]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == Gatesdclken::Gate
    }
    #[doc = "SD_CLK to card will NOT be gated automatically when there is no transfer."]
    #[inline(always)]
    pub fn is_nogate(&self) -> bool {
        *self == Gatesdclken::Nogate
    }
}
#[doc = "Field `GATESDCLKEN` writer - If this bit is 0, SD_CLK to card will not be gated automatically, when there is no transfer. If this bit set to 1, SD_CLK to card will be gated automatically,when there is no transfer."]
pub type GatesdclkenW<'a, REG> = crate::BitWriter<'a, REG, Gatesdclken>;
impl<'a, REG> GatesdclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SD_CLK to card will be gated automatically when there is no transfer."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut crate::W<REG> {
        self.variant(Gatesdclken::Gate)
    }
    #[doc = "SD_CLK to card will NOT be gated automatically when there is no transfer."]
    #[inline(always)]
    pub fn nogate(self) -> &'a mut crate::W<REG> {
        self.variant(Gatesdclken::Nogate)
    }
}
#[doc = "Chicken bit added to enable/disable the rtl fix made to delay the sampling of cmd_in and data_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlydis {
    #[doc = "1: Disable the rtl fix made to delay the sampling of cmd_in and data_in"]
    Disable = 1,
    #[doc = "0: Enable the rtl fix made to delay the sampling of cmd_in and data_in"]
    Enable = 0,
}
impl From<Dlydis> for bool {
    #[inline(always)]
    fn from(variant: Dlydis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYDIS` reader - Chicken bit added to enable/disable the rtl fix made to delay the sampling of cmd_in and data_in."]
pub type DlydisR = crate::BitReader<Dlydis>;
impl DlydisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlydis {
        match self.bits {
            true => Dlydis::Disable,
            false => Dlydis::Enable,
        }
    }
    #[doc = "Disable the rtl fix made to delay the sampling of cmd_in and data_in"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dlydis::Disable
    }
    #[doc = "Enable the rtl fix made to delay the sampling of cmd_in and data_in"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dlydis::Enable
    }
}
#[doc = "Field `DLYDIS` writer - Chicken bit added to enable/disable the rtl fix made to delay the sampling of cmd_in and data_in."]
pub type DlydisW<'a, REG> = crate::BitWriter<'a, REG, Dlydis>;
impl<'a, REG> DlydisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the rtl fix made to delay the sampling of cmd_in and data_in"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dlydis::Disable)
    }
    #[doc = "Enable the rtl fix made to delay the sampling of cmd_in and data_in"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dlydis::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is 0, SD_CLK to card will not be gated automatically, when there is no transfer. If this bit set to 1, SD_CLK to card will be gated automatically,when there is no transfer."]
    #[inline(always)]
    pub fn gatesdclken(&self) -> GatesdclkenR {
        GatesdclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Chicken bit added to enable/disable the rtl fix made to delay the sampling of cmd_in and data_in."]
    #[inline(always)]
    pub fn dlydis(&self) -> DlydisR {
        DlydisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is 0, SD_CLK to card will not be gated automatically, when there is no transfer. If this bit set to 1, SD_CLK to card will be gated automatically,when there is no transfer."]
    #[inline(always)]
    #[must_use]
    pub fn gatesdclken(&mut self) -> GatesdclkenW<VendorSpec> {
        GatesdclkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Chicken bit added to enable/disable the rtl fix made to delay the sampling of cmd_in and data_in."]
    #[inline(always)]
    #[must_use]
    pub fn dlydis(&mut self) -> DlydisW<VendorSpec> {
        DlydisW::new(self, 1)
    }
}
#[doc = "Vendor\n\nYou can [`read`](crate::Reg::read) this register and get [`vendor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendorSpec;
impl crate::RegisterSpec for VendorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendor::R`](R) reader structure"]
impl crate::Readable for VendorSpec {}
#[doc = "`write(|w| ..)` method takes [`vendor::W`](W) writer structure"]
impl crate::Writable for VendorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VENDOR to value 0"]
impl crate::Resettable for VendorSpec {
    const RESET_VALUE: u32 = 0;
}
