#[doc = "Register `HORIZBKPORCHCNT` reader"]
pub type R = crate::R<HorizbkporchcntSpec>;
#[doc = "Register `HORIZBKPORCHCNT` writer"]
pub type W = crate::W<HorizbkporchcntSpec>;
#[doc = "Field `HORZBKPCNT` reader - For Non Burst Sync pulse mode, for one lane. Minimum HBP count = Hsync End short packet + HBP Blanking packet overhead (header(4) + crc (2)) + RGB packet header For other lane counts minimum value = Minimum HBPcount / lane_count. For Non Burst Sync event / Burst Mode there is no HSA. Minimum HBP count = (Hsync Start short packet + HBP Blanking packet overhead + RGB packet header) / lane_count Min value - 14 in decimal (accounted with zero payloads for blanking packet\\]
for one lane. Max value - any 12 bit value greater than 14 based on DPI resolution"]
pub type HorzbkpcntR = crate::FieldReader<u16>;
#[doc = "Field `HORZBKPCNT` writer - For Non Burst Sync pulse mode, for one lane. Minimum HBP count = Hsync End short packet + HBP Blanking packet overhead (header(4) + crc (2)) + RGB packet header For other lane counts minimum value = Minimum HBPcount / lane_count. For Non Burst Sync event / Burst Mode there is no HSA. Minimum HBP count = (Hsync Start short packet + HBP Blanking packet overhead + RGB packet header) / lane_count Min value - 14 in decimal (accounted with zero payloads for blanking packet\\]
for one lane. Max value - any 12 bit value greater than 14 based on DPI resolution"]
pub type HorzbkpcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - For Non Burst Sync pulse mode, for one lane. Minimum HBP count = Hsync End short packet + HBP Blanking packet overhead (header(4) + crc (2)) + RGB packet header For other lane counts minimum value = Minimum HBPcount / lane_count. For Non Burst Sync event / Burst Mode there is no HSA. Minimum HBP count = (Hsync Start short packet + HBP Blanking packet overhead + RGB packet header) / lane_count Min value - 14 in decimal (accounted with zero payloads for blanking packet\\]
for one lane. Max value - any 12 bit value greater than 14 based on DPI resolution"]
    #[inline(always)]
    pub fn horzbkpcnt(&self) -> HorzbkpcntR {
        HorzbkpcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - For Non Burst Sync pulse mode, for one lane. Minimum HBP count = Hsync End short packet + HBP Blanking packet overhead (header(4) + crc (2)) + RGB packet header For other lane counts minimum value = Minimum HBPcount / lane_count. For Non Burst Sync event / Burst Mode there is no HSA. Minimum HBP count = (Hsync Start short packet + HBP Blanking packet overhead + RGB packet header) / lane_count Min value - 14 in decimal (accounted with zero payloads for blanking packet\\]
for one lane. Max value - any 12 bit value greater than 14 based on DPI resolution"]
    #[inline(always)]
    #[must_use]
    pub fn horzbkpcnt(&mut self) -> HorzbkpcntW<HorizbkporchcntSpec> {
        HorzbkpcntW::new(self, 0)
    }
}
#[doc = "Shows the horizontal back porch value in terms of txbyteclkhs.\n\nYou can [`read`](crate::Reg::read) this register and get [`horizbkporchcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horizbkporchcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HorizbkporchcntSpec;
impl crate::RegisterSpec for HorizbkporchcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`horizbkporchcnt::R`](R) reader structure"]
impl crate::Readable for HorizbkporchcntSpec {}
#[doc = "`write(|w| ..)` method takes [`horizbkporchcnt::W`](W) writer structure"]
impl crate::Writable for HorizbkporchcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HORIZBKPORCHCNT to value 0"]
impl crate::Resettable for HorizbkporchcntSpec {
    const RESET_VALUE: u32 = 0;
}
