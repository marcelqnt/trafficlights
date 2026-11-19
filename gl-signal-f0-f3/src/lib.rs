use lotus_script::prelude::*;
use lotus_script::traffic::VAR_TRAFFICLIGHT_PHASE;

const EXP_APP_K_INCREASE: f32 = 40.0;
const EXP_APP_K_DECREASE: f32 = 11.0;

/// Vierlampiges Fahrsignal F0–F3.
///
/// Oben links, oben Mitte, oben rechts, unten. Die Grünphasen des Tracks
/// wählen das Fahrbild, jede andere Phase zeigt Halt:
/// - Green1: F1 geradeaus (Mitte + unten)
/// - Green2: F2 rechts (rechts + unten)
/// - Green3: F3 links (links + unten)
/// - sonst: F0 Halt (links + rechts)
pub struct MyScript {
    light_u: Lamp,
    light_ol: Lamp,
    light_om: Lamp,
    light_or: Lamp,
}

script!(MyScript);

impl Default for MyScript {
    fn default() -> Self {
        Self {
            light_u: Lamp::new("light_U"),
            light_ol: Lamp::new("light_OL"),
            light_om: Lamp::new("light_OM"),
            light_or: Lamp::new("light_OR"),
        }
    }
}

impl Script for MyScript {
    fn tick(&mut self) {
        let phase = TrafficLightPhase::get_var(VAR_TRAFFICLIGHT_PHASE)
            .unwrap_or(TrafficLightPhase::OffPrimary);
        let (u, ol, om, or_lamp) = aspect(phase);
        self.light_u.approach(u);
        self.light_ol.approach(ol);
        self.light_om.approach(om);
        self.light_or.approach(or_lamp);
    }
}

/// Sollwerte in der Reihenfolge unten, oben links, oben Mitte, oben rechts.
fn aspect(phase: TrafficLightPhase) -> (bool, bool, bool, bool) {
    let straight = phase == TrafficLightPhase::Green1;
    let right = phase == TrafficLightPhase::Green2;
    let left = phase == TrafficLightPhase::Green3;
    let proceed = straight || right || left;
    (proceed, !(straight || right), straight, !(straight || left))
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
