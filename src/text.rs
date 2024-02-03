use crate::context::Context;

use sdl2::{rect::Rect, render::TextureQuery};

#[derive(Clone, Debug)]
pub struct Rope<'a> {
    length: usize,
    left_rope: Option<&'a Rope<'a>>,
    right_rope: Option<&'a Rope<'a>>,
    pub held_string: Option<String>
}

impl Rope<'_> {
    pub fn len(&self) -> usize {
        if self.right_rope.is_some() {
            return self.length + self.right_rope.clone().unwrap().len()
        }
        self.length
    }

    pub fn append(&mut self, next_rope: &Rope) {
        self.length = self.len();
        self.left_rope = Some(self);
        self.right_rope = Some(next_rope);
        self.held_string = None;
    }

    pub fn from_string(string: String) -> Rope<'static> {
        let mut leaves: Vec<Rope> = string
            .split(" ")
            .map(|e| Rope {
                length: e.len() + 1,
                left_rope: None,
                right_rope: None,
                held_string: Some(" ".to_string() + e)
            })
            .collect();
        
        let mut new_first_leaf = leaves[0].held_string.clone().unwrap();
        new_first_leaf.remove(0);

        leaves[0].held_string = Some(new_first_leaf);
        leaves[0].length -= 1;

        while leaves.len() != 1 {
            leaves = leaves
                .chunks(2)
                .map(|ropes: &[Rope]| {
                    let mut to_return = ropes[0].clone();
                    if ropes.len() == 2 {
                        to_return.append(&ropes[1].clone());
                    }
                    to_return
                })
                .collect();
        };

        leaves.first().unwrap().to_owned()
    }

    pub fn collect_string(&self) -> String {
        if self.held_string.is_some() {
            return self.held_string.clone().unwrap();
        }
        let mut ret = self.left_rope
            .as_ref()
            .unwrap()
            .collect_string();
        ret.push_str(self.right_rope.as_ref().unwrap().collect_string().as_str());
        ret
    }

    pub fn insert_char_at(&mut self, to_insert: char, mut index: usize) {
        let mut current_node: &Rope<'_> = self;
        let mut next_node: &Rope;
        while current_node.held_string.is_none() {
            if index >= current_node.length {
                index -= current_node.length;
                next_node = current_node.right_rope.unwrap();
                current_node = next_node;
            } else {
                next_node = current_node.left_rope.unwrap();
                current_node = next_node;
            }
        }

        current_node.held_string = {
            let mut new_str = current_node
                .held_string
                .clone()
                .unwrap();
            new_str.insert(index, to_insert);
            Some(new_str)
        };
    }
}

pub fn render_text_from_rope(rope: &Rope, context: &mut Context) {
    let text: String = rope.collect_string();
    let texture_creator = context.canvas.texture_creator();

    let (screen_width, _) = context.canvas.window().size();

    let text_texture = 
        context
        .ttf_context
        .load_font("./JetBrainsMono-VariableFont_wght.ttf", context.settings.font_size)
        .unwrap()
        .render(&text)
        .blended_wrapped(context.settings.text_color, screen_width)
        .unwrap()
        .as_texture(&texture_creator)
        .unwrap();
        
    let TextureQuery {width: text_width, height: text_height, ..} = text_texture.query();    
    let text_rect = Rect::new(0, 0, text_width, text_height);
    let _ = context.canvas.copy(&text_texture, None, text_rect).unwrap();
}