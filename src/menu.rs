use crate::*;

use bevy::prelude::*;


#[derive(Component)]
pub struct MenuButton {}

#[derive(Resource)]
pub struct MenuData {
    button_entity: Entity,
}

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = asset_server.load("UISign.png");
    
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }
        )
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(160.0),
                        height: Val::Px(32.0),
                        //border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: BackgroundColor(Color::RED),
                    ..default()
                }, MenuButton{}))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage::new(image.clone()),
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(32.0),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Button",
                            text_style
                        ));
                    });
                });
        }).id();
    commands.insert_resource(MenuData { button_entity });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color, mut border_color, mut style) in &mut interaction_query {
        println!("{:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::BLUE);
                border_color.0 = Color::RED;
                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::YELLOW);
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = BackgroundColor(Color::GREEN);
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn menu_cleanup(
    mut commands: Commands,
    menu_data: Res<MenuData>
) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}