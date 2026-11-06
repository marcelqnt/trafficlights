use lotus_script::prelude::*;
use lotus_script::traffic::VAR_TRAFFICLIGHT_PHASE;

const EXP_APP_K_INCREASE: f32 = 40.0;
const EXP_APP_K_DECREASE: f32 = 11.0;
/// Unter dieser Abweichung gilt die Lampe als aus- oder eingeschnappt.
const SETTLE_EPSILON: f32 = 0.004;

#[cfg(all(feature = "yellow", feature = "hh"))]
const FLASH_TIME: f32 = 0.9;
#[cfg(all(feature = "yellow", feature = "hh"))]
const FLASH_ON_TIME: f32 = 0.4;
#[cfg(all(feature = "yellow", not(feature = "hh")))]
const FLASH_TIME: f32 = 1.4;
#[cfg(all(feature = "yellow", not(feature = "hh")))]
const FLASH_ON_TIME: f32 = 0.7;

#[derive(Default)]
pub struct MyScript {
    light: f32,
    #[cfg(feature = "yellow")]
    flashtime: f32,
}

script!(MyScript);

impl Script for MyScript {
    fn tick(&mut self) {
        let phase = current_phase();
        let lamp_on = if is_blinking(phase) {
            self.flash_on()
        } else {
            self.reset_flash();
            phase_active(phase)
        };
        self.approach(if lamp_on { 1.0 } else { 0.0 }, !is_blinking(phase));
    }
}

fn current_phase() -> TrafficLightPhase {
    #[cfg(feature = "flashlight")]
    {
        TrafficLightPhase::OffSecondary
    }
    #[cfg(not(feature = "flashlight"))]
    {
        TrafficLightPhase::get_var(VAR_TRAFFICLIGHT_PHASE).unwrap_or(TrafficLightPhase::OffPrimary)
    }
}

fn is_blinking(phase: TrafficLightPhase) -> bool {
    #[cfg(feature = "yellow")]
    {
        phase == TrafficLightPhase::OffSecondary
    }
    #[cfg(not(feature = "yellow"))]
    {
        let _ = phase;
        false
    }
}

fn phase_active(phase: TrafficLightPhase) -> bool {
    #[cfg(feature = "red")]
    let red = phase == TrafficLightPhase::YellowA1
        || phase == TrafficLightPhase::YellowA2
        || phase == TrafficLightPhase::YellowA3
        || phase == TrafficLightPhase::Red1
        || phase == TrafficLightPhase::Red2
        || phase == TrafficLightPhase::Red3;
    #[cfg(not(feature = "red"))]
    let red = false;

    #[cfg(feature = "yellow")]
    let yellow = phase == TrafficLightPhase::YellowA1
        || phase == TrafficLightPhase::YellowA2
        || phase == TrafficLightPhase::YellowA3
        || phase == TrafficLightPhase::YellowB1
        || phase == TrafficLightPhase::YellowB2
        || phase == TrafficLightPhase::YellowB3;
    #[cfg(not(feature = "yellow"))]
    let yellow = false;

    #[cfg(feature = "green")]
    let green = phase == TrafficLightPhase::Green1
        || phase == TrafficLightPhase::Green2
        || phase == TrafficLightPhase::Green3
        || phase == TrafficLightPhase::YellowB3;
    #[cfg(not(feature = "green"))]
    let green = false;

    red || yellow || green
}

impl MyScript {
    fn reset_flash(&mut self) {
        #[cfg(feature = "yellow")]
        {
            self.flashtime = 0.0;
        }
    }

    fn flash_on(&mut self) -> bool {
        #[cfg(feature = "yellow")]
        {
            self.flashtime += time::delta();
            if self.flashtime > FLASH_TIME {
                self.flashtime = 0.0;
            }
            self.flashtime < FLASH_ON_TIME
        }
        #[cfg(not(feature = "yellow"))]
        {
            false
        }
    }

    fn approach(&mut self, target: f32, may_sleep: bool) {
        let mut next = if target < self.light {
            target + (self.light - target) * (-EXP_APP_K_DECREASE * time::delta()).exp()
        } else {
            target + (self.light - target) * (-EXP_APP_K_INCREASE * time::delta()).exp()
        };
        if may_sleep && (next - target).abs() < SETTLE_EPSILON {
            next = target;
        }
        let settled = may_sleep && next == target;
        if next != self.light || settled {
            self.light = next;
            set_var("light", self.light);
        }
        // if settled {
        //     time::idle_until_var_changes(VAR_TRAFFICLIGHT_PHASE);
        // }
    }
}
