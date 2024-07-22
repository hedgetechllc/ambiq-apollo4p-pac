#[doc = "Register `SIMOBODM` reader"]
pub type R = crate::R<SimobodmSpec>;
#[doc = "Register `SIMOBODM` writer"]
pub type W = crate::W<SimobodmSpec>;
#[doc = "Enable the gate into the interrupt block for the digital brownout detection on VDDC. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digboec {
    #[doc = "0: Enable brown out detection for VDDF using the analog method."]
    Boa = 0,
    #[doc = "1: Enable brown out detection for VDDF using the digital method."]
    Bod = 1,
}
impl From<Digboec> for bool {
    #[inline(always)]
    fn from(variant: Digboec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGBOEC` reader - Enable the gate into the interrupt block for the digital brownout detection on VDDC. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboecR = crate::BitReader<Digboec>;
impl DigboecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digboec {
        match self.bits {
            false => Digboec::Boa,
            true => Digboec::Bod,
        }
    }
    #[doc = "Enable brown out detection for VDDF using the analog method."]
    #[inline(always)]
    pub fn is_boa(&self) -> bool {
        *self == Digboec::Boa
    }
    #[doc = "Enable brown out detection for VDDF using the digital method."]
    #[inline(always)]
    pub fn is_bod(&self) -> bool {
        *self == Digboec::Bod
    }
}
#[doc = "Field `DIGBOEC` writer - Enable the gate into the interrupt block for the digital brownout detection on VDDC. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboecW<'a, REG> = crate::BitWriter<'a, REG, Digboec>;
impl<'a, REG> DigboecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable brown out detection for VDDF using the analog method."]
    #[inline(always)]
    pub fn boa(self) -> &'a mut crate::W<REG> {
        self.variant(Digboec::Boa)
    }
    #[doc = "Enable brown out detection for VDDF using the digital method."]
    #[inline(always)]
    pub fn bod(self) -> &'a mut crate::W<REG> {
        self.variant(Digboec::Bod)
    }
}
#[doc = "Enable the gate into the interrupt block for the digital brownout detection on VDDF. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digboef {
    #[doc = "0: Mask the VDDF digital brownout detection into the interrupt block."]
    Bom = 0,
    #[doc = "1: Enable the VDDF digital brownout detection into the interrupt block."]
    Boe = 1,
}
impl From<Digboef> for bool {
    #[inline(always)]
    fn from(variant: Digboef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGBOEF` reader - Enable the gate into the interrupt block for the digital brownout detection on VDDF. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboefR = crate::BitReader<Digboef>;
impl DigboefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digboef {
        match self.bits {
            false => Digboef::Bom,
            true => Digboef::Boe,
        }
    }
    #[doc = "Mask the VDDF digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_bom(&self) -> bool {
        *self == Digboef::Bom
    }
    #[doc = "Enable the VDDF digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_boe(&self) -> bool {
        *self == Digboef::Boe
    }
}
#[doc = "Field `DIGBOEF` writer - Enable the gate into the interrupt block for the digital brownout detection on VDDF. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboefW<'a, REG> = crate::BitWriter<'a, REG, Digboef>;
impl<'a, REG> DigboefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask the VDDF digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn bom(self) -> &'a mut crate::W<REG> {
        self.variant(Digboef::Bom)
    }
    #[doc = "Enable the VDDF digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn boe(self) -> &'a mut crate::W<REG> {
        self.variant(Digboef::Boe)
    }
}
#[doc = "Enable the gate into the interrupt block for the digital brownout detection on VDDS. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digboes {
    #[doc = "0: Mask the VDDS digital brownout detection into the interrupt block."]
    Bom = 0,
    #[doc = "1: Enable the VDDS digital brownout detection into the interrupt block."]
    Boe = 1,
}
impl From<Digboes> for bool {
    #[inline(always)]
    fn from(variant: Digboes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGBOES` reader - Enable the gate into the interrupt block for the digital brownout detection on VDDS. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboesR = crate::BitReader<Digboes>;
impl DigboesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digboes {
        match self.bits {
            false => Digboes::Bom,
            true => Digboes::Boe,
        }
    }
    #[doc = "Mask the VDDS digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_bom(&self) -> bool {
        *self == Digboes::Bom
    }
    #[doc = "Enable the VDDS digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_boe(&self) -> bool {
        *self == Digboes::Boe
    }
}
#[doc = "Field `DIGBOES` writer - Enable the gate into the interrupt block for the digital brownout detection on VDDS. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboesW<'a, REG> = crate::BitWriter<'a, REG, Digboes>;
impl<'a, REG> DigboesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask the VDDS digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn bom(self) -> &'a mut crate::W<REG> {
        self.variant(Digboes::Bom)
    }
    #[doc = "Enable the VDDS digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn boe(self) -> &'a mut crate::W<REG> {
        self.variant(Digboes::Boe)
    }
}
#[doc = "Enable the gate into the interrupt block for the digital brownout detection on VDDC_LV. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digboeclv {
    #[doc = "0: Mask the VDDC_LV digital brownout detection into the interrupt block."]
    Bom = 0,
    #[doc = "1: Enable brown VDDC_LV digital brownout detection into the interrupt block."]
    Boe = 1,
}
impl From<Digboeclv> for bool {
    #[inline(always)]
    fn from(variant: Digboeclv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGBOECLV` reader - Enable the gate into the interrupt block for the digital brownout detection on VDDC_LV. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboeclvR = crate::BitReader<Digboeclv>;
impl DigboeclvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Digboeclv {
        match self.bits {
            false => Digboeclv::Bom,
            true => Digboeclv::Boe,
        }
    }
    #[doc = "Mask the VDDC_LV digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_bom(&self) -> bool {
        *self == Digboeclv::Bom
    }
    #[doc = "Enable brown VDDC_LV digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn is_boe(&self) -> bool {
        *self == Digboeclv::Boe
    }
}
#[doc = "Field `DIGBOECLV` writer - Enable the gate into the interrupt block for the digital brownout detection on VDDC_LV. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
pub type DigboeclvW<'a, REG> = crate::BitWriter<'a, REG, Digboeclv>;
impl<'a, REG> DigboeclvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask the VDDC_LV digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn bom(self) -> &'a mut crate::W<REG> {
        self.variant(Digboeclv::Bom)
    }
    #[doc = "Enable brown VDDC_LV digital brownout detection into the interrupt block."]
    #[inline(always)]
    pub fn boe(self) -> &'a mut crate::W<REG> {
        self.variant(Digboeclv::Boe)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the gate into the interrupt block for the digital brownout detection on VDDC. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    pub fn digboec(&self) -> DigboecR {
        DigboecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the gate into the interrupt block for the digital brownout detection on VDDF. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    pub fn digboef(&self) -> DigboefR {
        DigboefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the gate into the interrupt block for the digital brownout detection on VDDS. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    pub fn digboes(&self) -> DigboesR {
        DigboesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the gate into the interrupt block for the digital brownout detection on VDDC_LV. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    pub fn digboeclv(&self) -> DigboeclvR {
        DigboeclvR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the gate into the interrupt block for the digital brownout detection on VDDC. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    #[must_use]
    pub fn digboec(&mut self) -> DigboecW<SimobodmSpec> {
        DigboecW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the gate into the interrupt block for the digital brownout detection on VDDF. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    #[must_use]
    pub fn digboef(&mut self) -> DigboefW<SimobodmSpec> {
        DigboefW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the gate into the interrupt block for the digital brownout detection on VDDS. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    #[must_use]
    pub fn digboes(&mut self) -> DigboesW<SimobodmSpec> {
        DigboesW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the gate into the interrupt block for the digital brownout detection on VDDC_LV. Note: The interrupt block must also be unmasked for ISR and interrupt status to be set"]
    #[inline(always)]
    #[must_use]
    pub fn digboeclv(&mut self) -> DigboeclvW<SimobodmSpec> {
        DigboeclvW::new(self, 3)
    }
}
#[doc = "This register unmasks the individual digital detection brownout bits into the interrupt block\n\nYou can [`read`](crate::Reg::read) this register and get [`simobodm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simobodm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SimobodmSpec;
impl crate::RegisterSpec for SimobodmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simobodm::R`](R) reader structure"]
impl crate::Readable for SimobodmSpec {}
#[doc = "`write(|w| ..)` method takes [`simobodm::W`](W) writer structure"]
impl crate::Writable for SimobodmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIMOBODM to value 0"]
impl crate::Resettable for SimobodmSpec {
    const RESET_VALUE: u32 = 0;
}
