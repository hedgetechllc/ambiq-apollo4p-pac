#[doc = "Register `IOM6IRQ` reader"]
pub type R = crate::R<Iom6irqSpec>;
#[doc = "Register `IOM6IRQ` writer"]
pub type W = crate::W<Iom6irqSpec>;
#[doc = "Field `IOM6IRQ` reader - IOM6 IRQ pad select."]
pub type Iom6irqR = crate::FieldReader;
#[doc = "Field `IOM6IRQ` writer - IOM6 IRQ pad select."]
pub type Iom6irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM6 IRQ pad select."]
    #[inline(always)]
    pub fn iom6irq(&self) -> Iom6irqR {
        Iom6irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM6 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom6irq(&mut self) -> Iom6irqW<Iom6irqSpec> {
        Iom6irqW::new(self, 0)
    }
}
#[doc = "IOM6 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom6irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom6irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom6irqSpec;
impl crate::RegisterSpec for Iom6irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom6irq::R`](R) reader structure"]
impl crate::Readable for Iom6irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom6irq::W`](W) writer structure"]
impl crate::Writable for Iom6irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM6IRQ to value 0x3f"]
impl crate::Resettable for Iom6irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
