use std::fmt::{self, Debug, Display};

#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }

    fn upgrade_quality(&mut self) {
        if self.quality < 50 {
            self.quality += 1;
        }
    }

    fn downgrade_quality(&mut self) {
        if self.quality > 0 {
            self.quality -= 1;
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name != "Aged Brie"
                && item.name != "Backstage passes to a TAFKAL80ETC concert"
                && item.name != "Sulfuras, Hand of Ragnaros"
            {
                item.downgrade_quality();
            } else {
                if item.quality < 50 {
                    item.upgrade_quality();

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            item.upgrade_quality();
                        }

                        if item.sell_in < 6 {
                            item.upgrade_quality();
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in -= 1;
            }

            if item.sell_in < 0 {
                if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    item.quality = 0;
                } else if item.name == "Aged Brie" {
                    item.upgrade_quality();
                } else if item.name != "Sulfuras, Hand of Ragnaros" {
                    item.downgrade_quality();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }

    #[test]
    pub fn test_etc_50_5() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            50,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(
            vec![Item::new(
                "Backstage passes to a TAFKAL80ETC concert",
                4,
                50
            )],
            rose.items
        );
    }

    #[test]
    pub fn test_etc_49_5() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            5,
            49,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(
            vec![Item::new(
                "Backstage passes to a TAFKAL80ETC concert",
                4,
                50
            )],
            rose.items
        );
    }

    #[test]
    pub fn test_etc_40_9() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            9,
            40,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(
            vec![Item::new(
                "Backstage passes to a TAFKAL80ETC concert",
                8,
                42
            )],
            rose.items
        );
    }

    #[test]
    pub fn test_etc_40_4() {
        let items = vec![Item::new(
            "Backstage passes to a TAFKAL80ETC concert",
            4,
            40,
        )];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(
            vec![Item::new(
                "Backstage passes to a TAFKAL80ETC concert",
                3,
                43
            )],
            rose.items
        );
    }
}
