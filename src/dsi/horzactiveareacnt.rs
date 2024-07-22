#[doc = "Register `HORZACTIVEAREACNT` reader"]
pub type R = crate::R<HorzactiveareacntSpec>;
#[doc = "Register `HORZACTIVEAREACNT` writer"]
pub type W = crate::W<HorzactiveareacntSpec>;
#[doc = "Field `HORACTCNT` reader - Shows the horizontal active area value in terms of txbyteclkhs. In Non Burst Mode, Count equal to RGB word count value In Burst Mode, RGB pixel packets are time compressed, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link. Hence, the count equals the time in txbyteclkhs for sending time compressed RGB pixels plus the time needed for moving to power save mode or the time needed for secondary channel to use the DSI link. But if the left out time for moving to low power mode is less than 8 txbyteclkhs (2txbyteclkhs for RGB data packet footer and 6txbyteclkhs for a blanking packet with zero payload), then this count will be added to the HFPs count for one lane."]
pub type HoractcntR = crate::FieldReader<u16>;
#[doc = "Field `HORACTCNT` writer - Shows the horizontal active area value in terms of txbyteclkhs. In Non Burst Mode, Count equal to RGB word count value In Burst Mode, RGB pixel packets are time compressed, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link. Hence, the count equals the time in txbyteclkhs for sending time compressed RGB pixels plus the time needed for moving to power save mode or the time needed for secondary channel to use the DSI link. But if the left out time for moving to low power mode is less than 8 txbyteclkhs (2txbyteclkhs for RGB data packet footer and 6txbyteclkhs for a blanking packet with zero payload), then this count will be added to the HFPs count for one lane."]
pub type HoractcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the horizontal active area value in terms of txbyteclkhs. In Non Burst Mode, Count equal to RGB word count value In Burst Mode, RGB pixel packets are time compressed, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link. Hence, the count equals the time in txbyteclkhs for sending time compressed RGB pixels plus the time needed for moving to power save mode or the time needed for secondary channel to use the DSI link. But if the left out time for moving to low power mode is less than 8 txbyteclkhs (2txbyteclkhs for RGB data packet footer and 6txbyteclkhs for a blanking packet with zero payload), then this count will be added to the HFPs count for one lane."]
    #[inline(always)]
    pub fn horactcnt(&self) -> HoractcntR {
        HoractcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the horizontal active area value in terms of txbyteclkhs. In Non Burst Mode, Count equal to RGB word count value In Burst Mode, RGB pixel packets are time compressed, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link. Hence, the count equals the time in txbyteclkhs for sending time compressed RGB pixels plus the time needed for moving to power save mode or the time needed for secondary channel to use the DSI link. But if the left out time for moving to low power mode is less than 8 txbyteclkhs (2txbyteclkhs for RGB data packet footer and 6txbyteclkhs for a blanking packet with zero payload), then this count will be added to the HFPs count for one lane."]
    #[inline(always)]
    #[must_use]
    pub fn horactcnt(&mut self) -> HoractcntW<HorzactiveareacntSpec> {
        HoractcntW::new(self, 0)
    }
}
#[doc = "Horizontal active area count / time for active image data / Horizontal Address\n\nYou can [`read`](crate::Reg::read) this register and get [`horzactiveareacnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`horzactiveareacnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HorzactiveareacntSpec;
impl crate::RegisterSpec for HorzactiveareacntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`horzactiveareacnt::R`](R) reader structure"]
impl crate::Readable for HorzactiveareacntSpec {}
#[doc = "`write(|w| ..)` method takes [`horzactiveareacnt::W`](W) writer structure"]
impl crate::Writable for HorzactiveareacntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HORZACTIVEAREACNT to value 0"]
impl crate::Resettable for HorzactiveareacntSpec {
    const RESET_VALUE: u32 = 0;
}
