use bevy::prelude::*;

use crate::state::GameState;

// ── Data ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Email {
    pub from: &'static str,
    pub subject: &'static str,
    pub body: &'static str,
    pub replies: [&'static str; 3],
}

const INBOX: &[Email] = &[
    Email {
        from: "boss@initech.com",
        subject: "TPS Reports",
        body: "Hey, just a reminder — all TPS reports need\nthe new cover sheet going forward.\nDid you get the memo?",
        replies: [
            "Yes, I'll update mine right away.",
            "I haven't seen the memo, can you resend it?",
            "On it. Sorry for the confusion.",
        ],
    },
    Email {
        from: "hr@initech.com",
        subject: "Office Supplies Request",
        body: "We are updating the office supply order.\nPlease reply with any items you need\nbefore end of day Friday.",
        replies: [
            "Just printer paper and a new stapler, thanks.",
            "I'm good for now, nothing needed.",
            "Could I get some sticky notes and pens?",
        ],
    },
    Email {
        from: "facilities@initech.com",
        subject: "Kitchen Cleaning Schedule",
        body: "Attached is this month's kitchen cleaning rota.\nYour assigned day is Wednesday.\nPlease confirm receipt.",
        replies: [
            "Got it, Wednesday works for me.",
            "I have a conflict on Wednesday — can we swap?",
            "Confirmed. I'll make sure the kitchen is clean.",
        ],
    },
    Email {
        from: "alice@initech.com",
        subject: "Lunch today?",
        body: "Hey! A few of us are heading to the sandwich\nplace around noon. Want to join?",
        replies: [
            "Sounds great, I'll be there!",
            "Sorry, I'm swamped today — maybe next time.",
            "I'm in! What time exactly?",
        ],
    },
];

// ── Resource ──────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct EmailState {
    pub current: usize,
    pub all_done: bool,
}

impl EmailState {
    pub fn current_email(&self) -> Option<&'static Email> {
        INBOX.get(self.current)
    }

    pub fn advance(&mut self) {
        self.current += 1;
        if self.current >= INBOX.len() {
            self.all_done = true;
        }
    }

    pub fn total(&self) -> usize {
        INBOX.len()
    }
}

// ── UI marker components ──────────────────────────────────────────────────────

#[derive(Component)]
struct EmailBodyText;

#[derive(Component)]
struct ReplyText(usize);

#[derive(Component)]
struct DoneText;

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct EmailPlugin;

impl Plugin for EmailPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmailState>()
            .add_systems(OnEnter(GameState::EmailMinigame), setup_email_ui)
            .add_systems(OnExit(GameState::EmailMinigame), cleanup_email_ui)
            .add_systems(
                Update,
                (handle_reply_input, update_email_display)
                    .chain()
                    .run_if(in_state(GameState::EmailMinigame)),
            );
    }
}

// ── Marker for cleanup ────────────────────────────────────────────────────────

#[derive(Component)]
struct EmailUiRoot;

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup_email_ui(mut commands: Commands, email_state: Res<EmailState>) {
    let body_text = email_body_text(&email_state);
    let replies = email_state
        .current_email()
        .map(|e| e.replies)
        .unwrap_or([""; 3]);

    commands
        .spawn((
            EmailUiRoot,
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
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("INBOX"),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::srgb(0.2, 0.6, 1.0)),
            ));

            // Email body
            root.spawn((
                Node {
                    width: Val::Px(640.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.12, 0.18)),
            ))
            .with_children(|panel| {
                panel.spawn((
                    EmailBodyText,
                    Text::new(body_text),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.85, 0.85, 0.9)),
                ));
            });

            // Reply buttons
            for i in 0..3 {
                root.spawn((
                    Node {
                        width: Val::Px(640.0),
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.2, 0.3)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        ReplyText(i),
                        Text::new(format!("[{}]  {}", i + 1, replies[i])),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.75, 0.85, 1.0)),
                    ));
                });
            }

            // "All done" message
            root.spawn((
                DoneText,
                Text::new("Inbox empty. Press Escape to return."),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.4, 0.8, 0.4)),
                Visibility::Hidden,
            ));

            // Hint
            root.spawn((
                Text::new("Press 1 / 2 / 3 to reply  ·  Esc to close"),
                TextFont { font_size: 13.0, ..default() },
                TextColor(Color::srgb(0.4, 0.45, 0.55)),
            ));
        });
}

fn cleanup_email_ui(mut commands: Commands, query: Query<Entity, With<EmailUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

pub fn handle_reply_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut email_state: ResMut<EmailState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if email_state.all_done {
        if keys.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::Office);
        }
        return;
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Office);
        return;
    }

    let chose = keys.just_pressed(KeyCode::Digit1)
        || keys.just_pressed(KeyCode::Digit2)
        || keys.just_pressed(KeyCode::Digit3);

    if chose {
        email_state.advance();
    }
}

fn update_email_display(
    email_state: Res<EmailState>,
    mut body_query: Query<&mut Text, With<EmailBodyText>>,
    mut reply_query: Query<(&ReplyText, &mut Text), Without<EmailBodyText>>,
    mut done_query: Query<&mut Visibility, With<DoneText>>,
) {
    if !email_state.is_changed() { return; }

    let Ok(mut done_vis) = done_query.single_mut() else { return };

    if email_state.all_done {
        *done_vis = Visibility::Visible;
        if let Ok(mut body) = body_query.single_mut() {
            body.0 = String::new();
        }
        for (_, mut t) in &mut reply_query {
            t.0 = String::new();
        }
        return;
    }

    *done_vis = Visibility::Hidden;

    if let Ok(mut body) = body_query.single_mut() {
        body.0 = email_body_text(&email_state);
    }

    if let Some(email) = email_state.current_email() {
        for (ReplyText(i), mut t) in &mut reply_query {
            t.0 = format!("[{}]  {}", i + 1, email.replies[*i]);
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn email_body_text(state: &EmailState) -> String {
    match state.current_email() {
        Some(e) => format!(
            "From:    {}\nSubject: {} ({}/{})\n\n{}",
            e.from,
            e.subject,
            state.current + 1,
            state.total(),
            e.body
        ),
        None => String::new(),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
            .init_resource::<ButtonInput<KeyCode>>();
        app
    }

    #[test]
    fn email_state_starts_at_first_email() {
        let state = EmailState::default();
        assert_eq!(state.current, 0);
        assert!(!state.all_done);
        assert!(state.current_email().is_some());
    }

    #[test]
    fn advance_moves_through_inbox() {
        let mut state = EmailState::default();
        let total = state.total();
        for i in 0..total {
            assert_eq!(state.current, i);
            assert!(!state.all_done);
            state.advance();
        }
        assert!(state.all_done);
        assert!(state.current_email().is_none());
    }

    #[test]
    fn reply_key_advances_email() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .init_resource::<EmailState>()
            .add_systems(
                Update,
                handle_reply_input.run_if(in_state(GameState::EmailMinigame)),
            );

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::EmailMinigame);
        app.update();

        let before = app.world().resource::<EmailState>().current;

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Digit1);
        app.update();

        let after = app.world().resource::<EmailState>().current;
        assert_eq!(after, before + 1, "pressing 1 should advance to next email");
    }

    #[test]
    fn escape_key_returns_to_office() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .init_resource::<EmailState>()
            .add_systems(
                Update,
                handle_reply_input.run_if(in_state(GameState::EmailMinigame)),
            );

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::EmailMinigame);
        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        app.update(); // system runs, schedules transition
        app.update(); // transition applied

        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Office);
    }

    #[test]
    fn all_three_reply_keys_advance_inbox() {
        for key in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3] {
            let mut app = test_app();
            app.init_state::<GameState>()
                .init_resource::<EmailState>()
                .add_systems(
                    Update,
                    handle_reply_input.run_if(in_state(GameState::EmailMinigame)),
                );

            app.world_mut()
                .resource_mut::<NextState<GameState>>()
                .set(GameState::EmailMinigame);
            app.update();

            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            app.update();

            let idx = app.world().resource::<EmailState>().current;
            assert_eq!(idx, 1, "key {key:?} should advance email index");
        }
    }

    #[test]
    fn completing_all_emails_sets_all_done() {
        let mut app = test_app();
        app.init_state::<GameState>()
            .init_resource::<EmailState>()
            .add_systems(
                Update,
                handle_reply_input.run_if(in_state(GameState::EmailMinigame)),
            );

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::EmailMinigame);
        app.update();

        let total = app.world().resource::<EmailState>().total();
        for _ in 0..total {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(KeyCode::Digit1);
            app.update();
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .release(KeyCode::Digit1);
            app.update();
        }

        let done = app.world().resource::<EmailState>().all_done;
        assert!(done, "all_done should be true after replying to every email");
    }
}
