//! A showcase of what the render can paint, to see it and not just
//! compile it: gradient and border, transforms that compose, a rotated
//! texture and the difference between blending a group and blending its pieces.

use crate::scene::*;
use crate::logic::{Context, Script};
use std::f32::consts::PI;

#[derive(Default)]
pub struct Showcase(usize);

impl Script for Showcase {
    fn scene(&mut self) -> Scene {
        let mut e = Scene::default();
        let angle = e.prop("angle", 0.0);
        let pulse = e.prop("pulse", 0.0);
        let sway = e.prop("sway", 0.0);
        e.behaviors = vec![
            Behavior::Advance { prop: angle, per_second: 0.9.into() },
            Behavior::Wave { prop: pulse, frequency: 2.2, amplitude: 0.14.into() },
            Behavior::Wave { prop: sway, frequency: 1.3, amplitude: 0.4.into() },
        ];
        let white = color(0.96, 0.97, 0.96);
        let ink = color(0.07, 0.075, 0.08);

        // A dark background so everything reads on any desktop.
        e.paint(Instr::Group { shadow: Some(Shadow { offset: (0.0.into(), 8.0.into()), blur: 24.0.into(), alpha: 0.3.into(), color: None }) });
        e.paint(Instr::Shape { shape: Shape::Rect { center: (360.0.into(), 186.0.into()), half_size: (350.0.into(), 176.0.into()), radius: 22.0.into() }, fusion: 0.0.into() });
        e.paint(Instr::Fill { paint: ink.clone().into(), alpha: 0.94.into(), rim: 0.04.into(), light: None, border: None, glass_spec: None });

        // 1 · gradient and border, on a box that spins on itself.
        e.paint(Instr::Group { shadow: None });
        e.paint(Instr::Shape {
            shape: Shape::Rect { center: (95.0.into(), 108.0.into()), half_size: (56.0.into(), 38.0.into()), radius: 16.0.into() }.rotated(angle * 0.35),
            fusion: 0.0.into(),
        });
        e.paint(Instr::Fill {
            paint: Paint::Gradient { radial: false, from: (45.0.into(), 70.0.into()), to: (145.0.into(), 146.0.into()), stops: vec![(0.0.into(), color(0.62, 0.84, 0.74)), (1.0.into(), color(0.55, 0.42, 0.92))] },
            alpha: 1.0.into(),
            rim: 0.0.into(),
            light: None,
            border: Some((2.5.into(), white.clone())),
            glass_spec: None,
        });

        // 2 · a clock: scale ∘ rotation ∘ rotation. The whole of it pulses; inside, the
        //     hands turn; and at the tip of the minute hand, a box turns the other way.
        let center = (255.0, 108.0);
        let here = || -> Point { (center.0.into(), center.1.into()) };
        e.paint(Instr::Transform(Some(Transform::at(here()).scale(1.0 + pulse, 1.0 + pulse))));
        e.paint(Instr::Solid { shape: Shape::ring(here(), 52.0, 3.0), color: white.clone(), alpha: 1.0.into(), glass_spec: None });
        for k in 0..12 {
            e.paint(Instr::Transform(Some(Transform::at(here()).rotate(k as f32 * PI / 6.0))));
            e.paint(Instr::Solid {
                shape: Shape::Segment { from: (center.0.into(), (center.1 - 45.0).into()), to: (center.0.into(), (center.1 - 39.0).into()), thickness: 2.0.into() },
                color: white.clone(),
                alpha: 0.55.into(), glass_spec: None,
            });
            e.paint(Instr::Transform(None));
        }
        e.paint(Instr::Transform(Some(Transform::at(here()).rotate(angle / 12.0))));
        e.paint(Instr::Solid { shape: Shape::Segment { from: here(), to: (center.0.into(), (center.1 - 22.0).into()), thickness: 4.0.into() }, color: white.clone(), alpha: 1.0.into(), glass_spec: None });
        e.paint(Instr::Transform(None));
        e.paint(Instr::Transform(Some(Transform::at(here()).rotate(angle.e()))));
        e.paint(Instr::Solid { shape: Shape::Segment { from: here(), to: (center.0.into(), (center.1 - 36.0).into()), thickness: 2.6.into() }, color: color(0.62, 0.84, 0.74), alpha: 1.0.into(), glass_spec: None });
        let tip: Point = (center.0.into(), (center.1 - 36.0).into());
        e.paint(Instr::Transform(Some(Transform::at(tip.clone()).rotate(angle * -3.0))));
        e.paint(Instr::Solid { shape: Shape::Rect { center: tip, half_size: (5.0.into(), 5.0.into()), radius: 1.5.into() }, color: color(0.98, 0.76, 0.32), alpha: 1.0.into(), glass_spec: None });
        e.paint(Instr::Transform(None));
        e.paint(Instr::Transform(None));
        e.paint(Instr::Transform(None));

        // 3 · a rotated texture, with a frame under the same transform: if
        //     box and texture did not go hand in hand, it would show.
        let label = (430.0, 108.0);
        e.paint(Instr::Transform(Some(Transform::at((label.0.into(), label.1.into())).rotate(sway.e()))));
        e.paint(Instr::Text { content: Content::Literal("pleamar".into()), at: (label.0.into(), label.1.into()), anchor: (0.5, 0.5), width: None, style: Style::new(22.0, white.clone()).weight(500), alpha: 1.0.into(), measure: None });
        e.paint(Instr::Solid {
            shape: Shape::Rect { center: (label.0.into(), label.1.into()), half_size: (64.0.into(), 20.0.into()), radius: 8.0.into() }.stroke(1.5),
            color: white.clone(),
            alpha: 0.5.into(), glass_spec: None,
        });
        e.paint(Instr::Transform(None));

        // 4 · blending the pieces (top) or blending the group (bottom). Behind, some
        //     stripes so you can see through.
        for k in 0..9 {
            let x = 560.0 + k as f32 * 13.0;
            e.paint(Instr::Solid { shape: Shape::Segment { from: (x.into(), 30.0.into()), to: (x.into(), 186.0.into()), thickness: 3.0.into() }, color: color(0.62, 0.84, 0.74), alpha: 0.8.into(), glass_spec: None });
        }
        let half = 0.5 + sway;
        let pair = |e: &mut Scene, y: f32, alpha: Expr| {
            e.paint(Instr::Solid { shape: Shape::circle((595.0.into(), y.into()), 26.0), color: color(0.96, 0.97, 0.96), alpha: alpha.clone(), glass_spec: None });
            e.paint(Instr::Solid { shape: Shape::circle((627.0.into(), y.into()), 26.0), color: color(0.93, 0.23, 0.2), alpha, glass_spec: None });
        };
        pair(&mut e, 68.0, half.clone());
        e.paint(Instr::Opacity(Some(half)));
        pair(&mut e, 148.0, 1.0.into());
        e.paint(Instr::Opacity(None));

        // A zone under the same transforms as what is seen: the label
        // sways, and the mouse hits where it is, not where it was.
        let z = e.zone_under(
            "label",
            Shape::Rect { center: (label.0.into(), label.1.into()), half_size: (64.0.into(), 20.0.into()), radius: 8.0.into() },
            1.0,
            vec![Transform::at((label.0.into(), label.1.into())).rotate(sway.e())],
        );
        let _ = z;

        // 5 · real text: it breaks into lines, cuts off with an ellipsis,
        //     mixes scripts and emoji, and aligns.
        let grey = color(0.62, 0.65, 0.64);
        let paragraph = "Shaped text: ligatures fi ffl, Arabic مرحبا بالعالم, Japanese こんにちは and emoji 🌊🎉🦀. This sentence is long on purpose, so that it does not fit in three lines and has to end in an ellipsis.";
        e.paint(Instr::Text { content: Content::Literal(paragraph.into()), at: (30.0.into(), 222.0.into()), anchor: (0.0, 0.0), width: Some(330.0.into()), style: Style::new(13.5, white.clone()).lines(3), alpha: 1.0.into(), measure: None });
        for (k, (a, t)) in [(TextAlign::Left, "on the left"), (TextAlign::Center, "in the center"), (TextAlign::Right, "on the right")].into_iter().enumerate() {
            e.paint(Instr::Text { content: Content::Literal(t.into()), at: (390.0.into(), (222.0 + k as f32 * 20.0).into()), anchor: (0.0, 0.0), width: Some(150.0.into()), style: Style::new(13.0, grey.clone()).align(a), alpha: 1.0.into(), measure: None });
        }
        e.paint(Instr::Solid { shape: Shape::Rect { center: (465.0.into(), 252.0.into()), half_size: (77.0.into(), 32.0.into()), radius: 6.0.into() }.stroke(1.0), color: grey.clone(), alpha: 0.35.into(), glass_spec: None });

        // 6 · images: an SVG, a PNG, and a tinted symbolic icon.
        let svg = e.image(ImageSource::Icon("firefox".into()), 48, 48);
        let png = e.image(ImageSource::File("/usr/share/icons/hicolor/256x256/apps/firefox.png".into()), 48, 48);
        let symbol = e.image(ImageSource::Icon("audio-volume-high-symbolic".into()), 40, 40);
        e.paint(Instr::Image { image: svg, target: (565.0.into(), 226.0.into(), 48.0.into(), 48.0.into()), alpha: 1.0.into(), tint: None, cell: None, fit: 0 });
        e.paint(Instr::Image { image: png, target: (620.0.into(), 226.0.into(), 48.0.into(), 48.0.into()), alpha: 1.0.into(), tint: None, cell: None, fit: 0 });
        e.paint(Instr::Transform(Some(Transform::at((585.0.into(), 292.0.into())).rotate(sway.e()))));
        e.paint(Instr::Image { image: symbol, target: (565.0.into(), 272.0.into(), 40.0.into(), 40.0.into()), alpha: 1.0.into(), tint: Some(color(0.62, 0.84, 0.74)), cell: None, fit: 0 });
        e.paint(Instr::Transform(None));

        // 7 · a box that grows with its label. The render measures the text and leaves the
        //     measurement in two properties; the box's width chases them with a
        //     spring. The logic only changes what it says.
        let tag = e.live_text("tag", "Hello");
        let (tag_w, tag_h) = e.measured("tag");
        let box_w = e.prop_with("tag.box", 60.0, Spring::LIVELY);
        e.behaviors.push(Behavior::Follow { prop: box_w, to: tag_w + 32.0 });
        e.paint(Instr::Group { shadow: None });
        e.paint(Instr::Shape { shape: Shape::Rect { center: (360.0.into(), 326.0.into()), half_size: (box_w * 0.5, tag_h * 0.5 + 7.0), radius: 16.0.into() }, fusion: 0.0.into() });
        e.paint(Instr::Fill { paint: color(0.62, 0.84, 0.74).into(), alpha: 1.0.into(), rim: 0.0.into(), light: None, border: None, glass_spec: None });
        e.paint(Instr::Text { content: Content::Live(tag), at: (360.0.into(), 326.0.into()), anchor: (0.5, 0.5), width: None, style: Style::new(15.0, color(0.07, 0.12, 0.10)).weight(500), alpha: 1.0.into(), measure: Some((tag_w, tag_h)) });

        e.surfaces = vec![Surface { height: 372, ..Default::default() }];
        e
    }
    fn on_event(&mut self, e: Event, c: &mut Context) {
        if matches!(e, Event::Alarm("start") | Event::Alarm("tag")) {
            const LABELS: [&str; 5] = ["Hello", "A label that is quite a lot longer", "🌊 pleamar", "Ok", "Measuring text from an expression"];
            c.text("tag", LABELS[self.0 % LABELS.len()]);
            self.0 += 1;
            c.alarm("tag", 1400);
        }
    }
}
