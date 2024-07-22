#[doc = "Register `UTMISTICKYSTATUS` reader"]
pub type R = crate::R<UtmistickystatusSpec>;
#[doc = "Register `UTMISTICKYSTATUS` writer"]
pub type W = crate::W<UtmistickystatusSpec>;
#[doc = "These bits are read only status bits from the PHY OBS port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Obsportstciky {
    #[doc = "3: bit 1:HS BIST results, bit 0:FS BIST results"]
    Obs3 = 3,
    #[doc = "2: bit 1: ODT calibration state, bit 0: Current calibration state"]
    Obs2 = 2,
    #[doc = "1: bit 1: Rx squelch signal, bit 0: Rx datap"]
    Obs1 = 1,
    #[doc = "0: bit 1: PLL lock signal, bit 0: Host Disconnect"]
    Obs0 = 0,
}
impl From<Obsportstciky> for u8 {
    #[inline(always)]
    fn from(variant: Obsportstciky) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Obsportstciky {
    type Ux = u8;
}
impl crate::IsEnum for Obsportstciky {}
#[doc = "Field `obsportstciky` reader - These bits are read only status bits from the PHY OBS port"]
pub type ObsportstcikyR = crate::FieldReader<Obsportstciky>;
impl ObsportstcikyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Obsportstciky {
        match self.bits {
            3 => Obsportstciky::Obs3,
            2 => Obsportstciky::Obs2,
            1 => Obsportstciky::Obs1,
            0 => Obsportstciky::Obs0,
            _ => unreachable!(),
        }
    }
    #[doc = "bit 1:HS BIST results, bit 0:FS BIST results"]
    #[inline(always)]
    pub fn is_obs3(&self) -> bool {
        *self == Obsportstciky::Obs3
    }
    #[doc = "bit 1: ODT calibration state, bit 0: Current calibration state"]
    #[inline(always)]
    pub fn is_obs2(&self) -> bool {
        *self == Obsportstciky::Obs2
    }
    #[doc = "bit 1: Rx squelch signal, bit 0: Rx datap"]
    #[inline(always)]
    pub fn is_obs1(&self) -> bool {
        *self == Obsportstciky::Obs1
    }
    #[doc = "bit 1: PLL lock signal, bit 0: Host Disconnect"]
    #[inline(always)]
    pub fn is_obs0(&self) -> bool {
        *self == Obsportstciky::Obs0
    }
}
#[doc = "Field `obsportstciky` writer - These bits are read only status bits from the PHY OBS port"]
pub type ObsportstcikyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Obsportstciky, crate::Safe>;
impl<'a, REG> ObsportstcikyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bit 1:HS BIST results, bit 0:FS BIST results"]
    #[inline(always)]
    pub fn obs3(self) -> &'a mut crate::W<REG> {
        self.variant(Obsportstciky::Obs3)
    }
    #[doc = "bit 1: ODT calibration state, bit 0: Current calibration state"]
    #[inline(always)]
    pub fn obs2(self) -> &'a mut crate::W<REG> {
        self.variant(Obsportstciky::Obs2)
    }
    #[doc = "bit 1: Rx squelch signal, bit 0: Rx datap"]
    #[inline(always)]
    pub fn obs1(self) -> &'a mut crate::W<REG> {
        self.variant(Obsportstciky::Obs1)
    }
    #[doc = "bit 1: PLL lock signal, bit 0: Host Disconnect"]
    #[inline(always)]
    pub fn obs0(self) -> &'a mut crate::W<REG> {
        self.variant(Obsportstciky::Obs0)
    }
}
impl R {
    #[doc = "Bits 0:1 - These bits are read only status bits from the PHY OBS port"]
    #[inline(always)]
    pub fn obsportstciky(&self) -> ObsportstcikyR {
        ObsportstcikyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits are read only status bits from the PHY OBS port"]
    #[inline(always)]
    #[must_use]
    pub fn obsportstciky(&mut self) -> ObsportstcikyW<UtmistickystatusSpec> {
        ObsportstcikyW::new(self, 0)
    }
}
#[doc = "This read only register provides the results from the PHY OBS port controlled by reg 0x20\\[5:4\\]. IF any bits are set, the bits are sticky. Clear this register using the OBSCLRSTAT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`utmistickystatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utmistickystatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtmistickystatusSpec;
impl crate::RegisterSpec for UtmistickystatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utmistickystatus::R`](R) reader structure"]
impl crate::Readable for UtmistickystatusSpec {}
#[doc = "`write(|w| ..)` method takes [`utmistickystatus::W`](W) writer structure"]
impl crate::Writable for UtmistickystatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UTMISTICKYSTATUS to value 0"]
impl crate::Resettable for UtmistickystatusSpec {
    const RESET_VALUE: u32 = 0;
}
