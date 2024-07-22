#[doc = "Register `IOM0IRQ` reader"]
pub type R = crate::R<Iom0irqSpec>;
#[doc = "Register `IOM0IRQ` writer"]
pub type W = crate::W<Iom0irqSpec>;
#[doc = "Field `IOM0IRQ` reader - IOM0 IRQ pad select."]
pub type Iom0irqR = crate::FieldReader;
#[doc = "Field `IOM0IRQ` writer - IOM0 IRQ pad select."]
pub type Iom0irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM0 IRQ pad select."]
    #[inline(always)]
    pub fn iom0irq(&self) -> Iom0irqR {
        Iom0irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM0 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom0irq(&mut self) -> Iom0irqW<Iom0irqSpec> {
        Iom0irqW::new(self, 0)
    }
}
#[doc = "IOM0 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom0irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom0irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom0irqSpec;
impl crate::RegisterSpec for Iom0irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom0irq::R`](R) reader structure"]
impl crate::Readable for Iom0irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom0irq::W`](W) writer structure"]
impl crate::Writable for Iom0irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM0IRQ to value 0x7f"]
impl crate::Resettable for Iom0irqSpec {
    const RESET_VALUE: u32 = 0x7f;
}
