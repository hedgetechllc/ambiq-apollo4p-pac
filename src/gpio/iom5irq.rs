#[doc = "Register `IOM5IRQ` reader"]
pub type R = crate::R<Iom5irqSpec>;
#[doc = "Register `IOM5IRQ` writer"]
pub type W = crate::W<Iom5irqSpec>;
#[doc = "Field `IOM5IRQ` reader - IOM5 IRQ pad select."]
pub type Iom5irqR = crate::FieldReader;
#[doc = "Field `IOM5IRQ` writer - IOM5 IRQ pad select."]
pub type Iom5irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM5 IRQ pad select."]
    #[inline(always)]
    pub fn iom5irq(&self) -> Iom5irqR {
        Iom5irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM5 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom5irq(&mut self) -> Iom5irqW<Iom5irqSpec> {
        Iom5irqW::new(self, 0)
    }
}
#[doc = "IOM5 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom5irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom5irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom5irqSpec;
impl crate::RegisterSpec for Iom5irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom5irq::R`](R) reader structure"]
impl crate::Readable for Iom5irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom5irq::W`](W) writer structure"]
impl crate::Writable for Iom5irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM5IRQ to value 0x3f"]
impl crate::Resettable for Iom5irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
