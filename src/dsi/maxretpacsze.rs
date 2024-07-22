#[doc = "Register `MAXRETPACSZE` reader"]
pub type R = crate::R<MaxretpacszeSpec>;
#[doc = "Register `MAXRETPACSZE` writer"]
pub type W = crate::W<MaxretpacszeSpec>;
#[doc = "Field `COUNTVAL` reader - Set the count value in bytes to collect the return data packet for reverse direction data flow in data lane0 in response to a DBI read operation; Count value equals the maximum size of the payload in a Long packet transmitted from peripheral back to; for DBI and DPI interleaving Min value - 1; Max value - Maximum payload for a long packet size is 1K bytes Note: DCS short Read Response or Long read response with 1 or 2 parameters is applicable in this mode; For DBI only, Min value - 1 Max value - Maximum payload for a long packet size is 1K bytes"]
pub type CountvalR = crate::FieldReader<u16>;
#[doc = "Field `COUNTVAL` writer - Set the count value in bytes to collect the return data packet for reverse direction data flow in data lane0 in response to a DBI read operation; Count value equals the maximum size of the payload in a Long packet transmitted from peripheral back to; for DBI and DPI interleaving Min value - 1; Max value - Maximum payload for a long packet size is 1K bytes Note: DCS short Read Response or Long read response with 1 or 2 parameters is applicable in this mode; For DBI only, Min value - 1 Max value - Maximum payload for a long packet size is 1K bytes"]
pub type CountvalW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `HSLP` reader - Indicates the data transfer type"]
pub type HslpR = crate::BitReader;
#[doc = "Field `HSLP` writer - Indicates the data transfer type"]
pub type HslpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Set the count value in bytes to collect the return data packet for reverse direction data flow in data lane0 in response to a DBI read operation; Count value equals the maximum size of the payload in a Long packet transmitted from peripheral back to; for DBI and DPI interleaving Min value - 1; Max value - Maximum payload for a long packet size is 1K bytes Note: DCS short Read Response or Long read response with 1 or 2 parameters is applicable in this mode; For DBI only, Min value - 1 Max value - Maximum payload for a long packet size is 1K bytes"]
    #[inline(always)]
    pub fn countval(&self) -> CountvalR {
        CountvalR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Indicates the data transfer type"]
    #[inline(always)]
    pub fn hslp(&self) -> HslpR {
        HslpR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Set the count value in bytes to collect the return data packet for reverse direction data flow in data lane0 in response to a DBI read operation; Count value equals the maximum size of the payload in a Long packet transmitted from peripheral back to; for DBI and DPI interleaving Min value - 1; Max value - Maximum payload for a long packet size is 1K bytes Note: DCS short Read Response or Long read response with 1 or 2 parameters is applicable in this mode; For DBI only, Min value - 1 Max value - Maximum payload for a long packet size is 1K bytes"]
    #[inline(always)]
    #[must_use]
    pub fn countval(&mut self) -> CountvalW<MaxretpacszeSpec> {
        CountvalW::new(self, 0)
    }
    #[doc = "Bit 15 - Indicates the data transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn hslp(&mut self) -> HslpW<MaxretpacszeSpec> {
        HslpW::new(self, 15)
    }
}
#[doc = "MAXRETPACSZE register description needed here.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxretpacsze::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxretpacsze::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxretpacszeSpec;
impl crate::RegisterSpec for MaxretpacszeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxretpacsze::R`](R) reader structure"]
impl crate::Readable for MaxretpacszeSpec {}
#[doc = "`write(|w| ..)` method takes [`maxretpacsze::W`](W) writer structure"]
impl crate::Writable for MaxretpacszeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXRETPACSZE to value 0"]
impl crate::Resettable for MaxretpacszeSpec {
    const RESET_VALUE: u32 = 0;
}
