use lotus_script::prelude::*;

const EXP_APP_K_INCREASE: f32 = 40.0;
const EXP_APP_K_DECREASE: f32 = 11.0;
const FLASH_TIME: f32 = 1.4;
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
        let phase = TrafficLightPhase::get_var("trafficlight_phase")
            .unwrap_or(TrafficLightPhase::OffPrimary);

        let myphase = phase_active(phase);

        #[cfg(feature = "yellow")]
        let flash_on = if phase == TrafficLightPhase::OffSecondary {
            self.flashtime += time::delta();
            if self.flashtime > FLASH_TIME {
                self.flashtime = 0.0;
            }
            self.flashtime < FLASH_ON_TIME
        } else {
            self.flashtime = 0.0;
            false
        };

        #[cfg(not(feature = "yellow"))]
        let flash_on = false;

        self.exp_app_bool(myphase || flash_on);
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
    fn exp_app(&mut self, target: f32) {
        self.light = if target < self.light {
            target + (self.light - target) * (-EXP_APP_K_DECREASE * time::delta()).exp()
        } else {
            target + (self.light - target) * (-EXP_APP_K_INCREASE * time::delta()).exp()
        };
        set_var("light", self.light);
    }

    fn exp_app_bool(&mut self, target: bool) {
        self.exp_app(if target { 1.0 } else { 0.0 });
    }
}
