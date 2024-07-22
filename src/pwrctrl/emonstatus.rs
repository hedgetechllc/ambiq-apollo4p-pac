#[doc = "Register `EMONSTATUS` reader"]
pub type R = crate::R<EmonstatusSpec>;
#[doc = "Register `EMONSTATUS` writer"]
pub type W = crate::W<EmonstatusSpec>;
#[doc = "Field `EMONOVERFLOW0` reader - Energy Monitor counter0 overflow"]
pub type Emonoverflow0R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW0` writer - Energy Monitor counter0 overflow"]
pub type Emonoverflow0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW1` reader - Energy Monitor counter1 overflow"]
pub type Emonoverflow1R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW1` writer - Energy Monitor counter1 overflow"]
pub type Emonoverflow1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW2` reader - Energy Monitor counter2 overflow"]
pub type Emonoverflow2R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW2` writer - Energy Monitor counter2 overflow"]
pub type Emonoverflow2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW3` reader - Energy Monitor counter3 overflow"]
pub type Emonoverflow3R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW3` writer - Energy Monitor counter3 overflow"]
pub type Emonoverflow3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW4` reader - Energy Monitor counter4 overflow"]
pub type Emonoverflow4R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW4` writer - Energy Monitor counter4 overflow"]
pub type Emonoverflow4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW5` reader - Energy Monitor counter5 overflow"]
pub type Emonoverflow5R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW5` writer - Energy Monitor counter5 overflow"]
pub type Emonoverflow5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW6` reader - Energy Monitor counter6 overflow"]
pub type Emonoverflow6R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW6` writer - Energy Monitor counter6 overflow"]
pub type Emonoverflow6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMONOVERFLOW7` reader - Energy Monitor counter7 overflow"]
pub type Emonoverflow7R = crate::BitReader;
#[doc = "Field `EMONOVERFLOW7` writer - Energy Monitor counter7 overflow"]
pub type Emonoverflow7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Energy Monitor counter0 overflow"]
    #[inline(always)]
    pub fn emonoverflow0(&self) -> Emonoverflow0R {
        Emonoverflow0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Energy Monitor counter1 overflow"]
    #[inline(always)]
    pub fn emonoverflow1(&self) -> Emonoverflow1R {
        Emonoverflow1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Energy Monitor counter2 overflow"]
    #[inline(always)]
    pub fn emonoverflow2(&self) -> Emonoverflow2R {
        Emonoverflow2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Energy Monitor counter3 overflow"]
    #[inline(always)]
    pub fn emonoverflow3(&self) -> Emonoverflow3R {
        Emonoverflow3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Energy Monitor counter4 overflow"]
    #[inline(always)]
    pub fn emonoverflow4(&self) -> Emonoverflow4R {
        Emonoverflow4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Energy Monitor counter5 overflow"]
    #[inline(always)]
    pub fn emonoverflow5(&self) -> Emonoverflow5R {
        Emonoverflow5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Energy Monitor counter6 overflow"]
    #[inline(always)]
    pub fn emonoverflow6(&self) -> Emonoverflow6R {
        Emonoverflow6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Energy Monitor counter7 overflow"]
    #[inline(always)]
    pub fn emonoverflow7(&self) -> Emonoverflow7R {
        Emonoverflow7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Monitor counter0 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow0(&mut self) -> Emonoverflow0W<EmonstatusSpec> {
        Emonoverflow0W::new(self, 0)
    }
    #[doc = "Bit 1 - Energy Monitor counter1 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow1(&mut self) -> Emonoverflow1W<EmonstatusSpec> {
        Emonoverflow1W::new(self, 1)
    }
    #[doc = "Bit 2 - Energy Monitor counter2 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow2(&mut self) -> Emonoverflow2W<EmonstatusSpec> {
        Emonoverflow2W::new(self, 2)
    }
    #[doc = "Bit 3 - Energy Monitor counter3 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow3(&mut self) -> Emonoverflow3W<EmonstatusSpec> {
        Emonoverflow3W::new(self, 3)
    }
    #[doc = "Bit 4 - Energy Monitor counter4 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow4(&mut self) -> Emonoverflow4W<EmonstatusSpec> {
        Emonoverflow4W::new(self, 4)
    }
    #[doc = "Bit 5 - Energy Monitor counter5 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow5(&mut self) -> Emonoverflow5W<EmonstatusSpec> {
        Emonoverflow5W::new(self, 5)
    }
    #[doc = "Bit 6 - Energy Monitor counter6 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow6(&mut self) -> Emonoverflow6W<EmonstatusSpec> {
        Emonoverflow6W::new(self, 6)
    }
    #[doc = "Bit 7 - Energy Monitor counter7 overflow"]
    #[inline(always)]
    #[must_use]
    pub fn emonoverflow7(&mut self) -> Emonoverflow7W<EmonstatusSpec> {
        Emonoverflow7W::new(self, 7)
    }
}
#[doc = "Energy Monitor status\n\nYou can [`read`](crate::Reg::read) this register and get [`emonstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emonstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmonstatusSpec;
impl crate::RegisterSpec for EmonstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emonstatus::R`](R) reader structure"]
impl crate::Readable for EmonstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`emonstatus::W`](W) writer structure"]
impl crate::Writable for EmonstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMONSTATUS to value 0"]
impl crate::Resettable for EmonstatusSpec {
    const RESET_VALUE: u32 = 0;
}
