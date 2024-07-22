#[doc = "Register `D2ASPARE` reader"]
pub type R = crate::R<D2aspareSpec>;
#[doc = "Register `D2ASPARE` writer"]
pub type W = crate::W<D2aspareSpec>;
#[doc = "Field `VDDCPUOVERRIDE` reader - VDDCPU Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCPUOVERRIDE=1 must be used."]
pub type VddcpuoverrideR = crate::BitReader;
#[doc = "Field `VDDCPUOVERRIDE` writer - VDDCPU Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCPUOVERRIDE=1 must be used."]
pub type VddcpuoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDCAOROVERRIDE` reader - VDDCAOR Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCAOROVERRIDE=1 must be used."]
pub type VddcaoroverrideR = crate::BitReader;
#[doc = "Field `VDDCAOROVERRIDE` writer - VDDCAOR Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCAOROVERRIDE=1 must be used."]
pub type VddcaoroverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - VDDCPU Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCPUOVERRIDE=1 must be used."]
    #[inline(always)]
    pub fn vddcpuoverride(&self) -> VddcpuoverrideR {
        VddcpuoverrideR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDCAOR Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCAOROVERRIDE=1 must be used."]
    #[inline(always)]
    pub fn vddcaoroverride(&self) -> VddcaoroverrideR {
        VddcaoroverrideR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - VDDCPU Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCPUOVERRIDE=1 must be used."]
    #[inline(always)]
    #[must_use]
    pub fn vddcpuoverride(&mut self) -> VddcpuoverrideW<D2aspareSpec> {
        VddcpuoverrideW::new(self, 3)
    }
    #[doc = "Bit 4 - VDDCAOR Override. Set to 1 to connect to the VDDC rail, set to 0 to connect to the VDDC_LV rail. Before setting this bit to 0, the VDDC_LV rail must first be enabled by setting SIMOBUCK0_b.VDDCLVRXCOMPEN. Do not modify this field unless directed to do so by Ambiq engineering. If modifying, a RMW operation such as MCUCTRL->D2ASPARE_b.VDDCAOROVERRIDE=1 must be used."]
    #[inline(always)]
    #[must_use]
    pub fn vddcaoroverride(&mut self) -> VddcaoroverrideW<D2aspareSpec> {
        VddcaoroverrideW::new(self, 4)
    }
}
#[doc = "Spare registers to analog module\n\nYou can [`read`](crate::Reg::read) this register and get [`d2aspare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2aspare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2aspareSpec;
impl crate::RegisterSpec for D2aspareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2aspare::R`](R) reader structure"]
impl crate::Readable for D2aspareSpec {}
#[doc = "`write(|w| ..)` method takes [`d2aspare::W`](W) writer structure"]
impl crate::Writable for D2aspareSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D2ASPARE to value 0"]
impl crate::Resettable for D2aspareSpec {
    const RESET_VALUE: u32 = 0;
}
