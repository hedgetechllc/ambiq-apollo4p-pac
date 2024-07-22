#[doc = "Register `IOM1IRQ` reader"]
pub type R = crate::R<Iom1irqSpec>;
#[doc = "Register `IOM1IRQ` writer"]
pub type W = crate::W<Iom1irqSpec>;
#[doc = "Field `IOM1IRQ` reader - IOM1 IRQ pad select."]
pub type Iom1irqR = crate::FieldReader;
#[doc = "Field `IOM1IRQ` writer - IOM1 IRQ pad select."]
pub type Iom1irqW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - IOM1 IRQ pad select."]
    #[inline(always)]
    pub fn iom1irq(&self) -> Iom1irqR {
        Iom1irqR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - IOM1 IRQ pad select."]
    #[inline(always)]
    #[must_use]
    pub fn iom1irq(&mut self) -> Iom1irqW<Iom1irqSpec> {
        Iom1irqW::new(self, 0)
    }
}
#[doc = "IOM1 IRQ select for flow control.\n\nYou can [`read`](crate::Reg::read) this register and get [`iom1irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iom1irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iom1irqSpec;
impl crate::RegisterSpec for Iom1irqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iom1irq::R`](R) reader structure"]
impl crate::Readable for Iom1irqSpec {}
#[doc = "`write(|w| ..)` method takes [`iom1irq::W`](W) writer structure"]
impl crate::Writable for Iom1irqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOM1IRQ to value 0x3f"]
impl crate::Resettable for Iom1irqSpec {
    const RESET_VALUE: u32 = 0x3f;
}
