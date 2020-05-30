# Notes
 - Unit of measurement: "Stave Space" - The distance between two stave
   lines (this is similar to what an "em" is for text typesetting).  I'm going
   to abbreviate it as SS.
 - I'm going to use the term "centerline" for the position of the middle stave
   line, "ascent" for the number of SS above the "centerline", and "descent" for
   the number of SS below the "centerline".
 - Indent a clef one SS from the start of the stave lines.

# Stem Direction
An alternative style for a stave with lyrics is all stems are up.

## Non-Beamed Single Stave Stem Direction
### One Voice
 - Above Center Line: Down
 - Below Center Line: Up
 - Center Line - 5 cases:
   1. Previous and next note are down: Down
   2. Previous and next note are up: Up
   3. Previous or next in the same beat or half bar is up: Up
   4. Previous or next in the same beat or half bar is down: Down
   5. Previous and next are in the same beat or half bar and don't
      match direction: Down

### Interval
For most instruments (other than string), the notes must have the same
duration.

 - Same as Simple Stem Direction for the furthest note from the center
   line in the chord.
 - If both are same distance from center, then it should be determined
   as though a note is on the center line (same 5 cases).

### Chord
 - Same as Single-Stave Interval Stem Direction, except
 - When the highest and lowest notes are the same distance from the
   center, then the direction is determined by whether or not the
   majority or the notes are above or below the middle stave line.
 - If the same number are above and below the middle stave line, then
   use the inner notes distances, as though they were outer notes.
 - If all notes are same distance from the center, then it should be
   determined as though a note is on the center line (same 5 cases).

# Dynamics
 - m as in mf/mp should be 1 stave space high
 - dynamic instructions such as cresc., dim. should be same size, italic but not
   bold
 - 1 stave, no lyrics => dynamics below stave
 - 1 stave, lyrics => dynamics above stave
 - 2 staves (joined by brackets), => dynamics between staves
 - 1 voice / all voices: center dynamics on notehead
 - middle voice: left of notehead
 - placement is as close to notehead as possible
   - not on stave (all other symbols required to be closer to note)
 - move to dynamic to the left if it can't fit with notes and other dynamics
   - dynamic may cover a barline

# Articulation
 - By default, above or below stave, whichever is closest to the noteheads
 - (^) Strong accent always goes above the stave
 - Lyrics or a lot of notes below the stave, accent goes above the stave
 - For 2 staves, each part requires it's own articulation marks, even when same
   (Articulation in 2 staves goes below the stem for the lower part and above
   the stem for the upper part)
 - Center on notehead (unless staccato or staccatisimo by itself, then center on
   stem)
 - Place in next clear space (unless a series of accents), except for (^)
   staccato and (') staccatissimo which intersect the stave lines.

# Lyrics / Text
 - Always place beneath the lowest marking on the stave
 - Use "instrumental" rhythmic spacing, unless the text is short for a longer
   duration, than adjust (avoid spaces between syllables that are too wide)
   - Shouldn't be able to put another average-width word in the space.

# Margins
## Clef
 - Indented by 1 S.S.
 - Clef changes are drawn at 2/3 size.

## Notes
 - Margins between notes should be an average of (between noteheads, and between
   stems) to give an illusion of evenness. {only works on 1 stave}
 - Minimum of 1/2 S.S. apart
 - "Correct" direction stems should line up across staves

### Rhythmic Spacing
| Duration       | Variable units |
|----------------|----------------|
| Whole Note     | 7              |
| Dotted Half    | 6              |
| Half           | 5              |
| Dotted Quarter | 4              |
| Quarter        | 3.5            |
| Dotted Eighth  | 3              |
| Eighth         | 2.5            |
| 16th           | 2              |
| (128th)        | (1.75)         |

## Measures
 - Measure distance must not vary wildly from one measure to the next.

### Beginning of Stave Kerning
 - Clef-KeySig => 1-1¼
 - KeySig-TimeSig => 1-1½
 - TimeSig-Note => 2
 - Clef|KeySig-Note => 2½
 - 1 Accidental: -1, 2 Accidentals: -1½ (But not evaluating to less than 1)

### Barline
 - 1 stave space on either side

## Staves
 - Staves should be at least 1 stave height apart
 - Systems should be farther apart than the greatest distance between staves
 - Each beat, calculate required distance for the measure to discover required
   distance for the page (based on possible conflicts)
 - Consistent top and bottom of staves throghout pages

## Beams
 - 1/2 SS thickness
 - 1/4 SS distance between beams
 - stems go through all beams

### Short Durations
 - Inner beams are length of a notehead

### 
 - Beams must be attached to top or bottom of stave line (or centered)
 - 2-3 notes: 1 SS vertical difference due to angle maximum
 - 4+ notes: 2 SS vertical difference due to angle maximum
 - Stem length 3.5 (same) notehead in space, 3.25 (short) notehead on stave line
 - Shortest stem determines start of beam, other stems must be made longer
   - Shorten stems minimum (ledger lines): 2.5 SS
   - Shorten stems minimum (on the stave): 3 SS
 - Beams never closer than 2.5 SS from notehead
 - Horizontally closer notes 1/4 or 1/2 SS regardless of interval
 - Larger intervals have greater distance between beam Y coordinates
 - 3 beams must always either not slant or slant full SS
 - Outer notes of beamed group determine angle of beam
 - Horizontal on repeated patterns of pitches and inner note closer to beam than
   any of the outer notes
 - For chords, use notes closest to the beam to decide angle
 - One note different from all the same doesn't change angle if it's the
   furthest from the beam

### Beams Deciding Stem Direction
 - Unequal # on either side: For ends on either side of the middle stave line,
   the majority of notes determine the direction.
 - Equal # on either side: Furthest note from middle stave line decides.
 - Unequal #, No majority: Left Rest: Right determines direction, Right Rest:
   Left Determines Direction, Non-matching: Default to down stem
 - Wide intervals: 2.5 SS to beam both directions, beam in middle
 - Consecutive "close to spanning a whole stave" groups should go the same
   direction

## Accidentals
 - Accidentals should be as close to note as possible (1 SS for flat/natural,
   and extra space for sharp to avoid collision) (top to bottom, lower
   collisions go left)
 - Longer durations can fit accidentals in without distorting rhythmic spacing

## Slur
 - Centered on noteheads
 - Outside beam
 - Consistent and flat (close to stave as possible)
 - If stem direction is same, slur goes above or below, whichever is closest to
   noteheads
 - Mixed direction goes above, unless a beam is in the way, except for when
   there's dynamic markings below the stave.
 - Slur as if all notes stemmed

## Glissando
 - Thin straight line prefered to wavy line
 - Text gliss. or port. next to it
 - Line goes through pitches on the stave (stopped short for accidentals)

