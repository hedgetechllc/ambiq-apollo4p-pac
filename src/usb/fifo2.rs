#[doc = "Register `FIFO2` reader"]
pub type R = crate::R<Fifo2Spec>;
#[doc = "Register `FIFO2` writer"]
pub type W = crate::W<Fifo2Spec>;
#[doc = "Field `FIFO` reader - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 2."]
pub type FifoR = crate::FieldReader<u32>;
#[doc = "Field `FIFO` writer - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 2."]
pub type FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 2."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 2."]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FifoW<Fifo2Spec> {
        FifoW::new(self, 0)
    }
}
#[doc = "Endpoint 2 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo2Spec;
impl crate::RegisterSpec for Fifo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo2::R`](R) reader structure"]
impl crate::Readable for Fifo2Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo2::W`](W) writer structure"]
impl crate::Writable for Fifo2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO2 to value 0"]
impl crate::Resettable for Fifo2Spec {
    const RESET_VALUE: u32 = 0;
}
