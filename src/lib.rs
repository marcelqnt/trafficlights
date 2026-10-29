use lotus_script::prelude::*;

const EXP_APP_K_INCREASE: f32 = 40.0;
const EXP_APP_K_DECREASE: f32 = 11.0;

#[derive(Default)]
pub struct MyScript {
    light: f32,
}

script!(MyScript);

impl Script for MyScript {
    fn tick(&mut self) {
        let phase = if let Some(phase) = TrafficLightPhase::get_var("trafficlight_phase") {
            phase_active(phase)
        } else {
            true
        };
        self.exp_app_bool(phase);
    }
}

fn phase_active(phase: TrafficLightPhase) -> bool {
    let yellow_a = phase == TrafficLightPhase::YellowA1
        || phase == TrafficLightPhase::YellowA2
        || phase == TrafficLightPhase::YellowA3;
    #[cfg(feature = "yellow")]
    {
        yellow_a
            || phase == TrafficLightPhase::YellowB1
            || phase == TrafficLightPhase::YellowB2
            || phase == TrafficLightPhase::YellowB3
    }
    #[cfg(not(feature = "yellow"))]
    {
        yellow_a
            || phase == TrafficLightPhase::Red1
            || phase == TrafficLightPhase::Red2
            || phase == TrafficLightPhase::Red3
    }
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
