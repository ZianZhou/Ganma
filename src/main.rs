use bevy::prelude::*;
use bevy::window::WindowPlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                title: "Happy Birthday, 干妈!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_text, animate_background))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Background color
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0), // Initial white background
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        BackgroundAnimation {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        },
    ));

    // Create text entity
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Happy Birthday, GanMa! \n".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Add a custom font here
                            font_size: 50.0,
                            color: Color::rgb(1.0, 0.5, 0.0), // Orange color
                        },
                    },
                    TextSection {
                        value: "\nI hope you’re having a wonderful day filled with love and happiness because you truly deserve it.\n\n\
                                I wanted to take a moment to thank you for everything you’ve done for me.\n\
                                You’ve raised me with so much care and patience,\n\
                                and I’m incredibly grateful for all the sacrifices you’ve made along the way.\n\n\
                                Thank you for treating me like your own son.\n\
                                You’ve always been there for me, guiding me and supporting me through every step of my life.\n\
                                I might not say it often, but I really appreciate everything you’ve done,\n\
                                even the little things. They mean more to me than I can express.\n\n\
                                I always think about how lucky I am to have both MaMa and you in my life.\n\
                                The way you’ve both shaped who I am today is something I’ll never take for granted.\n\
                                I want to work hard and become someone you can both be proud of—\n\
                                someone who lives up to the love and trust you’ve given me.\n\n\
                                I miss you all so much, shout out to TuTu.\n\
                                I think about the times we’ve spent together and how much I cherish those memories.\n\
                                I really hope we can be reunited soon so I can see everyone again and make more memories together.\n\n\
                                Thank you for everything, GanMa. I love you all so much.\n\n\
                                With love,\nZian".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"), // Add a custom font here
                            font_size: 25.0,
                            color: Color::rgb(0.0, 0.0, 0.0), // Black color
                        },
                    },
                ],
                alignment: TextAlignment::Center,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0), // Move text in front of background
            ..Default::default()
        },
        TextAnimation { timer: Timer::from_seconds(0.5, TimerMode::Repeating) },
    ));
}

#[derive(Component)]
struct TextAnimation {
    timer: Timer,
}

#[derive(Component)]
struct BackgroundAnimation {
    timer: Timer,
}

fn animate_text(time: Res<Time>, mut query: Query<(&mut Text, &mut TextAnimation)>) {
    for (mut text, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let mut rng = rand::thread_rng();
            for section in text.sections.iter_mut() {
                section.style.color =
                    Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
            }
        }
    }
}

fn animate_background(time: Res<Time>, mut query: Query<(&mut Sprite, &mut BackgroundAnimation)>) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            let mut rng = rand::thread_rng();
            sprite.color = Color::rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        }
    }
}
