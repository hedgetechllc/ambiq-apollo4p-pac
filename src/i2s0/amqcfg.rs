#[doc = "Register `AMQCFG` reader"]
pub type R = crate::R<AmqcfgSpec>;
#[doc = "Register `AMQCFG` writer"]
pub type W = crate::W<AmqcfgSpec>;
#[doc = "Field `MCLKSRC` reader - MCLK source. 1: Output of nco_clk divider. 0: MCLK from ambiq clock configuration directly"]
pub type MclksrcR = crate::BitReader;
#[doc = "Field `MCLKSRC` writer - MCLK source. 1: Output of nco_clk divider. 0: MCLK from ambiq clock configuration directly"]
pub type MclksrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASRCEN` reader - ASRC sub module enable. 0: Enabled. 1: Disabled/Bypassed"]
pub type AsrcenR = crate::BitReader;
#[doc = "Field `ASRCEN` writer - ASRC sub module enable. 0: Enabled. 1: Disabled/Bypassed"]
pub type AsrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCLK source. 1: Output of nco_clk divider. 0: MCLK from ambiq clock configuration directly"]
    #[inline(always)]
    pub fn mclksrc(&self) -> MclksrcR {
        MclksrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASRC sub module enable. 0: Enabled. 1: Disabled/Bypassed"]
    #[inline(always)]
    pub fn asrcen(&self) -> AsrcenR {
        AsrcenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK source. 1: Output of nco_clk divider. 0: MCLK from ambiq clock configuration directly"]
    #[inline(always)]
    #[must_use]
    pub fn mclksrc(&mut self) -> MclksrcW<AmqcfgSpec> {
        MclksrcW::new(self, 0)
    }
    #[doc = "Bit 1 - ASRC sub module enable. 0: Enabled. 1: Disabled/Bypassed"]
    #[inline(always)]
    #[must_use]
    pub fn asrcen(&mut self) -> AsrcenW<AmqcfgSpec> {
        AsrcenW::new(self, 1)
    }
}
#[doc = "Control the enablement of the ASRC module and the source of the MCLK used in the IPB core.\n\nYou can [`read`](crate::Reg::read) this register and get [`amqcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amqcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmqcfgSpec;
impl crate::RegisterSpec for AmqcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amqcfg::R`](R) reader structure"]
impl crate::Readable for AmqcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`amqcfg::W`](W) writer structure"]
impl crate::Writable for AmqcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMQCFG to value 0"]
impl crate::Resettable for AmqcfgSpec {
    const RESET_VALUE: u32 = 0;
}
