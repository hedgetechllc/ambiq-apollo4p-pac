#[doc = "Register `SCWLIM` reader"]
pub type R = crate::R<ScwlimSpec>;
#[doc = "Register `SCWLIM` writer"]
pub type W = crate::W<ScwlimSpec>;
#[doc = "Field `SCWLIMEN` reader - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
pub type ScwlimenR = crate::BitReader;
#[doc = "Field `SCWLIMEN` writer - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
pub type ScwlimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub fn scwlimen(&self) -> ScwlimenR {
        ScwlimenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    #[must_use]
    pub fn scwlimen(&mut self) -> ScwlimenW<ScwlimSpec> {
        ScwlimenW::new(self, 0)
    }
}
#[doc = "Scale Window Comparator Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`scwlim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scwlim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScwlimSpec;
impl crate::RegisterSpec for ScwlimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scwlim::R`](R) reader structure"]
impl crate::Readable for ScwlimSpec {}
#[doc = "`write(|w| ..)` method takes [`scwlim::W`](W) writer structure"]
impl crate::Writable for ScwlimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCWLIM to value 0"]
impl crate::Resettable for ScwlimSpec {
    const RESET_VALUE: u32 = 0;
}
