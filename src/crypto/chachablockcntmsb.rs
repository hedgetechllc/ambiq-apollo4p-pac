#[doc = "Register `CHACHABLOCKCNTMSB` reader"]
pub type R = crate::R<ChachablockcntmsbSpec>;
#[doc = "Register `CHACHABLOCKCNTMSB` writer"]
pub type W = crate::W<ChachablockcntmsbSpec>;
#[doc = "Field `CHACHABLOCKCNTMSB` reader - bits 31:0 of CHACHA_BLOCK_CNT_MSB register."]
pub type ChachablockcntmsbR = crate::FieldReader<u32>;
#[doc = "Field `CHACHABLOCKCNTMSB` writer - bits 31:0 of CHACHA_BLOCK_CNT_MSB register."]
pub type ChachablockcntmsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_BLOCK_CNT_MSB register."]
    #[inline(always)]
    pub fn chachablockcntmsb(&self) -> ChachablockcntmsbR {
        ChachablockcntmsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - bits 31:0 of CHACHA_BLOCK_CNT_MSB register."]
    #[inline(always)]
    #[must_use]
    pub fn chachablockcntmsb(&mut self) -> ChachablockcntmsbW<ChachablockcntmsbSpec> {
        ChachablockcntmsbW::new(self, 0)
    }
}
#[doc = "The two first words (n) in the last row of the cipher matrix are the block counter. At the end of each block (512b), the block_cnt for the next block is written by HW to the block_cnt_lsb and block_cnt_msb registers. Need reset block counter , if start new message.\n\nYou can [`read`](crate::Reg::read) this register and get [`chachablockcntmsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chachablockcntmsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachablockcntmsbSpec;
impl crate::RegisterSpec for ChachablockcntmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chachablockcntmsb::R`](R) reader structure"]
impl crate::Readable for ChachablockcntmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`chachablockcntmsb::W`](W) writer structure"]
impl crate::Writable for ChachablockcntmsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHABLOCKCNTMSB to value 0"]
impl crate::Resettable for ChachablockcntmsbSpec {
    const RESET_VALUE: u32 = 0;
}
