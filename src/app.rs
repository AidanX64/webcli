use crate::catalog::{self, BRANDS, CATEGORIES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Brands,
    Categories,
    Products,
    ProductDetail,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub selected_brand: usize,
    pub selected_category: usize,
    pub selected_product: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Brands,
            selected_brand: 0,
            selected_category: 0,
            selected_product: 0,
        }
    }
}

impl App {
    pub fn next(&mut self) {
        match self.screen {
            Screen::Brands => {
                self.selected_brand = (self.selected_brand + 1) % BRANDS.len();
            }
            Screen::Categories => {
                self.selected_category = (self.selected_category + 1) % CATEGORIES.len();
            }
            Screen::Products => {
                let count = self.current_products().len();
                if count > 0 {
                    self.selected_product = (self.selected_product + 1) % count;
                }
            }
            Screen::ProductDetail => {}
        }
    }

    pub fn previous(&mut self) {
        match self.screen {
            Screen::Brands => {
                self.selected_brand = (self.selected_brand + BRANDS.len() - 1) % BRANDS.len();
            }
            Screen::Categories => {
                self.selected_category =
                    (self.selected_category + CATEGORIES.len() - 1) % CATEGORIES.len();
            }
            Screen::Products => {
                let count = self.current_products().len();
                if count > 0 {
                    self.selected_product = (self.selected_product + count - 1) % count;
                }
            }
            Screen::ProductDetail => {}
        }
    }

    pub fn select(&mut self) {
        match self.screen {
            Screen::Brands => {
                self.selected_category = 0;
                self.selected_product = 0;
                self.screen = Screen::Categories;
            }
            Screen::Categories => {
                self.selected_product = 0;
                self.screen = Screen::Products;
            }
            Screen::Products => {
                if self.current_product().is_some() {
                    self.screen = Screen::ProductDetail;
                }
            }
            Screen::ProductDetail => {}
        }
    }

    pub fn back(&mut self) {
        match self.screen {
            Screen::Brands => {}
            Screen::Categories => self.screen = Screen::Brands,
            Screen::Products => self.screen = Screen::Categories,
            Screen::ProductDetail => self.screen = Screen::Products,
        }
    }

    pub fn current_brand(&self) -> &'static catalog::Brand {
        catalog::brand(self.selected_brand)
    }

    pub fn current_category(&self) -> &'static catalog::Category {
        catalog::category(self.selected_category)
    }

    pub fn current_products(&self) -> Vec<&'static catalog::Product> {
        catalog::products_for(self.current_brand().id, self.current_category().id)
    }

    pub fn current_product(&self) -> Option<&'static catalog::Product> {
        self.current_products().get(self.selected_product).copied()
    }
}
