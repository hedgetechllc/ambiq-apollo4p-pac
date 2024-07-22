#[doc = "Register `HSYNCCNT` reader"]
pub type R = crate::R<HsynccntSpec>;
#[doc = "Register `HSYNCCNT` writer"]
pub type W = crate::W<HsynccntSpec>;
#[doc = "Field `HORZCNT` reader - Shows the horizontal sync value in terms of byte clock (txbyteclkhs); Minimum HSA period should be sufficient to transmit a Hsync start short packet(4 bytes) i) For Non-burst Mode with sync pulse, Min value - 4 in decimal (plus an optional 6 bytes for a zero payload blanking packet); But if the value is less than 10 but more than 4, then this count will be added to the HBP's count for one lane; ii) For Non-Burst Sync Event and Burst Mode, there is no HSA, so you can program this to zero. If you program this register, these byte values will be added to HBP Max value - any 12 bit value greater than 4 based on DPI resolution"]
pub type HorzcntR = crate::FieldReader<u16>;
#[doc = "Field `HORZCNT` writer - Shows the horizontal sync value in terms of byte clock (txbyteclkhs); Minimum HSA period should be sufficient to transmit a Hsync start short packet(4 bytes) i) For Non-burst Mode with sync pulse, Min value - 4 in decimal (plus an optional 6 bytes for a zero payload blanking packet); But if the value is less than 10 but more than 4, then this count will be added to the HBP's count for one lane; ii) For Non-Burst Sync Event and Burst Mode, there is no HSA, so you can program this to zero. If you program this register, these byte values will be added to HBP Max value - any 12 bit value greater than 4 based on DPI resolution"]
pub type HorzcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the horizontal sync value in terms of byte clock (txbyteclkhs); Minimum HSA period should be sufficient to transmit a Hsync start short packet(4 bytes) i) For Non-burst Mode with sync pulse, Min value - 4 in decimal (plus an optional 6 bytes for a zero payload blanking packet); But if the value is less than 10 but more than 4, then this count will be added to the HBP's count for one lane; ii) For Non-Burst Sync Event and Burst Mode, there is no HSA, so you can program this to zero. If you program this register, these byte values will be added to HBP Max value - any 12 bit value greater than 4 based on DPI resolution"]
    #[inline(always)]
    pub fn horzcnt(&self) -> HorzcntR {
        HorzcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the horizontal sync value in terms of byte clock (txbyteclkhs); Minimum HSA period should be sufficient to transmit a Hsync start short packet(4 bytes) i) For Non-burst Mode with sync pulse, Min value - 4 in decimal (plus an optional 6 bytes for a zero payload blanking packet); But if the value is less than 10 but more than 4, then this count will be added to the HBP's count for one lane; ii) For Non-Burst Sync Event and Burst Mode, there is no HSA, so you can program this to zero. If you program this register, these byte values will be added to HBP Max value - any 12 bit value greater than 4 based on DPI resolution"]
    #[inline(always)]
    #[must_use]
    pub fn horzcnt(&mut self) -> HorzcntW<HsynccntSpec> {
        HorzcntW::new(self, 0)
    }
}
#[doc = "Shows the horizontal sync value in terms of byte clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsynccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsynccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsynccntSpec;
impl crate::RegisterSpec for HsynccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsynccnt::R`](R) reader structure"]
impl crate::Readable for HsynccntSpec {}
#[doc = "`write(|w| ..)` method takes [`hsynccnt::W`](W) writer structure"]
impl crate::Writable for HsynccntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSYNCCNT to value 0"]
impl crate::Resettable for HsynccntSpec {
    const RESET_VALUE: u32 = 0;
}
