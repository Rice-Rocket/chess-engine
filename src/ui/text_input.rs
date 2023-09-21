use bevy::prelude::*;


pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_text_input,
                update_text_input_interaction
            ));
    }
}

#[derive(Component)]
pub struct TextInput {
    pub value: String,
    pub inactive: bool,
    prefix: String,
    suffix: String,
    display_value: String,
    active: bool,
    only_numbers: bool,
}

impl TextInput {
    pub fn new(prefix: &str, default_value: &str, suffix: &str, only_numbers: bool) -> Self {
        Self {
            value: default_value.to_string(),
            inactive: false,
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
            display_value: default_value.to_string(),
            active: false,
            only_numbers,
        }
    }
}

pub fn update_text_input_interaction(
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<TextInput>, With<Button>)>,
    mut text_input_query: Query<(&mut TextInput, Entity), With<Button>>,
) {
    let mut pressed = None;
    for (interaction, entity) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => { pressed = Some(entity); },
            Interaction::Hovered | Interaction::None => (),
        }
    }
    if let Some(text_input_entity) = pressed {
        for (mut text_input, entity) in text_input_query.iter_mut() {
            if text_input.inactive {
                text_input.active = false;
                continue;
            }
            if entity == text_input_entity || text_input.active {
                text_input.active = !text_input.active; 
            }
        }
    }
}


fn update_text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keyboard: Res<Input<KeyCode>>,
    mut text_input_query: Query<(&mut TextInput, &Children), (With<TextInput>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    time: Res<Time>,
) {
    let mut possible_char = None;
    for char_ev in char_evr.iter() {
        possible_char = Some(char_ev);
    }

    for (mut text_input, children) in text_input_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        
        if keyboard.just_pressed(KeyCode::Return) && text_input.active {
            text_input.value = text_input.display_value.clone();
            text_input.active = false;
        }
        if keyboard.just_pressed(KeyCode::Back) && text_input.active {
            text_input.display_value.pop();
        }
        if let Some(char) = possible_char {
            if !char.char.is_control() && ((char.char.is_digit(10) && text_input.only_numbers) || !text_input.only_numbers) && text_input.active {
                text_input.display_value.push(char.char);
            }
        }
        let cursor = if text_input.active && (time.elapsed_seconds() % 1.5) > 0.75 { "|" } else { " " };
        text.sections[0].value = text_input.prefix.clone() + &text_input.display_value.clone() + cursor + &text_input.suffix.clone();
    }
}