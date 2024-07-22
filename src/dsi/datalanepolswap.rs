#[doc = "Register `DATALANEPOLSWAP` reader"]
pub type R = crate::R<DatalanepolswapSpec>;
#[doc = "Register `DATALANEPOLSWAP` writer"]
pub type W = crate::W<DatalanepolswapSpec>;
#[doc = "Data lane Polarity sw\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datalnpolswap {
    #[doc = "1: lane 0 polarity swap"]
    Lane0 = 1,
    #[doc = "2: lane 1 polarity swap"]
    Lane1 = 2,
    #[doc = "4: lane 2 polarity swap"]
    Lane2 = 4,
    #[doc = "8: lane 3 polarity swap"]
    Lane3 = 8,
    #[doc = "15: data lanes polarity swap"]
    Alllanes = 15,
}
impl From<Datalnpolswap> for u8 {
    #[inline(always)]
    fn from(variant: Datalnpolswap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datalnpolswap {
    type Ux = u8;
}
impl crate::IsEnum for Datalnpolswap {}
#[doc = "Field `DATALNPOLSWAP` reader - Data lane Polarity sw"]
pub type DatalnpolswapR = crate::FieldReader<Datalnpolswap>;
impl DatalnpolswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datalnpolswap> {
        match self.bits {
            1 => Some(Datalnpolswap::Lane0),
            2 => Some(Datalnpolswap::Lane1),
            4 => Some(Datalnpolswap::Lane2),
            8 => Some(Datalnpolswap::Lane3),
            15 => Some(Datalnpolswap::Alllanes),
            _ => None,
        }
    }
    #[doc = "lane 0 polarity swap"]
    #[inline(always)]
    pub fn is_lane0(&self) -> bool {
        *self == Datalnpolswap::Lane0
    }
    #[doc = "lane 1 polarity swap"]
    #[inline(always)]
    pub fn is_lane1(&self) -> bool {
        *self == Datalnpolswap::Lane1
    }
    #[doc = "lane 2 polarity swap"]
    #[inline(always)]
    pub fn is_lane2(&self) -> bool {
        *self == Datalnpolswap::Lane2
    }
    #[doc = "lane 3 polarity swap"]
    #[inline(always)]
    pub fn is_lane3(&self) -> bool {
        *self == Datalnpolswap::Lane3
    }
    #[doc = "data lanes polarity swap"]
    #[inline(always)]
    pub fn is_alllanes(&self) -> bool {
        *self == Datalnpolswap::Alllanes
    }
}
#[doc = "Field `DATALNPOLSWAP` writer - Data lane Polarity sw"]
pub type DatalnpolswapW<'a, REG> = crate::FieldWriter<'a, REG, 4, Datalnpolswap>;
impl<'a, REG> DatalnpolswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lane 0 polarity swap"]
    #[inline(always)]
    pub fn lane0(self) -> &'a mut crate::W<REG> {
        self.variant(Datalnpolswap::Lane0)
    }
    #[doc = "lane 1 polarity swap"]
    #[inline(always)]
    pub fn lane1(self) -> &'a mut crate::W<REG> {
        self.variant(Datalnpolswap::Lane1)
    }
    #[doc = "lane 2 polarity swap"]
    #[inline(always)]
    pub fn lane2(self) -> &'a mut crate::W<REG> {
        self.variant(Datalnpolswap::Lane2)
    }
    #[doc = "lane 3 polarity swap"]
    #[inline(always)]
    pub fn lane3(self) -> &'a mut crate::W<REG> {
        self.variant(Datalnpolswap::Lane3)
    }
    #[doc = "data lanes polarity swap"]
    #[inline(always)]
    pub fn alllanes(self) -> &'a mut crate::W<REG> {
        self.variant(Datalnpolswap::Alllanes)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data lane Polarity sw"]
    #[inline(always)]
    pub fn datalnpolswap(&self) -> DatalnpolswapR {
        DatalnpolswapR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data lane Polarity sw"]
    #[inline(always)]
    #[must_use]
    pub fn datalnpolswap(&mut self) -> DatalnpolswapW<DatalanepolswapSpec> {
        DatalnpolswapW::new(self, 0)
    }
}
#[doc = "Data lane polarity swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`datalanepolswap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalanepolswap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatalanepolswapSpec;
impl crate::RegisterSpec for DatalanepolswapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datalanepolswap::R`](R) reader structure"]
impl crate::Readable for DatalanepolswapSpec {}
#[doc = "`write(|w| ..)` method takes [`datalanepolswap::W`](W) writer structure"]
impl crate::Writable for DatalanepolswapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATALANEPOLSWAP to value 0"]
impl crate::Resettable for DatalanepolswapSpec {
    const RESET_VALUE: u32 = 0;
}
