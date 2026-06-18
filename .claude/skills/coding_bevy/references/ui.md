# Bevy 0.18 UI Patterns

Bevy UI uses a flexbox-style layout engine (Taffy). All UI nodes are ECS entities with
a `Node` component. Nothing in this section uses HTML/CSS — think of it as a pure Rust
flexbox.

Required features: `bevy_ui`, `bevy_text`, `default_font` (or the `2d` group).

---

## Core components

| Component | Purpose |
|-----------|---------|
| `Node` | Layout container — width, height, flex direction, padding, gap, etc. |
| `BackgroundColor` | Fills the node's background |
| `BorderColor` | Border colour — use `BorderColor::all(color)`, NOT `BorderColor(color)` |
| `BorderRadius` | Rounded corners — `BorderRadius::all(Val::Px(4.0))` |
| `Text` | Text content on a node |
| `TextFont` | Font size, font handle, weight |
| `TextColor` | Text colour |
| `Button` | Makes a node respond to click/hover interaction |
| `Visibility` | `Visibility::Hidden` / `Visibility::Visible` — toggling without despawning |

---

## Full-screen overlay

```rust
commands.spawn((
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(40.0)),
        row_gap: Val::Px(16.0),
        ..default()
    },
    BackgroundColor(Color::srgba(0.05, 0.07, 0.12, 0.97)),
));
```

---

## Spawning text

```rust
commands.spawn((
    Text::new("Hello!"),
    TextFont { font_size: 24.0, ..default() },
    TextColor(Color::WHITE),
));
```

`default_font` feature provides a built-in font — no asset file needed.

---

## Nested layout with `with_children`

```rust
commands
    .spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.12, 0.18)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Title"),
            TextFont { font_size: 28.0, ..default() },
            TextColor(Color::srgb(0.2, 0.6, 1.0)),
        ));

        parent.spawn((
            Text::new("Body text here."),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.85, 0.85, 0.9)),
        ));
    });
```

---

## Buttons

Add `Button` to a node to make it interactive. Read `Interaction` to respond to hover/click.

```rust
// Spawn a button
commands
    .spawn((
        Button,
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.2, 0.3)),
    ))
    .with_children(|btn| {
        btn.spawn((
            Text::new("Click me"),
            TextFont { font_size: 15.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });

// React to interaction
fn button_system(
    query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, children) in &query {
        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                color.0 = match interaction {
                    Interaction::Pressed  => Color::srgb(1.0, 1.0, 0.0),
                    Interaction::Hovered  => Color::srgb(0.8, 0.9, 1.0),
                    Interaction::None     => Color::WHITE,
                };
            }
        }
    }
}
```

---

## Updating text at runtime

Tag the text entity with a marker component, then query + mutate `Text` in a system:

```rust
#[derive(Component)]
struct ScoreText;

// Spawn
commands.spawn((ScoreText, Text::new("Score: 0"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE)));

// Update
fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() { return; }
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Score: {}", score.0);
    }
}
```

Use `is_changed()` to avoid rewriting the string every frame.

---

## Showing / hiding UI without despawning

```rust
fn toggle_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<MyPanel>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        if let Ok(mut vis) = query.single_mut() {
            *vis = match *vis {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible,
            };
        }
    }
}
```

---

## Cleaning up a UI tree

Spawn the root node with a marker component. `despawn()` in 0.18 removes the whole
hierarchy — no need for `despawn_recursive()` (which doesn't exist):

```rust
#[derive(Component)]
struct UiRoot;

fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<UiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
```

---

## Val — sizing units

| Value | Meaning |
|-------|---------|
| `Val::Px(n)` | Fixed pixel size |
| `Val::Percent(n)` | Percentage of parent |
| `Val::Auto` | Let the layout engine decide |
| `Val::Vw(n)` / `Val::Vh(n)` | Viewport width/height percentage |

---

## BorderColor — correct syntax

```rust
// WRONG — BorderColor is NOT a tuple struct
BorderColor(Color::WHITE)

// CORRECT
BorderColor::all(Color::WHITE)
// or per-side:
BorderColor { top: Color::WHITE, right: Color::WHITE, bottom: Color::WHITE, left: Color::WHITE }
```

---

## Children::iter() yields Entity, not &Entity

```rust
for child in children.iter() {  // child: Entity (Copy)
    if let Ok(mut text) = text_query.get_mut(child) { ... }
}
```

---

## Z-ordering: UI always renders above 2D world entities

UI nodes (with `Node`) render in a separate pass on top of all `Mesh2d` / `Sprite` entities,
regardless of z values. No z-fighting between UI and world geometry.

---

## Disjoint text queries

When a system queries `Text` for a tagged component AND untagged child text nodes,
use `Without` to avoid conflicts:

```rust
fn update(
    mut body: Query<&mut Text, With<BodyText>>,
    mut other: Query<&mut Text, Without<BodyText>>,
) { ... }
```
