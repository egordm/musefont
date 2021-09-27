<!-- markdownlint-disable -->
<div align="center">
    <h1>MuseFont</h1>
    <p>
        <b>A sheet music layout and drawing library inspired by <a href="https://github.com/musescore/MuseScore">MuseScore.</a></b>
    </p>
    <br/>
</div>
<!-- markdownlint-enable -->

## Preview
16th notes with hook       |  16th notes with line
:-------------------------:|:-------------------------:
![](https://github.com/EgorDm/musefont/blob/master/assets/preview/demo1.png?raw=true)  |  ![](https://github.com/EgorDm/musefont/blob/master/assets/preview/demo2.png?raw=true)

## Features
* Code based score building interface
* Highly customizable and uses same settings as [MuseScore](https://github.com/musescore/MuseScore)
* Generic drawing interface to support multiple frontends (currently only [pathfinder](https://github.com/servo/pathfinder))
* Score fonts are swappable (choice from mscore, smufl, gootville)
    * Other fonts are supported if they contain a well formatted metadata file
* Automatic layouting of the notes, their hooks and lines

## Usage
## Building scores
Load font and MuseScore configuration files. (demo files are found in assets folder)
```rust
let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/smufl");
let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/fonts/mscore");
let config = font::FontMapping::load(&config).unwrap();
let font = font::load(&path, "mscore.ttf", &config).expect("Font must load!");
```

Set up an initial renderer state
```rust
let mut state = RendererState::new();
state.set_debug(true); // Enables debug drawing
painter.set_score_font(font.clone());
painter.set_dpi(96.);
painter.set_scale(6.); // scaling of the notes. 1 being the formal print size
```

Build the score
```rust
// Create the global score
let score = Score::new(font.clone());

// Define a musical instrument (part) and add staff to it
let part = Part::new(score.clone(), "Triangle".to_string());
let staff = Staff::new(score.clone());
score.insert_part(part.clone(), 0);
score.insert_staff(staff.clone(), &part, 0);

// Add a measure to the score 
let measure = Measure::new(score.clone());

// Add a single eighth note (chord/segment) at given position at time 0/4
let chord = Chord::new(score.clone()).with_mut_i(|mut chord| {
    chord.set_pos(Point2F::new(100., 100.));
    chord.set_duration_type(Duration::new(DurationType::Eighth, 0));
});
Measure::add_at(measure.clone(), chord.clone().into(), Fraction::new(0, 4));

// Add a single note to the built chord with default pitch
let note = Note::new(score.clone());
chord.borrow_mut_el().add(note.clone().into());

// Layout and render the measure
MeasureRenderer::layout(measure.clone());
MeasureRenderer::render(measure.clone(), &mut state, painter);
```

## Implementing a graphical frontend
A graphical frontend can be implemented fairly easy and consists a [Painter](https://github.com/EgorDm/musefont/blob/master/musescore/src/drawing/painter.rs#L5).

During rendering the [Renderer](https://github.com/EgorDm/musefont/blob/master/musescore/src/score/renderer/base.rs#L4) generates 
a bunch of instruction in the drawing order. It is up to the [Painter](https://github.com/EgorDm/musefont/blob/master/musescore/src/drawing/painter.rs#L5) 
to ensure that the instructions are executed at the right resolution, in the right shape and displayed after rendering.

Currently Path (polyline), Rect (rectangle), Symbol (a font glyph) and a Point (circle) are valid [Instuctions](https://github.com/EgorDm/musefont/blob/master/musescore/src/drawing/instruction.rs#L6)

## TODO
- [ ] Staff and Score layouting
- [ ] Note height layouting based on pitch
- [ ] ...

