#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrandId {
    NorthFace,
    Arcteryx,
    Colombia,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CategoryId {
    RainShell,
    MidLayer,
    BaseLayer,
}

#[derive(Clone, Copy, Debug)]
pub struct Brand {
    pub id: BrandId,
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Category {
    pub id: CategoryId,
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Product {
    pub name: &'static str,
    pub brand: BrandId,
    pub category: CategoryId,
    pub price: &'static str,
    pub fit: &'static str,
    pub weight: &'static str,
    pub sizes: &'static [&'static str],
    pub in_stock: bool,
}

pub const BRANDS: [Brand; 3] = [
    Brand {
        id: BrandId::NorthFace,
        name: "North Face",
    },
    Brand {
        id: BrandId::Arcteryx,
        name: "Arc'teryx",
    },
    Brand {
        id: BrandId::Colombia,
        name: "Colombia",
    },
];

pub const CATEGORIES: [Category; 3] = [
    Category {
        id: CategoryId::RainShell,
        name: "Rain Shell",
    },
    Category {
        id: CategoryId::MidLayer,
        name: "Mid Layer",
    },
    Category {
        id: CategoryId::BaseLayer,
        name: "Base Layer",
    },
];

const ALPHA_SV_SIZES: [&str; 4] = ["S", "M", "L", "XL"];
const BETA_SV_SIZES: [&str; 3] = ["M", "L", "XL"];
const APLHA_SL_SIZES: [&str; 3] = ["S", "M", "L"];
const BETA_AR_SIZES: [&str; 4] = ["S", "M", "L", "XL"];

pub const PRODUCTS: [Product; 4] = [
    Product {
        name: "Alpha SV",
        brand: BrandId::Arcteryx,
        category: CategoryId::RainShell,
        price: "$900",
        fit: "Regular fit",
        weight: "485 g",
        sizes: &ALPHA_SV_SIZES,
        in_stock: true,
    },
    Product {
        name: "Beta SV",
        brand: BrandId::Arcteryx,
        category: CategoryId::RainShell,
        price: "$800",
        fit: "Regular fit",
        weight: "470 g",
        sizes: &BETA_SV_SIZES,
        in_stock: true,
    },
    Product {
        name: "Aplha SL",
        brand: BrandId::Arcteryx,
        category: CategoryId::RainShell,
        price: "$500",
        fit: "Trim fit",
        weight: "300 g",
        sizes: &APLHA_SL_SIZES,
        in_stock: false,
    },
    Product {
        name: "Beta AR",
        brand: BrandId::Arcteryx,
        category: CategoryId::RainShell,
        price: "$650",
        fit: "Regular fit",
        weight: "461 g",
        sizes: &BETA_AR_SIZES,
        in_stock: true,
    },
];

pub fn brand(index: usize) -> &'static Brand {
    &BRANDS[index]
}

pub fn category(index: usize) -> &'static Category {
    &CATEGORIES[index]
}

pub fn products_for(brand: BrandId, category: CategoryId) -> Vec<&'static Product> {
    PRODUCTS
        .iter()
        .filter(|product| product.brand == brand && product.category == category)
        .collect()
}
