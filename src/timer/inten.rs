#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TMR00INT` reader - Counter/Timer 0 interrupt based on CMP0."]
pub type Tmr00intR = crate::BitReader;
#[doc = "Field `TMR00INT` writer - Counter/Timer 0 interrupt based on CMP0."]
pub type Tmr00intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR01INT` reader - Counter/Timer 0 interrupt based on CMP1."]
pub type Tmr01intR = crate::BitReader;
#[doc = "Field `TMR01INT` writer - Counter/Timer 0 interrupt based on CMP1."]
pub type Tmr01intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10INT` reader - Counter/Timer 1 interrupt based on CMP0."]
pub type Tmr10intR = crate::BitReader;
#[doc = "Field `TMR10INT` writer - Counter/Timer 1 interrupt based on CMP0."]
pub type Tmr10intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11INT` reader - Counter/Timer 1 interrupt based on CMP1."]
pub type Tmr11intR = crate::BitReader;
#[doc = "Field `TMR11INT` writer - Counter/Timer 1 interrupt based on CMP1."]
pub type Tmr11intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20INT` reader - Counter/Timer 2 interrupt based on CMP0."]
pub type Tmr20intR = crate::BitReader;
#[doc = "Field `TMR20INT` writer - Counter/Timer 2 interrupt based on CMP0."]
pub type Tmr20intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR21INT` reader - Counter/Timer 2 interrupt based on CMP1."]
pub type Tmr21intR = crate::BitReader;
#[doc = "Field `TMR21INT` writer - Counter/Timer 2 interrupt based on CMP1."]
pub type Tmr21intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR30INT` reader - Counter/Timer 3 interrupt based on CMP0."]
pub type Tmr30intR = crate::BitReader;
#[doc = "Field `TMR30INT` writer - Counter/Timer 3 interrupt based on CMP0."]
pub type Tmr30intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR31INT` reader - Counter/Timer 3 interrupt based on CMP1."]
pub type Tmr31intR = crate::BitReader;
#[doc = "Field `TMR31INT` writer - Counter/Timer 3 interrupt based on CMP1."]
pub type Tmr31intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR40INT` reader - Counter/Timer 4 interrupt based on CMP0."]
pub type Tmr40intR = crate::BitReader;
#[doc = "Field `TMR40INT` writer - Counter/Timer 4 interrupt based on CMP0."]
pub type Tmr40intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR41INT` reader - Counter/Timer 4 interrupt based on CMP1."]
pub type Tmr41intR = crate::BitReader;
#[doc = "Field `TMR41INT` writer - Counter/Timer 4 interrupt based on CMP1."]
pub type Tmr41intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR50INT` reader - Counter/Timer 5 interrupt based on CMP0."]
pub type Tmr50intR = crate::BitReader;
#[doc = "Field `TMR50INT` writer - Counter/Timer 5 interrupt based on CMP0."]
pub type Tmr50intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR51INT` reader - Counter/Timer 5 interrupt based on CMP1."]
pub type Tmr51intR = crate::BitReader;
#[doc = "Field `TMR51INT` writer - Counter/Timer 5 interrupt based on CMP1."]
pub type Tmr51intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR60INT` reader - Counter/Timer 6 interrupt based on CMP0."]
pub type Tmr60intR = crate::BitReader;
#[doc = "Field `TMR60INT` writer - Counter/Timer 6 interrupt based on CMP0."]
pub type Tmr60intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR61INT` reader - Counter/Timer 6 interrupt based on CMP1."]
pub type Tmr61intR = crate::BitReader;
#[doc = "Field `TMR61INT` writer - Counter/Timer 6 interrupt based on CMP1."]
pub type Tmr61intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR70INT` reader - Counter/Timer 7 interrupt based on CMP0."]
pub type Tmr70intR = crate::BitReader;
#[doc = "Field `TMR70INT` writer - Counter/Timer 7 interrupt based on CMP0."]
pub type Tmr70intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR71INT` reader - Counter/Timer 7 interrupt based on CMP1."]
pub type Tmr71intR = crate::BitReader;
#[doc = "Field `TMR71INT` writer - Counter/Timer 7 interrupt based on CMP1."]
pub type Tmr71intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR80INT` reader - Counter/Timer 8 interrupt based on CMP0."]
pub type Tmr80intR = crate::BitReader;
#[doc = "Field `TMR80INT` writer - Counter/Timer 8 interrupt based on CMP0."]
pub type Tmr80intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR81INT` reader - Counter/Timer 8 interrupt based on CMP1."]
pub type Tmr81intR = crate::BitReader;
#[doc = "Field `TMR81INT` writer - Counter/Timer 8 interrupt based on CMP1."]
pub type Tmr81intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR90INT` reader - Counter/Timer 9 interrupt based on CMP0."]
pub type Tmr90intR = crate::BitReader;
#[doc = "Field `TMR90INT` writer - Counter/Timer 9 interrupt based on CMP0."]
pub type Tmr90intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR91INT` reader - Counter/Timer 9 interrupt based on CMP1."]
pub type Tmr91intR = crate::BitReader;
#[doc = "Field `TMR91INT` writer - Counter/Timer 9 interrupt based on CMP1."]
pub type Tmr91intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR100INT` reader - Counter/Timer 10 interrupt based on CMP0."]
pub type Tmr100intR = crate::BitReader;
#[doc = "Field `TMR100INT` writer - Counter/Timer 10 interrupt based on CMP0."]
pub type Tmr100intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR101INT` reader - Counter/Timer 10 interrupt based on CMP1."]
pub type Tmr101intR = crate::BitReader;
#[doc = "Field `TMR101INT` writer - Counter/Timer 10 interrupt based on CMP1."]
pub type Tmr101intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR110INT` reader - Counter/Timer 11 interrupt based on CMP0."]
pub type Tmr110intR = crate::BitReader;
#[doc = "Field `TMR110INT` writer - Counter/Timer 11 interrupt based on CMP0."]
pub type Tmr110intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR111INT` reader - Counter/Timer 11 interrupt based on CMP1."]
pub type Tmr111intR = crate::BitReader;
#[doc = "Field `TMR111INT` writer - Counter/Timer 11 interrupt based on CMP1."]
pub type Tmr111intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR120INT` reader - Counter/Timer 12 interrupt based on CMP0."]
pub type Tmr120intR = crate::BitReader;
#[doc = "Field `TMR120INT` writer - Counter/Timer 12 interrupt based on CMP0."]
pub type Tmr120intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR121INT` reader - Counter/Timer 12 interrupt based on CMP1."]
pub type Tmr121intR = crate::BitReader;
#[doc = "Field `TMR121INT` writer - Counter/Timer 12 interrupt based on CMP1."]
pub type Tmr121intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR130INT` reader - Counter/Timer 13 interrupt based on CMP0."]
pub type Tmr130intR = crate::BitReader;
#[doc = "Field `TMR130INT` writer - Counter/Timer 13 interrupt based on CMP0."]
pub type Tmr130intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR131INT` reader - Counter/Timer 13 interrupt based on CMP1."]
pub type Tmr131intR = crate::BitReader;
#[doc = "Field `TMR131INT` writer - Counter/Timer 13 interrupt based on CMP1."]
pub type Tmr131intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR140INT` reader - Counter/Timer 14 interrupt based on CMP0."]
pub type Tmr140intR = crate::BitReader;
#[doc = "Field `TMR140INT` writer - Counter/Timer 14 interrupt based on CMP0."]
pub type Tmr140intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR141INT` reader - Counter/Timer 14 interrupt based on CMP1."]
pub type Tmr141intR = crate::BitReader;
#[doc = "Field `TMR141INT` writer - Counter/Timer 14 interrupt based on CMP1."]
pub type Tmr141intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR150INT` reader - Counter/Timer 15 interrupt based on CMP0."]
pub type Tmr150intR = crate::BitReader;
#[doc = "Field `TMR150INT` writer - Counter/Timer 15 interrupt based on CMP0."]
pub type Tmr150intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR151INT` reader - Counter/Timer 15 interrupt based on CMP1."]
pub type Tmr151intR = crate::BitReader;
#[doc = "Field `TMR151INT` writer - Counter/Timer 15 interrupt based on CMP1."]
pub type Tmr151intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter/Timer 0 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr00int(&self) -> Tmr00intR {
        Tmr00intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter/Timer 0 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr01int(&self) -> Tmr01intR {
        Tmr01intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter/Timer 1 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr10int(&self) -> Tmr10intR {
        Tmr10intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter/Timer 1 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr11int(&self) -> Tmr11intR {
        Tmr11intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter/Timer 2 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr20int(&self) -> Tmr20intR {
        Tmr20intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter/Timer 2 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr21int(&self) -> Tmr21intR {
        Tmr21intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter/Timer 3 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr30int(&self) -> Tmr30intR {
        Tmr30intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter/Timer 3 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr31int(&self) -> Tmr31intR {
        Tmr31intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter/Timer 4 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr40int(&self) -> Tmr40intR {
        Tmr40intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer 4 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr41int(&self) -> Tmr41intR {
        Tmr41intR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer 5 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr50int(&self) -> Tmr50intR {
        Tmr50intR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer 5 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr51int(&self) -> Tmr51intR {
        Tmr51intR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer 6 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr60int(&self) -> Tmr60intR {
        Tmr60intR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer 6 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr61int(&self) -> Tmr61intR {
        Tmr61intR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Counter/Timer 7 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr70int(&self) -> Tmr70intR {
        Tmr70intR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Counter/Timer 7 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr71int(&self) -> Tmr71intR {
        Tmr71intR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter/Timer 8 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr80int(&self) -> Tmr80intR {
        Tmr80intR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Counter/Timer 8 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr81int(&self) -> Tmr81intR {
        Tmr81intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Counter/Timer 9 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr90int(&self) -> Tmr90intR {
        Tmr90intR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Counter/Timer 9 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr91int(&self) -> Tmr91intR {
        Tmr91intR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Counter/Timer 10 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr100int(&self) -> Tmr100intR {
        Tmr100intR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Counter/Timer 10 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr101int(&self) -> Tmr101intR {
        Tmr101intR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Counter/Timer 11 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr110int(&self) -> Tmr110intR {
        Tmr110intR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Counter/Timer 11 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr111int(&self) -> Tmr111intR {
        Tmr111intR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Counter/Timer 12 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr120int(&self) -> Tmr120intR {
        Tmr120intR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer 12 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr121int(&self) -> Tmr121intR {
        Tmr121intR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer 13 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr130int(&self) -> Tmr130intR {
        Tmr130intR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer 13 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr131int(&self) -> Tmr131intR {
        Tmr131intR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer 14 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr140int(&self) -> Tmr140intR {
        Tmr140intR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Counter/Timer 14 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr141int(&self) -> Tmr141intR {
        Tmr141intR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Counter/Timer 15 interrupt based on CMP0."]
    #[inline(always)]
    pub fn tmr150int(&self) -> Tmr150intR {
        Tmr150intR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Counter/Timer 15 interrupt based on CMP1."]
    #[inline(always)]
    pub fn tmr151int(&self) -> Tmr151intR {
        Tmr151intR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter/Timer 0 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr00int(&mut self) -> Tmr00intW<IntenSpec> {
        Tmr00intW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter/Timer 0 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr01int(&mut self) -> Tmr01intW<IntenSpec> {
        Tmr01intW::new(self, 1)
    }
    #[doc = "Bit 2 - Counter/Timer 1 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr10int(&mut self) -> Tmr10intW<IntenSpec> {
        Tmr10intW::new(self, 2)
    }
    #[doc = "Bit 3 - Counter/Timer 1 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr11int(&mut self) -> Tmr11intW<IntenSpec> {
        Tmr11intW::new(self, 3)
    }
    #[doc = "Bit 4 - Counter/Timer 2 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr20int(&mut self) -> Tmr20intW<IntenSpec> {
        Tmr20intW::new(self, 4)
    }
    #[doc = "Bit 5 - Counter/Timer 2 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr21int(&mut self) -> Tmr21intW<IntenSpec> {
        Tmr21intW::new(self, 5)
    }
    #[doc = "Bit 6 - Counter/Timer 3 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr30int(&mut self) -> Tmr30intW<IntenSpec> {
        Tmr30intW::new(self, 6)
    }
    #[doc = "Bit 7 - Counter/Timer 3 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr31int(&mut self) -> Tmr31intW<IntenSpec> {
        Tmr31intW::new(self, 7)
    }
    #[doc = "Bit 8 - Counter/Timer 4 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr40int(&mut self) -> Tmr40intW<IntenSpec> {
        Tmr40intW::new(self, 8)
    }
    #[doc = "Bit 9 - Counter/Timer 4 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr41int(&mut self) -> Tmr41intW<IntenSpec> {
        Tmr41intW::new(self, 9)
    }
    #[doc = "Bit 10 - Counter/Timer 5 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr50int(&mut self) -> Tmr50intW<IntenSpec> {
        Tmr50intW::new(self, 10)
    }
    #[doc = "Bit 11 - Counter/Timer 5 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr51int(&mut self) -> Tmr51intW<IntenSpec> {
        Tmr51intW::new(self, 11)
    }
    #[doc = "Bit 12 - Counter/Timer 6 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr60int(&mut self) -> Tmr60intW<IntenSpec> {
        Tmr60intW::new(self, 12)
    }
    #[doc = "Bit 13 - Counter/Timer 6 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr61int(&mut self) -> Tmr61intW<IntenSpec> {
        Tmr61intW::new(self, 13)
    }
    #[doc = "Bit 14 - Counter/Timer 7 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr70int(&mut self) -> Tmr70intW<IntenSpec> {
        Tmr70intW::new(self, 14)
    }
    #[doc = "Bit 15 - Counter/Timer 7 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr71int(&mut self) -> Tmr71intW<IntenSpec> {
        Tmr71intW::new(self, 15)
    }
    #[doc = "Bit 16 - Counter/Timer 8 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr80int(&mut self) -> Tmr80intW<IntenSpec> {
        Tmr80intW::new(self, 16)
    }
    #[doc = "Bit 17 - Counter/Timer 8 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr81int(&mut self) -> Tmr81intW<IntenSpec> {
        Tmr81intW::new(self, 17)
    }
    #[doc = "Bit 18 - Counter/Timer 9 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr90int(&mut self) -> Tmr90intW<IntenSpec> {
        Tmr90intW::new(self, 18)
    }
    #[doc = "Bit 19 - Counter/Timer 9 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr91int(&mut self) -> Tmr91intW<IntenSpec> {
        Tmr91intW::new(self, 19)
    }
    #[doc = "Bit 20 - Counter/Timer 10 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr100int(&mut self) -> Tmr100intW<IntenSpec> {
        Tmr100intW::new(self, 20)
    }
    #[doc = "Bit 21 - Counter/Timer 10 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr101int(&mut self) -> Tmr101intW<IntenSpec> {
        Tmr101intW::new(self, 21)
    }
    #[doc = "Bit 22 - Counter/Timer 11 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr110int(&mut self) -> Tmr110intW<IntenSpec> {
        Tmr110intW::new(self, 22)
    }
    #[doc = "Bit 23 - Counter/Timer 11 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr111int(&mut self) -> Tmr111intW<IntenSpec> {
        Tmr111intW::new(self, 23)
    }
    #[doc = "Bit 24 - Counter/Timer 12 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr120int(&mut self) -> Tmr120intW<IntenSpec> {
        Tmr120intW::new(self, 24)
    }
    #[doc = "Bit 25 - Counter/Timer 12 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr121int(&mut self) -> Tmr121intW<IntenSpec> {
        Tmr121intW::new(self, 25)
    }
    #[doc = "Bit 26 - Counter/Timer 13 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr130int(&mut self) -> Tmr130intW<IntenSpec> {
        Tmr130intW::new(self, 26)
    }
    #[doc = "Bit 27 - Counter/Timer 13 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr131int(&mut self) -> Tmr131intW<IntenSpec> {
        Tmr131intW::new(self, 27)
    }
    #[doc = "Bit 28 - Counter/Timer 14 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr140int(&mut self) -> Tmr140intW<IntenSpec> {
        Tmr140intW::new(self, 28)
    }
    #[doc = "Bit 29 - Counter/Timer 14 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr141int(&mut self) -> Tmr141intW<IntenSpec> {
        Tmr141intW::new(self, 29)
    }
    #[doc = "Bit 30 - Counter/Timer 15 interrupt based on CMP0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr150int(&mut self) -> Tmr150intW<IntenSpec> {
        Tmr150intW::new(self, 30)
    }
    #[doc = "Bit 31 - Counter/Timer 15 interrupt based on CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn tmr151int(&mut self) -> Tmr151intW<IntenSpec> {
        Tmr151intW::new(self, 31)
    }
}
#[doc = "Set bits in this register to allow this module to generate the corresponding interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
