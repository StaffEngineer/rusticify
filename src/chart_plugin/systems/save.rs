use base64::{engine::general_purpose, Engine};
use bevy::prelude::*;

use bevy_pkv::PkvStore;
use image::*;

use serde_json::json;
use std::{collections::HashMap, io::Cursor};

use super::ui_helpers::{EditableText, VeloNode};
use super::{load_doc_to_memory, VeloNodeContainer};
use crate::canvas::arrow::components::ArrowMeta;
use crate::components::Doc;
use crate::resources::SaveDocRequest;
use crate::resources::{AppState, SaveTabRequest};
use crate::utils::ReflectableUuid;
use crate::{chart_plugin::ui_helpers::style_to_pos, JsonNode, JsonNodeText, MAX_CHECKPOINTS};

pub fn should_save_doc(request: Option<Res<SaveDocRequest>>) -> bool {
    request.is_some()
}

pub fn should_save_tab(request: Option<Res<SaveTabRequest>>) -> bool {
    request.is_some()
}

pub fn remove_save_doc_request(world: &mut World) {
    world.remove_resource::<SaveDocRequest>().unwrap();
}

pub fn remove_save_tab_request(world: &mut World) {
    world.remove_resource::<SaveTabRequest>().unwrap();
}

pub fn save_doc(
    request: Res<SaveDocRequest>,
    mut app_state: ResMut<AppState>,
    mut pkv: ResMut<PkvStore>,
    mut commands: Commands,
) {
    let doc_id = request.doc_id;

    load_doc_to_memory(doc_id, &mut app_state, &mut pkv);

    for tab in app_state.docs.get_mut(&doc_id).unwrap().tabs.iter() {
        if tab.is_active {
            commands.insert_resource(SaveTabRequest {
                doc_id,
                tab_id: tab.id,
            });
        }
    }

    if let Ok(mut docs) = pkv.get::<HashMap<ReflectableUuid, Doc>>("docs") {
        docs.insert(doc_id, app_state.docs.get(&doc_id).unwrap().clone());
        pkv.set("docs", &docs).unwrap();
    } else {
        let mut docs = HashMap::new();
        docs.insert(doc_id, app_state.docs.get(&doc_id).unwrap().clone());
        pkv.set("docs", &docs).unwrap();
    }
    if let Ok(mut tags) = pkv.get::<HashMap<ReflectableUuid, Vec<String>>>("tags") {
        let doc = app_state.docs.get(&doc_id).unwrap();
        let tags = tags.get_mut(&doc_id).unwrap();
        tags.append(&mut doc.tags.clone());
        pkv.set("tags", &tags).unwrap();
    } else {
        let doc = app_state.docs.get(&doc_id).unwrap();
        pkv.set("tags", &doc.tags).unwrap();
    }
    if let Ok(mut names) = pkv.get::<HashMap<ReflectableUuid, String>>("names") {
        let doc = app_state.docs.get(&doc_id).unwrap();
        names.insert(doc.id, doc.name.clone());
        pkv.set("names", &names).unwrap();
    } else {
        let doc = app_state.docs.get(&doc_id).unwrap();
        let mut names = HashMap::new();
        names.insert(doc.id, doc.name.clone());
        pkv.set("names", &names).unwrap();
    }
    pkv.set("last_saved", &doc_id).unwrap();

    if let Some(path) = request.path.clone() {
        let current_doc = app_state.docs.get(&doc_id).unwrap().clone();
        std::fs::write(path, serde_json::to_string_pretty(&current_doc).unwrap())
            .expect("Error saving current document to file")
    }
}

pub fn save_tab(
    images: Res<Assets<Image>>,
    rec_container_query: Query<&Style, With<VeloNodeContainer>>,
    rec_query: Query<
        (
            &VeloNode,
            &UiImage,
            &BackgroundColor,
            &Children,
            &ZIndex,
            &Parent,
            &Style,
        ),
        (With<VeloNode>, Without<VeloNodeContainer>),
    >,
    arrows: Query<(&ArrowMeta, &Visibility), With<ArrowMeta>>,
    request: Res<SaveTabRequest>,
    mut app_state: ResMut<AppState>,
    text_query: Query<&mut Text, With<EditableText>>,
) {
    let mut json = json!({
        "images": {},
        "nodes": [],
        "arrows": [],
    });
    let json_images = json["images"].as_object_mut().unwrap();
    for (rect, image, _, _, _, _, _) in rec_query.iter() {
        if let Some(image) = images.get(&image.texture) {
            if let Ok(img) = image.clone().try_into_dynamic() {
                let mut image_data: Vec<u8> = Vec::new();
                img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
                    .unwrap();
                let res_base64 = general_purpose::STANDARD.encode(image_data);
                json_images.insert(rect.id.0.to_string(), json!(res_base64));
            }
        }
    }

    let json_nodes = json["nodes"].as_array_mut().unwrap();
    for (rect, _, bg_color, children, z_index, parent, test_pos_style) in rec_query.iter() {
        let text = text_query.get(children[children.len() - 1]).unwrap();
        let mut str = "".to_string();
        let mut text_copy = text.clone();
        text_copy.sections.pop();
        for section in text_copy.sections.iter() {
            str = format!("{}{}", str, section.value.clone());
        }
        let style: &Style = rec_container_query.get(parent.get()).unwrap();
        let left = style.position.left;
        let bottom = style.position.bottom;
        let size = style.size;
        let bg_color = bg_color.0;
        let z_index = match *z_index {
            ZIndex::Local(v) => v,
            _ => -1,
        };
        json_nodes.push(json!(JsonNode {
            node_type: crate::NodeType::Rect,
            id: rect.id.0,
            left,
            bottom,
            width: size.width,
            height: size.height,
            bg_color,
            text: JsonNodeText {
                text: str,
                pos: style_to_pos((test_pos_style.justify_content, test_pos_style.align_items)),
            },
            z_index,
            tags: vec![],
        }));
    }

    let json_arrows = json["arrows"].as_array_mut().unwrap();
    for (arrow_meta, visibility) in arrows.iter() {
        if visibility != Visibility::Hidden {
            json_arrows.push(json!(arrow_meta));
        }
    }

    let doc_id = request.doc_id;

    for tab in &mut app_state.docs.get_mut(&doc_id).unwrap().tabs {
        if request.tab_id == tab.id {
            if (tab.checkpoints.len() as i32) > MAX_CHECKPOINTS {
                tab.checkpoints.pop_front();
            }
            if let Some(last) = tab.checkpoints.back() {
                if last == &json.to_string() {
                    break;
                }
            }
            tab.checkpoints.push_back(json.to_string());
            break;
        }
    }
}
