use bevy::prelude::*;

#[derive(Component)]
pub struct StatMenuParentNode {}

pub enum MenuStatistic {
    MoveGenTime,
}

impl MenuStatistic {
    pub const DEFAULT_COLOR: Color = Color::rgb(0.53, 0.49, 0.48);
    pub const MOVE_GEN_TIME_COLOR: Color = Color::rgb(0.9, 0.4, 0.1);
}

#[derive(Component)]
pub struct StatMenuText {
    pub stat: MenuStatistic,
}

#[derive(Resource)]
pub struct CalcStatistics {
    pub move_gen_time: f32,
}

impl Default for CalcStatistics {
    fn default() -> Self {
        CalcStatistics {
            move_gen_time: 0.0,
        }
    }
}


pub fn spawn_calc_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Percent(2.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }, StatMenuParentNode {}))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Move gen time: N/A",
                TextStyle {
                    font: asset_server.load("ui/font/Monaco.ttf"),
                    font_size: 15.0,
                    color: MenuStatistic::MOVE_GEN_TIME_COLOR,
                }
            ), StatMenuText { stat: MenuStatistic::MoveGenTime }));
        });
}

pub fn update_menu_stats(
    calc_stats: Res<CalcStatistics>,
    mut menu_text_query: Query<(&mut Text, &StatMenuText)>,
) {
    for (mut text, stat) in menu_text_query.iter_mut() {
        text.sections[0].value = match stat.stat {
            MenuStatistic::MoveGenTime => { format!("Move gen time: {} ms", calc_stats.move_gen_time) }
        }
    }
}