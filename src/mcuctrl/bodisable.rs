#[doc = "Register `BODISABLE` reader"]
pub type R = crate::R<BodisableSpec>;
#[doc = "Register `BODISABLE` writer"]
pub type W = crate::W<BodisableSpec>;
#[doc = "Disable Unregulated 1.8V Brown-out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodlrde {
    #[doc = "0: Enable Unregulated 1.8v brown out reset."]
    En = 0,
    #[doc = "1: Disable Unregulated 1.8v brown out reset."]
    Dis = 1,
}
impl From<Bodlrde> for bool {
    #[inline(always)]
    fn from(variant: Bodlrde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODLRDE` reader - Disable Unregulated 1.8V Brown-out reset."]
pub type BodlrdeR = crate::BitReader<Bodlrde>;
impl BodlrdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodlrde {
        match self.bits {
            false => Bodlrde::En,
            true => Bodlrde::Dis,
        }
    }
    #[doc = "Enable Unregulated 1.8v brown out reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bodlrde::En
    }
    #[doc = "Disable Unregulated 1.8v brown out reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bodlrde::Dis
    }
}
#[doc = "Field `BODLRDE` writer - Disable Unregulated 1.8V Brown-out reset."]
pub type BodlrdeW<'a, REG> = crate::BitWriter<'a, REG, Bodlrde>;
impl<'a, REG> BodlrdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Unregulated 1.8v brown out reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bodlrde::En)
    }
    #[doc = "Disable Unregulated 1.8v brown out reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bodlrde::Dis)
    }
}
#[doc = "Disable VDDC Brown Out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodcren {
    #[doc = "1: Enable VDDC Brown Out reset."]
    En = 1,
    #[doc = "0: Disable VDDC Brown Out reset."]
    Dis = 0,
}
impl From<Bodcren> for bool {
    #[inline(always)]
    fn from(variant: Bodcren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCREN` reader - Disable VDDC Brown Out reset."]
pub type BodcrenR = crate::BitReader<Bodcren>;
impl BodcrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodcren {
        match self.bits {
            true => Bodcren::En,
            false => Bodcren::Dis,
        }
    }
    #[doc = "Enable VDDC Brown Out reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bodcren::En
    }
    #[doc = "Disable VDDC Brown Out reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bodcren::Dis
    }
}
#[doc = "Field `BODCREN` writer - Disable VDDC Brown Out reset."]
pub type BodcrenW<'a, REG> = crate::BitWriter<'a, REG, Bodcren>;
impl<'a, REG> BodcrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable VDDC Brown Out reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bodcren::En)
    }
    #[doc = "Disable VDDC Brown Out reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bodcren::Dis)
    }
}
#[doc = "Disable VDDF Brown Out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodfren {
    #[doc = "1: Enable VDDF Brown Out reset."]
    En = 1,
    #[doc = "0: Disable VDDF Brown Out reset."]
    Dis = 0,
}
impl From<Bodfren> for bool {
    #[inline(always)]
    fn from(variant: Bodfren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODFREN` reader - Disable VDDF Brown Out reset."]
pub type BodfrenR = crate::BitReader<Bodfren>;
impl BodfrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodfren {
        match self.bits {
            true => Bodfren::En,
            false => Bodfren::Dis,
        }
    }
    #[doc = "Enable VDDF Brown Out reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bodfren::En
    }
    #[doc = "Disable VDDF Brown Out reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bodfren::Dis
    }
}
#[doc = "Field `BODFREN` writer - Disable VDDF Brown Out reset."]
pub type BodfrenW<'a, REG> = crate::BitWriter<'a, REG, Bodfren>;
impl<'a, REG> BodfrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable VDDF Brown Out reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bodfren::En)
    }
    #[doc = "Disable VDDF Brown Out reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bodfren::Dis)
    }
}
#[doc = "Disable VDDS Brown Out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodsren {
    #[doc = "1: Enable VDDS Brown Out reset."]
    En = 1,
    #[doc = "0: Disable VDDS Brown Out reset."]
    Dis = 0,
}
impl From<Bodsren> for bool {
    #[inline(always)]
    fn from(variant: Bodsren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODSREN` reader - Disable VDDS Brown Out reset."]
pub type BodsrenR = crate::BitReader<Bodsren>;
impl BodsrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodsren {
        match self.bits {
            true => Bodsren::En,
            false => Bodsren::Dis,
        }
    }
    #[doc = "Enable VDDS Brown Out reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bodsren::En
    }
    #[doc = "Disable VDDS Brown Out reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bodsren::Dis
    }
}
#[doc = "Field `BODSREN` writer - Disable VDDS Brown Out reset."]
pub type BodsrenW<'a, REG> = crate::BitWriter<'a, REG, Bodsren>;
impl<'a, REG> BodsrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable VDDS Brown Out reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bodsren::En)
    }
    #[doc = "Disable VDDS Brown Out reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bodsren::Dis)
    }
}
#[doc = "Disable VDDC_LV Brown Out reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodclvren {
    #[doc = "1: Enable VDDC_LV Brown Out reset."]
    En = 1,
    #[doc = "0: Disable VDDC_LV Brown Out reset."]
    Dis = 0,
}
impl From<Bodclvren> for bool {
    #[inline(always)]
    fn from(variant: Bodclvren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCLVREN` reader - Disable VDDC_LV Brown Out reset."]
pub type BodclvrenR = crate::BitReader<Bodclvren>;
impl BodclvrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodclvren {
        match self.bits {
            true => Bodclvren::En,
            false => Bodclvren::Dis,
        }
    }
    #[doc = "Enable VDDC_LV Brown Out reset."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Bodclvren::En
    }
    #[doc = "Disable VDDC_LV Brown Out reset."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Bodclvren::Dis
    }
}
#[doc = "Field `BODCLVREN` writer - Disable VDDC_LV Brown Out reset."]
pub type BodclvrenW<'a, REG> = crate::BitWriter<'a, REG, Bodclvren>;
impl<'a, REG> BodclvrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable VDDC_LV Brown Out reset."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Bodclvren::En)
    }
    #[doc = "Disable VDDC_LV Brown Out reset."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Bodclvren::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Unregulated 1.8V Brown-out reset."]
    #[inline(always)]
    pub fn bodlrde(&self) -> BodlrdeR {
        BodlrdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable VDDC Brown Out reset."]
    #[inline(always)]
    pub fn bodcren(&self) -> BodcrenR {
        BodcrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable VDDF Brown Out reset."]
    #[inline(always)]
    pub fn bodfren(&self) -> BodfrenR {
        BodfrenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable VDDS Brown Out reset."]
    #[inline(always)]
    pub fn bodsren(&self) -> BodsrenR {
        BodsrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable VDDC_LV Brown Out reset."]
    #[inline(always)]
    pub fn bodclvren(&self) -> BodclvrenR {
        BodclvrenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Unregulated 1.8V Brown-out reset."]
    #[inline(always)]
    #[must_use]
    pub fn bodlrde(&mut self) -> BodlrdeW<BodisableSpec> {
        BodlrdeW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable VDDC Brown Out reset."]
    #[inline(always)]
    #[must_use]
    pub fn bodcren(&mut self) -> BodcrenW<BodisableSpec> {
        BodcrenW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable VDDF Brown Out reset."]
    #[inline(always)]
    #[must_use]
    pub fn bodfren(&mut self) -> BodfrenW<BodisableSpec> {
        BodfrenW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable VDDS Brown Out reset."]
    #[inline(always)]
    #[must_use]
    pub fn bodsren(&mut self) -> BodsrenW<BodisableSpec> {
        BodsrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable VDDC_LV Brown Out reset."]
    #[inline(always)]
    #[must_use]
    pub fn bodclvren(&mut self) -> BodclvrenW<BodisableSpec> {
        BodclvrenW::new(self, 4)
    }
}
#[doc = "Brownout Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`bodisable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodisable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodisableSpec;
impl crate::RegisterSpec for BodisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodisable::R`](R) reader structure"]
impl crate::Readable for BodisableSpec {}
#[doc = "`write(|w| ..)` method takes [`bodisable::W`](W) writer structure"]
impl crate::Writable for BodisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BODISABLE to value 0"]
impl crate::Resettable for BodisableSpec {
    const RESET_VALUE: u32 = 0;
}
