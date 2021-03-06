use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Seeds,
    RawPotato,
    RawPotatoBlight,
    CookedPotato,
    CanOfBeans,
    Gun,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Self::Seeds => "Potato seeds",
            Self::RawPotato | Self::RawPotatoBlight => "Raw potato",
            Self::CookedPotato => "Cooked potato",
            Self::CanOfBeans => "Can of beans",
            Self::Gun => "A gun",
        }
    }

    pub fn is_cookable(&self) -> bool {
        matches!(self, Self::RawPotato | Self::RawPotatoBlight)
    }

    pub fn is_edible(&self) -> bool {
        matches!(self, Self::CookedPotato | Self::CanOfBeans)
    }
}

pub struct Inventory {
    items: Vec<(Item, usize)>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![(Item::CanOfBeans, 5)],
        }
    }
}

impl Inventory {
    pub fn add(&mut self, item: Item, count: usize) {
        for (i, n) in self.items.iter_mut() {
            if i == &item {
                *n += count;
                return;
            }
        }
        self.items.push((item, count));
    }

    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn items(&self) -> impl Iterator<Item = (Item, usize)> + '_ {
        // we need to compress blight and non-blight potatoes
        let blight_potato_count = self.count(Item::RawPotatoBlight);
        let raw_potato_count = self.count(Item::RawPotato);
        self.items
            .iter()
            .cloned()
            .filter_map(move |(item, count)| match item {
                Item::RawPotato => Some((item, count + blight_potato_count)),
                // if we have no raw potatoes, we return the blight potatoes as raw instead
                Item::RawPotatoBlight if raw_potato_count == 0 => {
                    Some((Item::RawPotato, blight_potato_count))
                }
                Item::RawPotatoBlight => None,
                _ => Some((item, count)),
            })
    }

    pub fn count(&self, item: Item) -> usize {
        self.items
            .iter()
            .filter_map(|(i, count)| if i == &item { Some(*count) } else { None })
            .next()
            .unwrap_or_default()
    }

    pub fn cook_all(&mut self) {
        if let Some(raw_potato_index) = self.items.iter().position(|(i, _)| i == &Item::RawPotato) {
            let potatoes = self.items[raw_potato_index].1;
            self.items.remove(raw_potato_index);
            if let Some(cooked_potato_index) = self
                .items
                .iter()
                .position(|(i, _)| i == &Item::CookedPotato)
            {
                self.items[cooked_potato_index].1 += potatoes;
            } else {
                self.items.push((Item::CookedPotato, potatoes));
            }
        }
        self.remove_all(Item::RawPotatoBlight);
    }

    pub fn has_edibles(&self) -> bool {
        self.items.iter().any(|(i, _)| i.is_edible())
    }

    pub fn has_cookables(&self) -> bool {
        self.items.iter().any(|(i, _)| i.is_cookable())
    }

    pub fn remove_all(&mut self, item: Item) {
        if let Some(idx) = self.items.iter().position(|(i, _)| i == &item) {
            self.items.remove(idx);
        }
    }

    pub fn remove(&mut self, item: Item, count: usize) {
        if let Some(idx) = self.items.iter().position(|(i, _)| i == &item) {
            if self.items[idx].1 <= count {
                self.items.remove(idx);
            } else {
                self.items[idx].1 -= count;
            }
        }
    }

    pub fn try_remove(&mut self, item: Item, count: usize) -> bool {
        if let Some(idx) = self.items.iter().position(|(i, _)| i == &item) {
            match self.items[idx].1.cmp(&count) {
                Ordering::Less => false,
                Ordering::Equal => {
                    self.items.remove(idx);
                    true
                }
                Ordering::Greater => {
                    self.items[idx].1 -= count;
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn remove_edible(&mut self) -> bool {
        for idx in 0..self.items.len() {
            if matches!(self.items[idx].0, Item::CookedPotato | Item::CanOfBeans) {
                self.items[idx].1 -= 1;
                if self.items[idx].1 == 0 {
                    self.items.remove(idx);
                }
                return true;
            }
        }
        false
    }
}
