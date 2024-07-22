#[doc = "Register `SATMAX` reader"]
pub type R = crate::R<SatmaxSpec>;
#[doc = "Register `SATMAX` writer"]
pub type W = crate::W<SatmaxSpec>;
#[doc = "Field `SATCAMAX` reader - Sets the number of saturation events that may occur before a SATCA interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
pub type SatcamaxR = crate::FieldReader<u16>;
#[doc = "Field `SATCAMAX` writer - Sets the number of saturation events that may occur before a SATCA interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
pub type SatcamaxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SATCBMAX` reader - Sets the number of saturation events that may occur before a SATCB interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
pub type SatcbmaxR = crate::FieldReader<u16>;
#[doc = "Field `SATCBMAX` writer - Sets the number of saturation events that may occur before a SATCB interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
pub type SatcbmaxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Sets the number of saturation events that may occur before a SATCA interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
    #[inline(always)]
    pub fn satcamax(&self) -> SatcamaxR {
        SatcamaxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Sets the number of saturation events that may occur before a SATCB interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
    #[inline(always)]
    pub fn satcbmax(&self) -> SatcbmaxR {
        SatcbmaxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the number of saturation events that may occur before a SATCA interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
    #[inline(always)]
    #[must_use]
    pub fn satcamax(&mut self) -> SatcamaxW<SatmaxSpec> {
        SatcamaxW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Sets the number of saturation events that may occur before a SATCB interrupt occurs. Once this interrupt occurs, the saturation event counter must be cleared by writing the SATCLR register. A value of 0 is invalid and will cause the saturation interrupt to assert immediately."]
    #[inline(always)]
    #[must_use]
    pub fn satcbmax(&mut self) -> SatcbmaxW<SatmaxSpec> {
        SatcbmaxW::new(self, 16)
    }
}
#[doc = "Saturation Comparator Event Counter Limits\n\nYou can [`read`](crate::Reg::read) this register and get [`satmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`satmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SatmaxSpec;
impl crate::RegisterSpec for SatmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`satmax::R`](R) reader structure"]
impl crate::Readable for SatmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`satmax::W`](W) writer structure"]
impl crate::Writable for SatmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATMAX to value 0x0001_0001"]
impl crate::Resettable for SatmaxSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
