/* Timers */
/* Manual Page 965 */

use super::{common, pointer};

pub struct Timer {
    cr1:        *mut u32,       // Control Register 1
    cr2:        *mut u32,       // Control Register 2
    smcr:       *mut u32,       // Slave Mode Control Register
    dier:       *mut u32,       // DMA/ Interrupt Enable Register
    sr:         *mut u32,       // Status Register
    egr:        *mut u32,       // Event Generation Register
    ccmr1:      *mut u32,       // Capture/Compare Mode Register 1
    ccmr2:      *mut u32,       // Capture/Compare Mode Register 2
    ccer:       *mut u32,       // Capture/Compare Enable Register
    cnt:        *mut u32,       // Counter Register
    psc:        *mut u32,       // Prescaler Register
    arr:        *mut u32,       // Auto Reload Register
    rcr:        *mut u32,       // Repetition Counter Register
    ccr1:       *mut u32,       // Capture/Compare Register 1
    ccr2:       *mut u32,       // Capture/Compare Register 2
    ccr3:       *mut u32,       // Capture/Compare Register 3
    ccr4:       *mut u32,       // Capture/Compare Register 4
    dcr:        *mut u32,       // DMA Control Register
    dmar:       *mut u32,       // DMA Address Register
    or:         *mut u32,       // Option Register
}

/* Register Offset */
const CR1:      u32 = 0x00;
const CR2:      u32 = 0x04;
const SMCR:     u32 = 0x08;
const DIER:     u32 = 0x0C;
const SR:       u32 = 0x10;
const EGR:      u32 = 0x14;
const CCMR1:    u32 = 0x18;
const CCMR2:    u32 = 0x1C;
const CCER:     u32 = 0x20;
const CNT:      u32 = 0x24;
const PSC:      u32 = 0x28;
const ARR:      u32 = 0x2C;
const RCR:      u32 = 0x30;
const CCR1:     u32 = 0x34;
const CCR2:     u32 = 0x38;
const CCR3:     u32 = 0x3C;
const CCR4:     u32 = 0x40;
const DCR:      u32 = 0x48;
const DMAR:     u32 = 0x4C;
const OR:       u32 = 0x50;

/* Enumerations */
// 0 = Continous, 1 = ONS (clears en bit)
pub enum TimerType {Cont, Ons}

// 0 = Upcounter, 1 = Downcounter
pub enum Direction {Upcount, Downcount}

/* Register Masks */
/* CR1 */
const CMS_MASK:         u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */
const CKD_MASK:         u32 = common::MASK_2_BIT;       /* Mode is mask required, here we set the mask to two bit 11 */

/* CCMR */
const CCS_MASK:         u32 = common::MASK_2_BIT;
const OCM_MASK:         u32 = common::MASK_3_BIT;

/* Register Bits */
/* CR1 */
const EN_BIT:           u32 = common::BIT_0;            /* 0 = Disabled, 1 = Enabled */
const UDIS_BIT:         u32 = common::BIT_1;            /* Update event, 0 = EN, 1 = DISABLED */
const URS_BIT:          u32 = common::BIT_2;            /* 0 = All events enables, 1 = Only OF or UF Events */
const OPM_BIT:          u32 = common::BIT_3;            /* 0 = Continous, 1 = ONS (clears en bit) */
const DIR_BIT:          u32 = common::BIT_4;            /* 0 = Upcounter, 1 = Downcounter (ONLY ACTIVE IF CMS = 00) */
const ARPE_BIT:         u32 = common::BIT_7;            /* 0 ARR = Not Buffered, 1 = Buffered */
const UIFREMAP_BIT:     u32 = common::BIT_11;           /* Output, 0 = Pulse, 1 = Toggle */

/* SR */
const UPDATE_BIT:       u32 = common::BIT_0;

/* CCER */
const CC1E_BIT:         u32 = common::BIT_0;            /* Capture/Compare Output Enable */
const CC1P_BIT:         u32 = common::BIT_1;            /* Capture/Compare Output Polarity */
const CC1NP_BIT:        u32 = common::BIT_3;            /* Capture/Compare Output Polarity */
const CC2E_BIT:         u32 = common::BIT_4;            /* Capture/Compare Output Enable */
const CC2P_BIT:         u32 = common::BIT_5;            /* Capture/Compare Output Polarity */
const CC2NP_BIT:        u32 = common::BIT_7;            /* Capture/Compare Output Polarity */
const CC3E_BIT:         u32 = common::BIT_8;            /* Capture/Compare Output Enable */
const CC3P_BIT:         u32 = common::BIT_9;            /* Capture/Compare Output Polarity */
const CC3NP_BIT:        u32 = common::BIT_11;           /* Capture/Compare Output Polarity */
const CC4E_BIT:         u32 = common::BIT_12;           /* Capture/Compare Output Enable */
const CC4P_BIT:         u32 = common::BIT_13;           /* Capture/Compare Output Polarity */
const CC4NP_BIT:        u32 = common::BIT_15;           /* Capture/Compare Output Polarity */

/* Register Offsets */
/* CR1 */
const CMS_OFFSET:       u32 = 5;                        /* 00 = Edge Aligned     01 = Center Aligned Down     10 = Center Aligned Up     11 - Center Aligned Up */
const CKD_OFFSET:       u32 = 8;                        /* 00 = Tdts = Tclk_int  01 = Tdts = 2*Tclk_int       10 = Tdts = 4*Tclk_int     11 - Reserved */

/* CCMR */
const CC1S_OFFSET:      u32 = 0;                        /* Capture Compare 1 Mode 00: CC2 channel is configured as output 01: CC1 channel is configured as input, IC1 is mapped on TI1 10: CC1 channel is configured as input, IC1 is mapped on TI1 11: CC1 channel is configured as input, IC1 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register) */
const OC1M_OFFSET:      u32 = 4;                        /* Output Compare 1 Mode 0110: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCR1 0111: PWM mode 1 - In upcounting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else inactive. */
const CC2S_OFFSET:      u32 = 8;                        /* Capture Compare 2 Mode 00: CC2 channel is configured as output 01: CC2 channel is configured as input, IC2 is mapped on TI2 10: CC2 channel is configured as input, IC2 is mapped on TI1 11: CC2 channel is configured as input, IC2 is mapped on TRC. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register) */
const OC2M_OFFSET:      u32 = 12;                       /* Output Compare 2 Mode 0110: PWM mode 1 - In upcounting, channel 2 is active as long as TIMx_CNT<TIMx_CCR2 0111: PWM mode 2 - In upcounting, channel 2 is inactive as long as TIMx_CNT<TIMx_CCR2 else active. In downcounting, channel 2 is active as long as TIMx_CNT>TIMx_CCR2 else inactive. */

/* CNT */
const CLEAR_CNT:        u32 = 0;

/* PWM */
const PWM_MODE1:        u32 = 6;
const PWM_MODE2:        u32 = 7;
const CCS_OUTPUT:       u32 = 0;

impl Timer {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Timer {
        return Timer {
            cr1:    (base + CR1)    as *mut u32,
            cr2:    (base + CR2)    as *mut u32,
            smcr:   (base + SMCR)   as *mut u32,
            dier:   (base + DIER)   as *mut u32,
            sr:     (base + SR)     as *mut u32,
            egr:    (base + EGR)    as *mut u32,
            ccmr1:  (base + CCMR1)  as *mut u32,
            ccmr2:  (base + CCMR2)  as *mut u32,
            ccer:   (base + CCER)   as *mut u32,
            cnt:    (base + CNT)    as *mut u32,
            psc:    (base + PSC)    as *mut u32,
            arr:    (base + ARR)    as *mut u32,
            rcr:    (base + RCR)    as *mut u32,
            ccr1:   (base + CCR1)   as *mut u32,
            ccr2:   (base + CCR2)   as *mut u32,
            ccr3:   (base + CCR3)   as *mut u32,
            ccr4:   (base + CCR4)   as *mut u32,
            dcr:    (base + DCR)    as *mut u32,
            dmar:   (base + DMAR)   as *mut u32,
            or:     (base + OR)     as *mut u32
        };
    }

    /* Open The Timer And Setup Function */
    pub fn open(&self, timer_type: TimerType, dir: Direction) {
        pointer::clr_ptr_vol_bit_u32(self.cr1, UDIS_BIT);
        pointer::clr_ptr_vol_bit_u32(self.cr1, URS_BIT);
    
        match timer_type {
            TimerType::Ons          =>      pointer::set_ptr_vol_bit_u32(self.cr1, OPM_BIT),
            TimerType::Cont         =>      pointer::clr_ptr_vol_bit_u32(self.cr1, OPM_BIT)
        }
    
        match dir {
            Direction::Downcount    =>      pointer::set_ptr_vol_bit_u32(self.cr1, DIR_BIT),
            Direction::Upcount      =>      pointer::clr_ptr_vol_bit_u32(self.cr1, DIR_BIT)
        }

        pointer::set_ptr_vol_u32(self.cr1, CMS_OFFSET, CMS_MASK, 0);
        pointer::clr_ptr_vol_bit_u32(self.cr1, ARPE_BIT);
        pointer::set_ptr_vol_u32(self.cr1, CKD_OFFSET, CKD_MASK, 0);
        pointer::clr_ptr_vol_bit_u32(self.cr1, UIFREMAP_BIT);
    }
    
    /* Get Interrupt Flag */
    pub fn get_flag(&self) -> bool {
        return pointer::get_ptr_vol_bit_u32(self.sr, UPDATE_BIT);
    }
    
    /* Clear Interrupt Flag */
    pub fn clr_flag(&self) {
        pointer::clr_ptr_vol_bit_u32(self.sr, UPDATE_BIT);
    }

    /* Start Timer */
    pub fn set_interrupt(&self) {
        pointer::set_ptr_vol_bit_u32(self.dier, UPDATE_BIT);
    }
    
    /* Read Counter */
    pub fn get_cnt(&self) -> u32 {
        return pointer::get_ptr_vol_raw_u32(self.cnt);
    }

    /* Clear Counter */
    pub fn clr_cnt(&self) {
        pointer::set_ptr_vol_raw_u32(self.cnt, CLEAR_CNT);
    }
    
    /* Start Timer */
    pub fn start(&self) {
        pointer::set_ptr_vol_bit_u32(self.cr1, EN_BIT);
    }
    
    /* Stop Timer */
    pub fn stop(&self,) {
        pointer::clr_ptr_vol_bit_u32(self.cr1, EN_BIT);
    }
    
    /* Set Time and Scaling Of The Timer */
    pub fn set_scl(&self, time: u32, clk_speed: u32, prescl: u32) {  
        let val;
        let psc;
        
        if prescl == 0 {
            val = (time * clk_speed) - 1;
            psc = prescl;
        } else {
            val = ((time * clk_speed) / prescl) - 1;
            psc = prescl - 1;
        }
    
        pointer::set_ptr_vol_raw_u32(self.psc, psc);
        pointer::set_ptr_vol_raw_u32(self.arr, val);
    }

    /* Simple Spin And Wait On A Timer */
    pub fn wait(&self) {
        while self.get_flag() == false {
            // BLANK WAIT, WORKS DUE TO VOLITILE READ
        }
        self.clr_flag();
    }

    /* Setup For PWM on CH 1 */
    pub fn set_pwm_ccr1(&self, cnt: u32) {
        pointer::set_ptr_vol_raw_u32(self.ccr1, cnt);
    }

    /* Setup For PWM on CH 2 */
    pub fn set_pwm_ccr2(&self, cnt: u32) {
        pointer::set_ptr_vol_raw_u32(self.ccr2, cnt);
    }

    /* Setup For PWM on CH 3 */
    pub fn set_pwm_ccr3(&self, cnt: u32) {
        pointer::set_ptr_vol_raw_u32(self.ccr3, cnt);
    }

    /* Setup For PWM on CH 4 */
    pub fn set_pwm_ccr4(&self, cnt: u32) {
        pointer::set_ptr_vol_raw_u32(self.ccr4, cnt);
    }

    /* Default Set Up For PWM */
    pub fn set_pwm_ch1(&self) {
        pointer::set_ptr_vol_u32(self.ccmr1, CC1S_OFFSET, CCS_MASK, CCS_OUTPUT);
        pointer::set_ptr_vol_u32(self.ccmr1, OC1M_OFFSET, OCM_MASK, PWM_MODE1);
        pointer::set_ptr_vol_bit_u32(self.ccer, CC1E_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC1P_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC1NP_BIT);
    }

    /* Default Set Up For PWM */
    pub fn set_pwm_ch2(&self) {
        pointer::set_ptr_vol_u32(self.ccmr1, CC2S_OFFSET, CCS_MASK, CCS_OUTPUT);
        pointer::set_ptr_vol_u32(self.ccmr1, OC2M_OFFSET, OCM_MASK, PWM_MODE1);
        pointer::set_ptr_vol_bit_u32(self.ccer, CC2E_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC2P_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC2NP_BIT);
    }

    /* Default Set Up For PWM */
    pub fn set_pwm_ch3(&self) {
        pointer::set_ptr_vol_u32(self.ccmr2, CC1S_OFFSET, CCS_MASK, CCS_OUTPUT);
        pointer::set_ptr_vol_u32(self.ccmr2, OC1M_OFFSET, OCM_MASK, PWM_MODE1);
        pointer::set_ptr_vol_bit_u32(self.ccer, CC3E_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC3P_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC3NP_BIT);
    }

    /* Default Set Up For PWM */
    pub fn set_pwm_ch4(&self) {
        pointer::set_ptr_vol_u32(self.ccmr2, CC2S_OFFSET, CCS_MASK, CCS_OUTPUT);
        pointer::set_ptr_vol_u32(self.ccmr2, OC2M_OFFSET, OCM_MASK, PWM_MODE1);
        pointer::set_ptr_vol_bit_u32(self.ccer, CC4E_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC4P_BIT);
        pointer::clr_ptr_vol_bit_u32(self.ccer, CC4NP_BIT);
    }

    pub fn delay(&self, time: u32, clk_speed: u32, prescl: u32) {
        self.set_scl(time, clk_speed, prescl);
        self.start();
        self.clr_flag();

        while !self.get_flag() {
            // SPIN HERE
        }

        self.stop();
        self.clr_cnt();
        self.clr_flag();
    }
}
