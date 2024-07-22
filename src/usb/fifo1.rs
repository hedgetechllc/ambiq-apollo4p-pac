#[doc = "Register `FIFO1` reader"]
pub type R = crate::R<Fifo1Spec>;
#[doc = "Register `FIFO1` writer"]
pub type W = crate::W<Fifo1Spec>;
#[doc = "Field `FIFO` reader - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 1."]
pub type FifoR = crate::FieldReader<u32>;
#[doc = "Field `FIFO` writer - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 1."]
pub type FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 1."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 1."]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FifoW<Fifo1Spec> {
        FifoW::new(self, 0)
    }
}
#[doc = "Endpoint 1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo1Spec;
impl crate::RegisterSpec for Fifo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo1::R`](R) reader structure"]
impl crate::Readable for Fifo1Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo1::W`](W) writer structure"]
impl crate::Writable for Fifo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for Fifo1Spec {
    const RESET_VALUE: u32 = 0;
}
