#[doc = "Register `SDIO1FCLKSEL` reader"]
pub type R = crate::R<Sdio1fclkselSpec>;
#[doc = "Register `SDIO1FCLKSEL` writer"]
pub type W = crate::W<Sdio1fclkselSpec>;
#[doc = "SDIO1 Functional Clock Source Selection. .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main Clock."]
    MainClk = 0,
    #[doc = "1: Main System PLL Clock."]
    MainSysPllClk = 1,
    #[doc = "2: SYSPLL0 AUX0_PLL_Clock."]
    Syspll0Aux0PllClock = 2,
    #[doc = "3: FFRO Clock."]
    FfroClk = 3,
    #[doc = "4: SYSPLL0 AUX1_PLL_Clock."]
    Syspll0Aux1PllClock = 4,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - SDIO1 Functional Clock Source Selection. ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::MainClk),
            1 => Some(Sel::MainSysPllClk),
            2 => Some(Sel::Syspll0Aux0PllClock),
            3 => Some(Sel::FfroClk),
            4 => Some(Sel::Syspll0Aux1PllClock),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Sel::MainClk
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn is_main_sys_pll_clk(&self) -> bool {
        *self == Sel::MainSysPllClk
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll_clock(&self) -> bool {
        *self == Sel::Syspll0Aux0PllClock
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll_clock(&self) -> bool {
        *self == Sel::Syspll0Aux1PllClock
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - SDIO1 Functional Clock Source Selection. ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainClk)
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn main_sys_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainSysPllClk)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux0_pll_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux0PllClock)
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux1_pll_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux1PllClock)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - SDIO1 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO1FCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SDIO1 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Sdio1fclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "SDIO1 FCLK selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio1fclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio1fclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdio1fclkselSpec;
impl crate::RegisterSpec for Sdio1fclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio1fclksel::R`](R) reader structure"]
impl crate::Readable for Sdio1fclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`sdio1fclksel::W`](W) writer structure"]
impl crate::Writable for Sdio1fclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO1FCLKSEL to value 0x07"]
impl crate::Resettable for Sdio1fclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
