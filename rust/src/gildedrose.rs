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

    fn upgrade_quality(&mut self, quantity: i32) {
        for _ in 0..quantity {
            if self.quality < 50 {
                self.quality += 1;
            }
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

const ETC_CONCERT_TICKET: &str = "Backstage passes to a TAFKAL80ETC concert";
const AGED_BRIE: &str = "Aged Brie";
const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            match item.name.as_str() {
                ETC_CONCERT_TICKET => {
                    item.sell_in -= 1;

                    if item.sell_in < 0 {
                        item.quality = 0;
                        continue;
                    }

                    item.upgrade_quality(match item.sell_in {
                        0..=4 => 3,
                        5..=9 => 2,
                        _ => 1,
                    });

                    continue;
                }

                AGED_BRIE => {
                    item.sell_in -= 1;
                    item.upgrade_quality(if item.sell_in < 0 { 2 } else { 1 });

                    continue;
                }

                SULFURAS => {
                    item.upgrade_quality(1);

                    continue;
                }

                _ => {}
            };

            if item.name != SULFURAS {
                item.downgrade_quality();
            } else {
                item.upgrade_quality(1);
            }

            if item.name != SULFURAS {
                item.sell_in -= 1;
            }

            if item.sell_in < 0 {
                if item.name != SULFURAS {
                    item.downgrade_quality();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, AGED_BRIE, ETC_CONCERT_TICKET, SULFURAS};

    fn test_update_sell_in(item_in: Item, want: i32) {
        let items = vec![item_in];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(want, rose.items[0].sell_in);
    }

    fn test_update_quality(item_in: Item, want: i32) {
        let items = vec![item_in];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(want, rose.items[0].quality);
    }

    #[test]
    pub fn test_reduce_sell_in_except_sulfuras() {
        test_update_sell_in(Item::new("foo", 10, 0), 9);
        test_update_sell_in(Item::new(AGED_BRIE, 10, 0), 9);
        test_update_sell_in(Item::new(ETC_CONCERT_TICKET, 10, 0), 9);
        test_update_sell_in(Item::new(SULFURAS, 10, 0), 10);
    }

    #[test]
    pub fn test_quality_cannot_upgrade_to_more_than_50() {
        test_update_quality(Item::new(AGED_BRIE, 10, 50), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 10, 50), 50);
        test_update_quality(Item::new(SULFURAS, 10, 50), 50);
    }

    #[test]
    pub fn test_quality_can_upgrade_when_under_50() {
        test_update_quality(Item::new(AGED_BRIE, 10, 49), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 10, 49), 50);
        test_update_quality(Item::new(SULFURAS, 10, 49), 50);
    }

    #[test]
    pub fn test_quality_stays_the_same_over_50() {
        test_update_quality(Item::new(AGED_BRIE, 10, 123456), 123456);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 10, 123456), 123456);
        test_update_quality(Item::new(SULFURAS, 10, 123456), 123456);
    }

    #[test]
    pub fn test_etc_quality_sell_in_11() {
        // Quality never goes over 50
        // Quality does +1 if quality >= 11
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 11, 50), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 11, 49), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 11, 48), 49);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 11, 30), 31);
    }

    #[test]
    pub fn test_etc_quality_sell_in_6_to_10() {
        // Quality never goes over 50
        // Quality does +3 if 6 <= quality <= 10
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 6, 50), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 6, 49), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 6, 48), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 10, 47), 49);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 10, 30), 32);
    }

    #[test]
    pub fn test_etc_quality_sell_in_1_to_5() {
        // Quality never goes over 50
        // Quality does +3 if 1 <= quality <= 5
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 1, 50), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 1, 49), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 1, 48), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 5, 47), 50);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 5, 46), 49);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 5, 30), 33);
    }

    #[test]
    pub fn test_etc_quality_sell_in_0_or_less() {
        // Quality always go to 0 if sell_in is 0 or negative
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 0, 50), 0);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, 0, 49), 0);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, -1, 48), 0);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, -5, 47), 0);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, -10, 46), 0);
        test_update_quality(Item::new(ETC_CONCERT_TICKET, -50, 30), 0);
    }

    #[test]
    pub fn test_aged_brie_upgrade_once_when_sell_in_positive() {
        // Quality always upgrades by 1 (limit: 50) with a positive sell_in
        test_update_quality(Item::new(AGED_BRIE, 10, 50), 50);
        test_update_quality(Item::new(AGED_BRIE, 10, 49), 50);
        test_update_quality(Item::new(AGED_BRIE, 10, 48), 49);
        test_update_quality(Item::new(AGED_BRIE, 10, 30), 31);
    }

    #[test]
    pub fn test_aged_brie_upgrade_twice_when_sell_in_not_positive() {
        // Quality+2 when sell_in <= 0
        test_update_quality(Item::new(AGED_BRIE, 0, 5), 7);
        test_update_quality(Item::new(AGED_BRIE, 0, 4), 6);
        test_update_quality(Item::new(AGED_BRIE, -1, 48), 50);
        test_update_quality(Item::new(AGED_BRIE, -10, 47), 49);
        test_update_quality(Item::new(AGED_BRIE, -50, 30), 32);
    }

    #[test]
    pub fn test_suflfuras_always_upgrade_quality() {
        // Quality+1 no matter what
        test_update_quality(Item::new(SULFURAS, 150, 46), 47);
        test_update_quality(Item::new(SULFURAS, 10, 5), 6);
        test_update_quality(Item::new(SULFURAS, 0, 4), 5);
        test_update_quality(Item::new(SULFURAS, -1, 48), 49);
        test_update_quality(Item::new(SULFURAS, -10, 47), 48);
    }
}
