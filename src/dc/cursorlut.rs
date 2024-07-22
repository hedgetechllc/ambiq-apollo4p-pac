#[doc = "Register `CURSORLUT` reader"]
pub type R = crate::R<CursorlutSpec>;
#[doc = "Register `CURSORLUT` writer"]
pub type W = crate::W<CursorlutSpec>;
#[doc = "Field `CURLUT0B` reader - Cursor LUT blue bits"]
pub type Curlut0bR = crate::FieldReader;
#[doc = "Field `CURLUT0B` writer - Cursor LUT blue bits"]
pub type Curlut0bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CURLUT0G` reader - Cursor LUT green bits"]
pub type Curlut0gR = crate::FieldReader;
#[doc = "Field `CURLUT0G` writer - Cursor LUT green bits"]
pub type Curlut0gW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CURLUT0R` reader - Cursor LUT red bits"]
pub type Curlut0rR = crate::FieldReader;
#[doc = "Field `CURLUT0R` writer - Cursor LUT red bits"]
pub type Curlut0rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cursor LUT blue bits"]
    #[inline(always)]
    pub fn curlut0b(&self) -> Curlut0bR {
        Curlut0bR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Cursor LUT green bits"]
    #[inline(always)]
    pub fn curlut0g(&self) -> Curlut0gR {
        Curlut0gR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Cursor LUT red bits"]
    #[inline(always)]
    pub fn curlut0r(&self) -> Curlut0rR {
        Curlut0rR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cursor LUT blue bits"]
    #[inline(always)]
    #[must_use]
    pub fn curlut0b(&mut self) -> Curlut0bW<CursorlutSpec> {
        Curlut0bW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Cursor LUT green bits"]
    #[inline(always)]
    #[must_use]
    pub fn curlut0g(&mut self) -> Curlut0gW<CursorlutSpec> {
        Curlut0gW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Cursor LUT red bits"]
    #[inline(always)]
    #[must_use]
    pub fn curlut0r(&mut self) -> Curlut0rW<CursorlutSpec> {
        Curlut0rW::new(self, 16)
    }
}
#[doc = "R\\[0\\]G\\[0\\]B\\[0\\]
thru R\\[15\\]G\\[15\\]B\\[15\\]
Cursor Look-up Table where x starts at 0 thru 15.Access to all 16 registers is best accomplished by passing an index via a macro. e.g. pseudocode #define DC_CURSORLUT(n) (*((volatile uint32_t*)(&amp;CURSORLUT + (4*n))))\n\nYou can [`read`](crate::Reg::read) this register and get [`cursorlut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cursorlut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CursorlutSpec;
impl crate::RegisterSpec for CursorlutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cursorlut::R`](R) reader structure"]
impl crate::Readable for CursorlutSpec {}
#[doc = "`write(|w| ..)` method takes [`cursorlut::W`](W) writer structure"]
impl crate::Writable for CursorlutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURSORLUT to value 0"]
impl crate::Resettable for CursorlutSpec {
    const RESET_VALUE: u32 = 0;
}
