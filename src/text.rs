use crate::context::Context;

use sdl2::{rect::Rect, render::TextureQuery};

#[derive(Clone, Debug)]
pub struct Rope {
    length: usize,
    left_rope: Option<Box<Rope>>,
    right_rope: Option<Box<Rope>>,
    pub held_string: Option<String>
}

impl Rope {
    pub fn len(&self) -> usize {
        if self.right_rope.is_some() {
            return self.length + self.right_rope.clone().unwrap().len()
        }
        self.length
    }

    pub fn substr(&self, start: usize, end: usize) -> String {
        if start > end {
            panic!("end is before start");
        }
        if self.held_string.is_some() {
            self.held_string.clone().unwrap()[start..end].to_owned()
        } else if start < self.length && end < self.length {
            self.left_rope.clone().unwrap().substr(start, end)
        } else if start >= self.length && end >= self.length {
            self.right_rope.clone().unwrap().substr(start - self.length, end - self.length)
        } else {
            let mut string: String = self.left_rope.clone().unwrap().substr(start, self.length);
            string.push_str(&self.right_rope.clone().unwrap().substr(0, end - self.length));
            string
        }
    }

    pub fn append(&mut self, next_rope: Rope) {
        self.left_rope = Some(Box::new(self.clone()));
        self.length = self.len();
        self.right_rope = Some(Box::new(next_rope));
        self.held_string = None;
    }

    pub fn from_string(string: String) -> Rope {
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
                        to_return.append(ropes[1].clone());
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
        let mut current_node = self;
        while current_node.held_string.is_none() {
            if index >= current_node.length {
                index -= current_node.length;
                current_node = current_node
                    .right_rope
                    .as_mut()
                    .unwrap();
            } else {
                current_node = current_node
                    .left_rope
                    .as_mut()
                    .unwrap();
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
        current_node.length += 1;
    }

    pub fn insert_string_at(&mut self, to_insert: &str, mut index: usize) {
        let mut current_node = self;
        while current_node.held_string.is_none() {
            if index >= current_node.length {
                index -= current_node.length;
                current_node = current_node
                    .right_rope
                    .as_mut()
                    .unwrap();
            } else {
                current_node = current_node
                    .left_rope
                    .as_mut()
                    .unwrap();
            }
        }

        let current_held_string = current_node.held_string.clone().unwrap();
        let (left_str, right_str) = current_held_string.split_at(index);

        current_node.held_string = None;
        current_node.length = left_str.len();
        current_node.left_rope = Some(
            Box::new(
                Rope::from_string(left_str.to_string())
            )
        );

        let mut new_rope: Rope = Rope::from_string(to_insert.to_string());
        new_rope.append(Rope::from_string(right_str.to_string()));
        
        current_node.right_rope = Some(
            Box::new(new_rope)
        );
    }

    pub fn rebalance(&mut self) {
        todo!("havent done rebalancing yet");
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

pub struct Cursor {
    position: usize,
    row: usize,
    column: usize
}

impl Cursor {

}