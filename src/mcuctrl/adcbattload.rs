#[doc = "Register `ADCBATTLOAD` reader"]
pub type R = crate::R<AdcbattloadSpec>;
#[doc = "Register `ADCBATTLOAD` writer"]
pub type W = crate::W<AdcbattloadSpec>;
#[doc = "Enable the ADC battery load resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Battload {
    #[doc = "0: Battery load is disconnected"]
    Dis = 0,
    #[doc = "1: Battery load is enabled"]
    En = 1,
}
impl From<Battload> for bool {
    #[inline(always)]
    fn from(variant: Battload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATTLOAD` reader - Enable the ADC battery load resistor"]
pub type BattloadR = crate::BitReader<Battload>;
impl BattloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Battload {
        match self.bits {
            false => Battload::Dis,
            true => Battload::En,
        }
    }
    #[doc = "Battery load is disconnected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Battload::Dis
    }
    #[doc = "Battery load is enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Battload::En
    }
}
#[doc = "Field `BATTLOAD` writer - Enable the ADC battery load resistor"]
pub type BattloadW<'a, REG> = crate::BitWriter<'a, REG, Battload>;
impl<'a, REG> BattloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Battery load is disconnected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Battload::Dis)
    }
    #[doc = "Battery load is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Battload::En)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    pub fn battload(&self) -> BattloadR {
        BattloadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    #[must_use]
    pub fn battload(&mut self) -> BattloadW<AdcbattloadSpec> {
        BattloadW::new(self, 0)
    }
}
#[doc = "ADC Battery Load Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbattload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbattload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcbattloadSpec;
impl crate::RegisterSpec for AdcbattloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbattload::R`](R) reader structure"]
impl crate::Readable for AdcbattloadSpec {}
#[doc = "`write(|w| ..)` method takes [`adcbattload::W`](W) writer structure"]
impl crate::Writable for AdcbattloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBATTLOAD to value 0"]
impl crate::Resettable for AdcbattloadSpec {
    const RESET_VALUE: u32 = 0;
}
