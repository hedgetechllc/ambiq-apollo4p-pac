#[doc = "Register `DEPTHDYH` reader"]
pub type R = crate::R<DepthdyhSpec>;
#[doc = "Register `DEPTHDYH` writer"]
pub type W = crate::W<DepthdyhSpec>;
#[doc = "Field `YAXISHI` reader - Added depth value for each step at y-axis"]
pub type YaxishiR = crate::FieldReader<u32>;
#[doc = "Field `YAXISHI` writer - Added depth value for each step at y-axis"]
pub type YaxishiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Added depth value for each step at y-axis"]
    #[inline(always)]
    pub fn yaxishi(&self) -> YaxishiR {
        YaxishiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Added depth value for each step at y-axis"]
    #[inline(always)]
    #[must_use]
    pub fn yaxishi(&mut self) -> YaxishiW<DepthdyhSpec> {
        YaxishiW::new(self, 0)
    }
}
#[doc = "Added depth value for each step at y-axis (32 high bits integral.)\n\nYou can [`read`](crate::Reg::read) this register and get [`depthdyh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`depthdyh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DepthdyhSpec;
impl crate::RegisterSpec for DepthdyhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`depthdyh::R`](R) reader structure"]
impl crate::Readable for DepthdyhSpec {}
#[doc = "`write(|w| ..)` method takes [`depthdyh::W`](W) writer structure"]
impl crate::Writable for DepthdyhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEPTHDYH to value 0"]
impl crate::Resettable for DepthdyhSpec {
    const RESET_VALUE: u32 = 0;
}
