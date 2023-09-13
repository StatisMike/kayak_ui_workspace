use bevy::{prelude::{Plugin, Commands, Query, Entity, With, Camera2d, Color, App, warn}};
use kayak_ui::{CameraUIKayak, widgets::{KayakWidgetsContextPlugin, KayakAppBundle, BackgroundBundle, TextWidgetBundle, TextProps, KayakWidgets}, prelude::{KChildren, KayakRootContext, rsx, KStyle, KPositionType, StyleProp, Units, Edge, Alignment, EventDispatcher, KayakContextPlugin}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(KayakWidgets)
        .add_plugin(KayakContextPlugin)
        .add_startup_system(startup);
        
    }
}

pub fn startup(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>
  ) {
    if let Ok(camera_entity) = camera_query.get_single() {
      commands.entity(camera_entity).insert(CameraUIKayak);
    
      let mut widget_context = KayakRootContext::new(camera_entity);
      widget_context.add_plugin(KayakWidgetsContextPlugin);
  
      let parent_id = None;
      rsx! {
        <KayakAppBundle>
            <BackgroundBundle styles = {
                KStyle {
                position_type: KPositionType::SelfDirected.into(),
                width: StyleProp::Value(Units::Percentage(60.)),
                height: StyleProp::Value(Units::Percentage(60.)),
                top: Units::Percentage(20.).into(),
                left: Units::Percentage(20.).into(),
                background_color: Color::BLACK.into(),
                border_color: Color::PURPLE.into(),
                border: Edge::all(5.).into(),
                ..Default::default()
                }}>
                <TextWidgetBundle
                text={TextProps {
                    content: "This is from library crate".into(),
                    size: 20.0,
                    alignment: Alignment::Middle,
                    ..Default::default()
                }}
                />
            </BackgroundBundle>
        </KayakAppBundle>
      };
  
      commands.spawn((widget_context, EventDispatcher::default()));
  
    } else {
      warn!("Couldn't find Camera2d!")
    }
}