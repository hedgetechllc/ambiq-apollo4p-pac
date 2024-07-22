#[doc = "Register `FIFO0` reader"]
pub type R = crate::R<Fifo0Spec>;
#[doc = "Register `FIFO0` writer"]
pub type W = crate::W<Fifo0Spec>;
#[doc = "Field `FIFO` reader - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 0."]
pub type FifoR = crate::FieldReader<u32>;
#[doc = "Field `FIFO` writer - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 0."]
pub type FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 0."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing to this register loads data into the IN FIFO and reading from this register unloads data from the OUT FIFO for endpoint 0."]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FifoW<Fifo0Spec> {
        FifoW::new(self, 0)
    }
}
#[doc = "Endpoint 0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo0Spec;
impl crate::RegisterSpec for Fifo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo0::R`](R) reader structure"]
impl crate::Readable for Fifo0Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo0::W`](W) writer structure"]
impl crate::Writable for Fifo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO0 to value 0"]
impl crate::Resettable for Fifo0Spec {
    const RESET_VALUE: u32 = 0;
}
