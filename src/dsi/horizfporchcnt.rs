#[doc = "Register `HORIZFPORCHCNT` reader"]
pub type R = crate::R<HorizfporchcntSpec>;
#[doc = "Register `HORIZFPORCHCNT` writer"]
pub type W = crate::W<HorizfporchcntSpec>;
#[doc = "Field `HORZFTPCNT` reader - Minimum HFP period should be sufficient to transmit RGB Data packet footer (2 bytes) + Blanking packet overhead (6 bytes) +adjustable count (16 bytes) for non burst mode; For other lane counts Minimum value = (RGB Data packet footer(2 bytes) + Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) For burst mode, Minimum HFP period should be sufficient to transmit Blanking packet overhead(6 bytes) +adjustable count (16 bytes) for one lane for other lane counts Minimum value = ( Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) Min value - 8 in decimal for non-burst mode (accounted with zero payload for blanking packet) for one lane Min value - 6 in decimal for burst mode for one lane Max value - any 12 bit value greater than the minimum value based on DPI resolution; Minimum HFP period should be sufficient to transmit"]
pub type HorzftpcntR = crate::FieldReader<u16>;
#[doc = "Field `HORZFTPCNT` writer - Minimum HFP period should be sufficient to transmit RGB Data packet footer (2 bytes) + Blanking packet overhead (6 bytes) +adjustable count (16 bytes) for non burst mode; For other lane counts Minimum value = (RGB Data packet footer(2 bytes) + Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) For burst mode, Minimum HFP period should be sufficient to transmit Blanking packet overhead(6 bytes) +adjustable count (16 bytes) for one lane for other lane counts Minimum value = ( Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) Min value - 8 in decimal for non-burst mode (accounted with zero payload for blanking packet) for one lane Min value - 6 in decimal for burst mode for one lane Max value - any 12 bit value greater than the minimum value based on DPI resolution; Minimum HFP period should be sufficient to transmit"]
pub type HorzftpcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum HFP period should be sufficient to transmit RGB Data packet footer (2 bytes) + Blanking packet overhead (6 bytes) +adjustable count (16 bytes) for non burst mode; For other lane counts Minimum value = (RGB Data packet footer(2 bytes) + Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) For burst mode, Minimum HFP period should be sufficient to transmit Blanking packet overhead(6 bytes) +adjustable count (16 bytes) for one lane for other lane counts Minimum value = ( Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) Min value - 8 in decimal for non-burst mode (accounted with zero payload for blanking packet) for one lane Min value - 6 in decimal for burst mode for one lane Max value - any 12 bit value greater than the minimum value based on DPI resolution; Minimum HFP period should be sufficient to transmit"]
    #[inline(always)]
    pub fn horzftpcnt(&self) -> HorzftpcntR {
        HorzftpcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum HFP period should be sufficient to transmit RGB Data packet footer (2 bytes) + Blanking packet overhead (6 bytes) +adjustable count (16 bytes) for non burst mode; For other lane counts Minimum value = (RGB Data packet footer(2 bytes) + Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) For burst mode, Minimum HFP period should be sufficient to transmit Blanking packet overhead(6 bytes) +adjustable count (16 bytes) for one lane for other lane counts Minimum value = ( Blanking packet overhead(6 bytes)) / (lane_count) + adjustable count(16 bytes) Min value - 8 in decimal for non-burst mode (accounted with zero payload for blanking packet) for one lane Min value - 6 in decimal for burst mode for one lane Max value - any 12 bit value greater than the minimum value based on DPI resolution; Minimum HFP period should be sufficient to transmit"]
    #[inline(always)]
    #[must_use]
    pub fn horzftpcnt(&mut self) -> HorzftpcntW<HorizfporchcntSpec> {
        HorzftpcntW::new(self, 0)
    }
}
#[doc = "Shows the horizontal front porch value in terms of txbyteclkhs.\n\nYou can [`read`](crate::Reg::read) this register and get [`horizfporchcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horizfporchcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HorizfporchcntSpec;
impl crate::RegisterSpec for HorizfporchcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`horizfporchcnt::R`](R) reader structure"]
impl crate::Readable for HorizfporchcntSpec {}
#[doc = "`write(|w| ..)` method takes [`horizfporchcnt::W`](W) writer structure"]
impl crate::Writable for HorizfporchcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HORIZFPORCHCNT to value 0"]
impl crate::Resettable for HorizfporchcntSpec {
    const RESET_VALUE: u32 = 0;
}
