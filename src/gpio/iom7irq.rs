#[doc = "Register `IOM7IRQ` reader"]
pub type R = crate::R<Iom7irqSpec>;
#[doc = "Register `IOM7IRQ` writer"]
pub type W = crate::W<Iom7irqSpec>;
#[doc = "Field `IOM7IRQ` reader - IOM7 IRQ pad select."]
pub type Iom7irqR = crate::FieldReader;
#[doc = "Field `IOM7IRQ` writer - IOM7 IRQ pad select."]
pub type Iom7irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM7 IRQ pad select."]
    #[inline(always)]
    pub fn iom7irq(&self) -> Iom7irqR {
        Iom7irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM7 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom7irq(&mut self) -> Iom7irqW<Iom7irqSpec> {
        Iom7irqW::new(self, 0)
    }
}
#[doc = "IOM7 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom7irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom7irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom7irqSpec;
impl crate::RegisterSpec for Iom7irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom7irq::R`](R) reader structure"]
impl crate::Readable for Iom7irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom7irq::W`](W) writer structure"]
impl crate::Writable for Iom7irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM7IRQ to value 0x3f"]
impl crate::Resettable for Iom7irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
