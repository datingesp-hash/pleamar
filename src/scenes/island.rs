//! A k4-style island, with the same pieces as Marea and not a single new
//! line in the render. Here the logic neither animates nor decides anything: growing with the
//! mouse and releasing the drop on press are two layers and four rules.

use crate::scene::*;
use crate::logic::{Context, Script};

const CX: f32 = 360.0;
const TOP: f32 = 24.0;
const CLOSED: (f32, f32) = (190.0, 36.0);
const OPEN: (f32, f32) = (430.0, 92.0);

#[derive(Default)]
pub struct Island {
    step: bool,
    seconds: u32,
}

impl Script for Island {
    fn scene(&mut self) -> Scene {
        let mut e = Scene::default();
        let width = e.prop("island.width", CLOSED.0);
        let height = e.prop("island.height", CLOSED.1);
        let detail = e.prop("detail", 0.0);
        let drop = e.prop("drop", 0.0);
        let fusion = e.prop("fusion", 0.0);
        let pulse = e.prop("pulse", 0.0);

        let island = Shape::Rect {
            center: (CX.into(), TOP + height * 0.5),
            half_size: (width * 0.5, height * 0.5),
            radius: (height * 0.5).min(30.0),
        };
        let drop_x = CX + width * 0.5 - 18.0 + drop * 48.0;
        let drop_y = TOP + 18.0;

        e.paint(Instr::Group { shadow: Some(Shadow { offset: (0.0.into(), 8.0.into()), blur: 26.0.into(), alpha: 0.3.into(), color: None }) });
        e.paint(Instr::Shape { shape: island.clone(), fusion: 0.0.into() });
        e.paint(Instr::Shape { shape: Shape::circle((drop_x.clone(), drop_y.into()), 18.0 * drop.e().smoothstep(0.0, 0.35)), fusion: fusion.e() });
        e.paint(Instr::Fill { paint: color(0.04, 0.043, 0.045).into(), alpha: 1.0.into(), rim: 0.06.into(), light: None, border: None, glass_spec: None });

        // The drop's pilot light, which pulses by itself.
        e.paint(Instr::Solid {
            shape: Shape::circle((drop_x, drop_y.into()), 4.5 + pulse),
            color: color(0.95, 0.36, 0.32),
            alpha: drop.e().smoothstep(0.6, 1.0), glass_spec: None,
        });

        e.paint(Instr::Clip(Some((island.clone(), 1.0))));
        let t = detail.e().clamp(0.0, 1.0);
        let white = color(0.96, 0.97, 0.96);
        let time = e.live_text("time", "--:--");
        let elapsed = e.live_text("elapsed", "0:00 / 4:43");
        // The time travels from the center to its corner when the island grows.
        e.paint(Instr::Text {
            content: Content::Live(time),
            at: (mix(CX, CX - OPEN.0 * 0.5 + 84.0, t.clone()), mix(TOP + 18.0, TOP + 24.0, t.clone())),
            anchor: (0.5, 0.5),
            width: None,
            style: Style::new(15.0, white.clone()).weight(600),
            alpha: 0.95.into(), measure: None });
        // And next to it, the icon of whatever is playing. Finding it is the platform's business.
        let icon = e.image(ImageSource::Icon("firefox".into()), 28, 28);
        e.paint(Instr::Image { image: icon, target: ((CX - OPEN.0 * 0.5 + 22.0).into(), (TOP + 10.0).into(), 28.0.into(), 28.0.into()), alpha: t.clone(), tint: None, cell: None, fit: 0 });
        let left = CX - OPEN.0 * 0.5;
        e.paint(Instr::Text { content: Content::Literal("Tycho — Awake".into()), at: ((left + 26.0).into(), (TOP + 58.0).into()), anchor: (0.0, 0.5), width: Some(250.0.into()), style: Style::new(16.0, white.clone()).weight(500).lines(1), alpha: t.clone(), measure: None });
        e.paint(Instr::Text { content: Content::Literal("Now playing".into()), at: ((left + OPEN.0 - 26.0).into(), (TOP + 24.0).into()), anchor: (1.0, 0.5), width: None, style: Style::new(12.5, white.clone()), alpha: t.clone() * 0.55, measure: None });
        e.paint(Instr::Text { content: Content::Live(elapsed), at: ((left + OPEN.0 - 26.0).into(), (TOP + 76.0).into()), anchor: (1.0, 0.5), width: None, style: Style::new(12.5, white), alpha: t.clone() * 0.6, measure: None });
        e.paint(Instr::Solid {
            shape: Shape::Rect { center: ((CX - 60.0).into(), (TOP + 76.0).into()), half_size: (130.0.into(), 1.5.into()), radius: 1.5.into() },
            color: color(0.62, 0.84, 0.74),
            alpha: t, glass_spec: None,
        });
        e.paint(Instr::Clip(None));

        e.behaviors = vec![Behavior::Wave { prop: pulse, frequency: 5.0, amplitude: drop * 0.9 }];

        let hover = e.fact("hover", 0.0);
        let loose = e.fact("loose", 0.0);
        e.layer("size", Spring::LIVELY, vec![
            Claim::during("big", hover).sets(vec![
                go_to(width, OPEN.0, Spring::LIVELY, 0),
                go_to(height, OPEN.1, Spring::LIVELY, 40),
                go_to(detail, 1.0, Spring::LIVELY, 140),
            ]),
            Claim::fallback("small").sets(vec![
                go_to(detail, 0.0, Spring::LIVELY, 0),
                go_to(height, CLOSED.1, Spring::LIVELY, 60),
                go_to(width, CLOSED.0, Spring::LIVELY, 60),
            ]),
        ]);
        let neck = |a: f32| vec![go_to(fusion, 44.0, Spring::QUICK, 0), go_to(drop, a, Spring::CALM, 40), go_to(fusion, 0.0, Spring::CALM, 520)];
        e.layer("drop", Spring::CALM, vec![
            Claim::during("out", loose).sets(neck(1.0)),
            Claim::fallback("in").sets(neck(0.0)),
        ]);

        let z = e.zone("island", island, 1.0);
        e.rule(Trigger::Enter(z), vec![Effect::Fact(hover, 1.0.into())]);
        e.rule(Trigger::Leave(z), vec![Effect::Fact(hover, 0.0.into())]);
        e.rule(Trigger::Press(z), vec![Effect::Toggle(loose)]);

        e.surfaces = vec![Surface { height: 140, ..Default::default() }];
        e
    }

    fn on_event(&mut self, e: Event, c: &mut Context) {
        match e {
            //  A clock: every second the logic says what time it is, and nothing more.
            Event::Alarm("start") | Event::Alarm("second") => {
                let now = chrono::Local::now();
                c.text("time", now.format("%H:%M").to_string());
                self.seconds += 1;
                c.text("elapsed", format!("{}:{:02} / 4:43", (134 + self.seconds) / 60, (134 + self.seconds) % 60));
                c.alarm("second", 1000);
            }
            Event::Layer("drop", _) => c.work(),
            // Without a mouse there is no zone to open it: the demo sets the facts itself.
            Event::Demo => {
                self.step = !self.step;
                c.fact("hover", self.step);
                c.fact("loose", self.step);
            }
            _ => {}
        }
    }
}
