#[doc = "Register `DINFIFORSTPNTR` reader"]
pub type R = crate::R<DinfiforstpntrSpec>;
#[doc = "Register `DINFIFORSTPNTR` writer"]
pub type W = crate::W<DinfiforstpntrSpec>;
#[doc = "Field `RST` reader - Writing any value to this address resets the DIN_FIFO pointers."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Writing any value to this address resets the DIN_FIFO pointers."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing any value to this address resets the DIN_FIFO pointers."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the DIN_FIFO pointers."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<DinfiforstpntrSpec> {
        RstW::new(self, 0)
    }
}
#[doc = "Writing to this register resets the DIN_FIFO pointers.\n\nYou can [`read`](crate::Reg::read) this register and get [`dinfiforstpntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinfiforstpntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DinfiforstpntrSpec;
impl crate::RegisterSpec for DinfiforstpntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dinfiforstpntr::R`](R) reader structure"]
impl crate::Readable for DinfiforstpntrSpec {}
#[doc = "`write(|w| ..)` method takes [`dinfiforstpntr::W`](W) writer structure"]
impl crate::Writable for DinfiforstpntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DINFIFORSTPNTR to value 0x01"]
impl crate::Resettable for DinfiforstpntrSpec {
    const RESET_VALUE: u32 = 0x01;
}
