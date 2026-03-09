use crate::{
    catalog::{self, BRANDS, CATEGORIES, Product},
    wishlist::{Wishlist, WishlistItem},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Brands,
    Categories,
    Products,
    ProductDetail,
    Wishlist,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WishlistField {
    Name,
    Brand,
    Category,
    Price,
    Fit,
    Weight,
    Sizes,
    InStock,
}

impl WishlistField {
    pub const ALL: [Self; 8] = [
        Self::Name,
        Self::Brand,
        Self::Category,
        Self::Price,
        Self::Fit,
        Self::Weight,
        Self::Sizes,
        Self::InStock,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Brand => "Brand",
            Self::Category => "Category",
            Self::Price => "Price",
            Self::Fit => "Fit",
            Self::Weight => "Weight",
            Self::Sizes => "Sizes",
            Self::InStock => "In stock",
        }
    }

    fn parse_value(self, value: &str, draft: &mut WishlistDraft) {
        match self {
            Self::Name => draft.name = value.trim().to_string(),
            Self::Brand => draft.brand = value.trim().to_string(),
            Self::Category => draft.category = value.trim().to_string(),
            Self::Price => draft.price = value.trim().to_string(),
            Self::Fit => draft.fit = value.trim().to_string(),
            Self::Weight => draft.weight = value.trim().to_string(),
            Self::Sizes => {
                draft.sizes = value
                    .split(',')
                    .map(|size| size.trim())
                    .filter(|size| !size.is_empty())
                    .map(ToOwned::to_owned)
                    .collect();
            }
            Self::InStock => {
                draft.in_stock = matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "y" | "yes" | "true" | "1"
                );
            }
        }
    }

    fn value_from(self, draft: &WishlistDraft) -> String {
        match self {
            Self::Name => draft.name.clone(),
            Self::Brand => draft.brand.clone(),
            Self::Category => draft.category.clone(),
            Self::Price => draft.price.clone(),
            Self::Fit => draft.fit.clone(),
            Self::Weight => draft.weight.clone(),
            Self::Sizes => draft.sizes.join(", "),
            Self::InStock => {
                if draft.in_stock {
                    "yes".to_string()
                } else {
                    "no".to_string()
                }
            }
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Sizes => "Comma-separated sizes, for example: S, M, L",
            Self::InStock => "Type yes/no, true/false, or y/n",
            _ => "Enter a value and press Enter to continue",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct WishlistDraft {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: String,
    pub fit: String,
    pub weight: String,
    pub sizes: Vec<String>,
    pub in_stock: bool,
}

impl WishlistDraft {
    fn into_item(self) -> WishlistItem {
        WishlistItem {
            name: self.name,
            brand: self.brand,
            category: self.category,
            price: self.price,
            fit: self.fit,
            weight: self.weight,
            sizes: self.sizes,
            in_stock: self.in_stock,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WishlistFormState {
    pub field_index: usize,
    pub input: String,
    pub draft: WishlistDraft,
}

impl WishlistFormState {
    fn new() -> Self {
        Self {
            field_index: 0,
            input: String::new(),
            draft: WishlistDraft::default(),
        }
    }

    pub fn field(&self) -> WishlistField {
        WishlistField::ALL[self.field_index]
    }
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    previous_screen: Screen,
    pub selected_brand: usize,
    pub selected_category: usize,
    pub selected_product: usize,
    pub selected_wishlist_item: usize,
    pub wishlist: Wishlist,
    pub wishlist_form: Option<WishlistFormState>,
    pub status_message: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let (wishlist, status_message) = match Wishlist::load() {
            Ok(wishlist) => (wishlist, None),
            Err(err) => (
                Wishlist::default(),
                Some(format!("Wishlist load failed: {err}")),
            ),
        };

        Self {
            screen: Screen::Brands,
            previous_screen: Screen::Brands,
            selected_brand: 0,
            selected_category: 0,
            selected_product: 0,
            selected_wishlist_item: 0,
            wishlist,
            wishlist_form: None,
            status_message,
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
            Screen::Wishlist => {
                let count = self.wishlist.len();
                if count > 0 {
                    self.selected_wishlist_item = (self.selected_wishlist_item + 1) % count;
                }
            }
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
            Screen::Wishlist => {
                let count = self.wishlist.len();
                if count > 0 {
                    self.selected_wishlist_item = (self.selected_wishlist_item + count - 1) % count;
                }
            }
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
            Screen::ProductDetail | Screen::Wishlist => {}
        }
    }

    pub fn back(&mut self) {
        if self.wishlist_form.is_some() {
            self.cancel_wishlist_form();
            return;
        }

        match self.screen {
            Screen::Brands => {}
            Screen::Categories => self.screen = Screen::Brands,
            Screen::Products => self.screen = Screen::Categories,
            Screen::ProductDetail => self.screen = Screen::Products,
            Screen::Wishlist => self.screen = self.previous_screen,
        }
    }

    pub fn open_wishlist(&mut self) {
        if self.screen != Screen::Wishlist {
            self.previous_screen = self.screen;
        }
        self.screen = Screen::Wishlist;
        self.clamp_wishlist_selection();
    }

    pub fn current_brand(&self) -> &'static catalog::Brand {
        catalog::brand(self.selected_brand)
    }

    pub fn current_category(&self) -> &'static catalog::Category {
        catalog::category(self.selected_category)
    }

    pub fn current_products(&self) -> Vec<&'static Product> {
        catalog::products_for(self.current_brand().id, self.current_category().id)
    }

    pub fn current_product(&self) -> Option<&'static Product> {
        self.current_products().get(self.selected_product).copied()
    }

    pub fn current_wishlist_item(&self) -> Option<&WishlistItem> {
        self.wishlist.get(self.selected_wishlist_item)
    }

    pub fn add_to_wishlist_from_catalog(&mut self) {
        let Some(product) = self.current_product() else {
            return;
        };

        let item = WishlistItem {
            name: product.name.to_string(),
            brand: self.current_brand().name.to_string(),
            category: self.current_category().name.to_string(),
            price: product.price.to_string(),
            fit: product.fit.to_string(),
            weight: product.weight.to_string(),
            sizes: product
                .sizes
                .iter()
                .map(|size| (*size).to_string())
                .collect(),
            in_stock: product.in_stock,
        };

        if self.wishlist.add(item) {
            self.selected_wishlist_item = self.wishlist.len().saturating_sub(1);
            self.status_message = Some("Added product to wishlist".to_string());
        } else {
            self.status_message = Some("Wishlist already contains that product".to_string());
        }
    }

    pub fn start_wishlist_form(&mut self) {
        self.wishlist_form = Some(WishlistFormState::new());
        self.status_message = None;
    }

    pub fn cancel_wishlist_form(&mut self) {
        self.wishlist_form = None;
        self.status_message = Some("Wishlist form cancelled".to_string());
    }

    pub fn input_char(&mut self, ch: char) {
        if let Some(form) = &mut self.wishlist_form {
            form.input.push(ch);
        }
    }

    pub fn pop_input_char(&mut self) {
        if let Some(form) = &mut self.wishlist_form {
            form.input.pop();
        }
    }

    pub fn submit_wishlist_field(&mut self) {
        let Some(form) = &mut self.wishlist_form else {
            return;
        };

        let field = form.field();
        let value = form.input.clone();
        field.parse_value(&value, &mut form.draft);

        if form.field_index + 1 < WishlistField::ALL.len() {
            form.field_index += 1;
            form.input = form.field().value_from(&form.draft);
            return;
        }

        let item = form.draft.clone().into_item();
        if item.name.trim().is_empty()
            || item.brand.trim().is_empty()
            || item.category.trim().is_empty()
        {
            self.status_message = Some("Name, brand, and category are required".to_string());
            return;
        }

        if self.wishlist.add(item) {
            self.selected_wishlist_item = self.wishlist.len().saturating_sub(1);
            self.wishlist_form = None;
            self.status_message = Some("Wishlist item saved".to_string());
        } else {
            self.status_message = Some("Wishlist already contains that item".to_string());
        }
    }

    pub fn delete_selected_wishlist_item(&mut self) {
        if self.wishlist.remove(self.selected_wishlist_item).is_some() {
            self.clamp_wishlist_selection();
            self.status_message = Some("Wishlist item deleted".to_string());
        }
    }

    pub fn persist_wishlist(&mut self) -> Result<(), String> {
        self.wishlist.save().map_err(|err| {
            let message = format!("Wishlist save failed: {err}");
            self.status_message = Some(message.clone());
            message
        })
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    fn clamp_wishlist_selection(&mut self) {
        if self.wishlist.is_empty() {
            self.selected_wishlist_item = 0;
        } else if self.selected_wishlist_item >= self.wishlist.len() {
            self.selected_wishlist_item = self.wishlist.len() - 1;
        }
    }
}
