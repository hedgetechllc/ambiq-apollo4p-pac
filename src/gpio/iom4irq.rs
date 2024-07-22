#[doc = "Register `IOM4IRQ` reader"]
pub type R = crate::R<Iom4irqSpec>;
#[doc = "Register `IOM4IRQ` writer"]
pub type W = crate::W<Iom4irqSpec>;
#[doc = "Field `IOM4IRQ` reader - IOM4 IRQ pad select."]
pub type Iom4irqR = crate::FieldReader;
#[doc = "Field `IOM4IRQ` writer - IOM4 IRQ pad select."]
pub type Iom4irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM4 IRQ pad select."]
    #[inline(always)]
    pub fn iom4irq(&self) -> Iom4irqR {
        Iom4irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM4 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom4irq(&mut self) -> Iom4irqW<Iom4irqSpec> {
        Iom4irqW::new(self, 0)
    }
}
#[doc = "IOM4 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom4irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom4irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom4irqSpec;
impl crate::RegisterSpec for Iom4irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom4irq::R`](R) reader structure"]
impl crate::Readable for Iom4irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom4irq::W`](W) writer structure"]
impl crate::Writable for Iom4irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM4IRQ to value 0x3f"]
impl crate::Resettable for Iom4irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
