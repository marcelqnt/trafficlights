use lotus_script::prelude::*;
use lotus_script::traffic::VAR_TRAFFICLIGHT_PHASE;

const EXP_APP_K_INCREASE: f32 = 40.0;
const EXP_APP_K_DECREASE: f32 = 11.0;

const VAR_AUS_WENN_HALT: &str = "$AusWennHalt";
const VAR_SIGNALROUTE_ACTIVE: &str = "signalroute_active";
const VAR_SIGNALSTATE: &str = "signalstate";
const VAR_LIGHT_HALT: &str = "light_halt";
const VAR_LIGHT_FAHRT: &str = "light_fahrt";

/// Bit 0 erzwingt das Haltlicht, Bit 1 das Fahrtlicht.
const SIGNALSTATE_HALT: i64 = 1;
const SIGNALSTATE_FAHRT: i64 = 2;

/// Achtlinsiges Fahrsignal mit getrenntem Halt- und Fahrtlicht.
///
/// Fahrt leuchtet in den Grünphasen, bei aktiver Fahrstraße oder bei
/// gesetztem Fahrt-Bit. Halt leuchtet, solange keine Fahrt aus Phase oder
/// Fahrstraße anliegt, außer `$AusWennHalt` blendet dieses Haltbild aus.
/// Das Halt-Bit erzwingt das Haltlicht zusätzlich.
pub struct MyScript {
    light_halt: Lamp,
    light_fahrt: Lamp,
}

script!(MyScript);

impl Default for MyScript {
    fn default() -> Self {
        Self {
            light_halt: Lamp::new(VAR_LIGHT_HALT),
            light_fahrt: Lamp::new(VAR_LIGHT_FAHRT),
        }
    }
}

impl Script for MyScript {
    fn tick(&mut self) {
        let phase = TrafficLightPhase::get_var(VAR_TRAFFICLIGHT_PHASE)
            .unwrap_or(TrafficLightPhase::OffPrimary);
        let proceed = matches!(
            phase,
            TrafficLightPhase::Green1 | TrafficLightPhase::Green2 | TrafficLightPhase::Green3
        );
        let route = bool::get_var(VAR_SIGNALROUTE_ACTIVE);
        let state = i64::get_var(VAR_SIGNALSTATE);
        let aus_wenn_halt = bool::get_var(VAR_AUS_WENN_HALT);
        let (halt, fahrt) = aspect(proceed, route, state, aus_wenn_halt);
        self.light_halt.approach(halt);
        self.light_fahrt.approach(fahrt);
    }
}

fn aspect(proceed: bool, route: bool, signalstate: i64, aus_wenn_halt: bool) -> (bool, bool) {
    let fahrt = proceed || route || (signalstate & SIGNALSTATE_FAHRT) != 0;
    let halt = (!aus_wenn_halt && !(proceed || route)) || (signalstate & SIGNALSTATE_HALT) != 0;
    (halt, fahrt)
}

struct Lamp {
    name: &'static str,
    value: f32,
}

impl Lamp {
    const fn new(name: &'static str) -> Self {
        Self { name, value: 0.0 }
    }

    fn approach(&mut self, on: bool) {
        let target = if on { 1.0 } else { 0.0 };
        let k = if target < self.value {
            EXP_APP_K_DECREASE
        } else {
            EXP_APP_K_INCREASE
        };
        let next = target + (self.value - target) * (-k * time::delta()).exp();
        if next != self.value {
            self.value = next;
            set_var(self.name, self.value);
        }
    }
}
