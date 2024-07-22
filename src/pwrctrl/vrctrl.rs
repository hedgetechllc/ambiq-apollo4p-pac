#[doc = "Register `VRCTRL` reader"]
pub type R = crate::R<VrctrlSpec>;
#[doc = "Register `VRCTRL` writer"]
pub type W = crate::W<VrctrlSpec>;
#[doc = "Enables and Selects the SIMO Buck as the supply for the low-voltage power domains. It takes the initial value from the bit set in Customer INFO space.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Simobucken {
    #[doc = "1: Enable the SIMO Buck"]
    En = 1,
    #[doc = "0: Disable the SIMO Buck"]
    Dis = 0,
}
impl From<Simobucken> for bool {
    #[inline(always)]
    fn from(variant: Simobucken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIMOBUCKEN` reader - Enables and Selects the SIMO Buck as the supply for the low-voltage power domains. It takes the initial value from the bit set in Customer INFO space."]
pub type SimobuckenR = crate::BitReader<Simobucken>;
impl SimobuckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Simobucken {
        match self.bits {
            true => Simobucken::En,
            false => Simobucken::Dis,
        }
    }
    #[doc = "Enable the SIMO Buck"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Simobucken::En
    }
    #[doc = "Disable the SIMO Buck"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Simobucken::Dis
    }
}
#[doc = "Field `SIMOBUCKEN` writer - Enables and Selects the SIMO Buck as the supply for the low-voltage power domains. It takes the initial value from the bit set in Customer INFO space."]
pub type SimobuckenW<'a, REG> = crate::BitWriter<'a, REG, Simobucken>;
impl<'a, REG> SimobuckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the SIMO Buck"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Simobucken::En)
    }
    #[doc = "Disable the SIMO Buck"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Simobucken::Dis)
    }
}
impl R {
    #[doc = "Bit 0 - Enables and Selects the SIMO Buck as the supply for the low-voltage power domains. It takes the initial value from the bit set in Customer INFO space."]
    #[inline(always)]
    pub fn simobucken(&self) -> SimobuckenR {
        SimobuckenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables and Selects the SIMO Buck as the supply for the low-voltage power domains. It takes the initial value from the bit set in Customer INFO space."]
    #[inline(always)]
    #[must_use]
    pub fn simobucken(&mut self) -> SimobuckenW<VrctrlSpec> {
        SimobuckenW::new(self, 0)
    }
}
#[doc = "This register includes additional debug control bits. This is an internal Ambiq-only register. Customers should not attempt to change this or else functionality cannot be guaranteed.\n\nYou can [`read`](crate::Reg::read) this register and get [`vrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrctrlSpec;
impl crate::RegisterSpec for VrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrctrl::R`](R) reader structure"]
impl crate::Readable for VrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vrctrl::W`](W) writer structure"]
impl crate::Writable for VrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VRCTRL to value 0"]
impl crate::Resettable for VrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
