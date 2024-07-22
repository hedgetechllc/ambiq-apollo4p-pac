#[doc = "Register `VIDEOMODEFMT` reader"]
pub type R = crate::R<VideomodefmtSpec>;
#[doc = "Register `VIDEOMODEFMT` writer"]
pub type W = crate::W<VideomodefmtSpec>;
#[doc = "Sets the Video mode format (packet sequence) to be supported in DSI; in Non Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed equal to RGB word count value; in Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed greater than the RGB word count value, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link;\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Videmdfmt {
    #[doc = "0: VIDEMDFMT enum description needed here."]
    Videmdfmt0 = 0,
    #[doc = "1: Non Burst Mode with Sync Pulse"]
    Nonburstpulse = 1,
    #[doc = "2: Non Burst Mode with Sync events"]
    Nonburstevents = 2,
    #[doc = "3: MODE Burst Mode"]
    Burst = 3,
}
impl From<Videmdfmt> for u8 {
    #[inline(always)]
    fn from(variant: Videmdfmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Videmdfmt {
    type Ux = u8;
}
impl crate::IsEnum for Videmdfmt {}
#[doc = "Field `VIDEMDFMT` reader - Sets the Video mode format (packet sequence) to be supported in DSI; in Non Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed equal to RGB word count value; in Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed greater than the RGB word count value, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link;"]
pub type VidemdfmtR = crate::FieldReader<Videmdfmt>;
impl VidemdfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Videmdfmt {
        match self.bits {
            0 => Videmdfmt::Videmdfmt0,
            1 => Videmdfmt::Nonburstpulse,
            2 => Videmdfmt::Nonburstevents,
            3 => Videmdfmt::Burst,
            _ => unreachable!(),
        }
    }
    #[doc = "VIDEMDFMT enum description needed here."]
    #[inline(always)]
    pub fn is_videmdfmt_0(&self) -> bool {
        *self == Videmdfmt::Videmdfmt0
    }
    #[doc = "Non Burst Mode with Sync Pulse"]
    #[inline(always)]
    pub fn is_nonburstpulse(&self) -> bool {
        *self == Videmdfmt::Nonburstpulse
    }
    #[doc = "Non Burst Mode with Sync events"]
    #[inline(always)]
    pub fn is_nonburstevents(&self) -> bool {
        *self == Videmdfmt::Nonburstevents
    }
    #[doc = "MODE Burst Mode"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Videmdfmt::Burst
    }
}
#[doc = "Field `VIDEMDFMT` writer - Sets the Video mode format (packet sequence) to be supported in DSI; in Non Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed equal to RGB word count value; in Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed greater than the RGB word count value, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link;"]
pub type VidemdfmtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Videmdfmt, crate::Safe>;
impl<'a, REG> VidemdfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VIDEMDFMT enum description needed here."]
    #[inline(always)]
    pub fn videmdfmt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Videmdfmt::Videmdfmt0)
    }
    #[doc = "Non Burst Mode with Sync Pulse"]
    #[inline(always)]
    pub fn nonburstpulse(self) -> &'a mut crate::W<REG> {
        self.variant(Videmdfmt::Nonburstpulse)
    }
    #[doc = "Non Burst Mode with Sync events"]
    #[inline(always)]
    pub fn nonburstevents(self) -> &'a mut crate::W<REG> {
        self.variant(Videmdfmt::Nonburstevents)
    }
    #[doc = "MODE Burst Mode"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Videmdfmt::Burst)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets the Video mode format (packet sequence) to be supported in DSI; in Non Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed equal to RGB word count value; in Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed greater than the RGB word count value, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link;"]
    #[inline(always)]
    pub fn videmdfmt(&self) -> VidemdfmtR {
        VidemdfmtR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the Video mode format (packet sequence) to be supported in DSI; in Non Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed equal to RGB word count value; in Burst Mode, in addition to programming this register the horizontal active area count register value should also be programmed greater than the RGB word count value, leaving more time during a scan line for LP mode (saving power) or for multiplexing other transmissions onto the DSI link;"]
    #[inline(always)]
    #[must_use]
    pub fn videmdfmt(&mut self) -> VidemdfmtW<VideomodefmtSpec> {
        VidemdfmtW::new(self, 0)
    }
}
#[doc = "Sets the Video mode format (packet sequence) to be supported in DSI.\n\nYou can [`read`](crate::Reg::read) this register and get [`videomodefmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`videomodefmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideomodefmtSpec;
impl crate::RegisterSpec for VideomodefmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`videomodefmt::R`](R) reader structure"]
impl crate::Readable for VideomodefmtSpec {}
#[doc = "`write(|w| ..)` method takes [`videomodefmt::W`](W) writer structure"]
impl crate::Writable for VideomodefmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIDEOMODEFMT to value 0"]
impl crate::Resettable for VideomodefmtSpec {
    const RESET_VALUE: u32 = 0;
}
