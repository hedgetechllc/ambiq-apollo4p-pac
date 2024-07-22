#[doc = "Register `CALCTRL` reader"]
pub type R = crate::R<CalctrlSpec>;
#[doc = "Register `CALCTRL` writer"]
pub type W = crate::W<CalctrlSpec>;
#[doc = "Field `RESETDLY` reader - Sets the delay between the deassertion of ADC reset and the start of calibration in ADC CLKs. Minimum per spec is 8 CLKs. Specifying a value smaller than 8 is not supported."]
pub type ResetdlyR = crate::FieldReader<u16>;
#[doc = "Field `RESETDLY` writer - Sets the delay between the deassertion of ADC reset and the start of calibration in ADC CLKs. Minimum per spec is 8 CLKs. Specifying a value smaller than 8 is not supported."]
pub type ResetdlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ISODLY` reader - Sets the delay between the deassertion of ADC isolation and the deassertion of ADC reset in ADC CLKs. This allows for synchronous reset of logic within the ADC."]
pub type IsodlyR = crate::FieldReader<u16>;
#[doc = "Field `ISODLY` writer - Sets the delay between the deassertion of ADC isolation and the deassertion of ADC reset in ADC CLKs. This allows for synchronous reset of logic within the ADC."]
pub type IsodlyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sets the delay between the deassertion of ADC reset and the start of calibration in ADC CLKs. Minimum per spec is 8 CLKs. Specifying a value smaller than 8 is not supported."]
    #[inline(always)]
    pub fn resetdly(&self) -> ResetdlyR {
        ResetdlyR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Sets the delay between the deassertion of ADC isolation and the deassertion of ADC reset in ADC CLKs. This allows for synchronous reset of logic within the ADC."]
    #[inline(always)]
    pub fn isodly(&self) -> IsodlyR {
        IsodlyR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the delay between the deassertion of ADC reset and the start of calibration in ADC CLKs. Minimum per spec is 8 CLKs. Specifying a value smaller than 8 is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn resetdly(&mut self) -> ResetdlyW<CalctrlSpec> {
        ResetdlyW::new(self, 0)
    }
    #[doc = "Bits 10:19 - Sets the delay between the deassertion of ADC isolation and the deassertion of ADC reset in ADC CLKs. This allows for synchronous reset of logic within the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn isodly(&mut self) -> IsodlyW<CalctrlSpec> {
        IsodlyW::new(self, 10)
    }
}
#[doc = "Calibration Control\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlSpec;
impl crate::RegisterSpec for CalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl::R`](R) reader structure"]
impl crate::Readable for CalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl::W`](W) writer structure"]
impl crate::Writable for CalctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALCTRL to value 0x200a"]
impl crate::Resettable for CalctrlSpec {
    const RESET_VALUE: u32 = 0x200a;
}
