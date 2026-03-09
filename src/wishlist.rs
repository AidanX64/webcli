use std::{
    fs, io,
    path::{Path, PathBuf},
};

const WISHLIST_FILE: &str = "wishlist.md";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WishlistItem {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: String,
    pub fit: String,
    pub weight: String,
    pub sizes: Vec<String>,
    pub in_stock: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Wishlist {
    items: Vec<WishlistItem>,
}

impl Wishlist {
    pub fn load() -> io::Result<Self> {
        let path = wishlist_path();
        if !path.exists() {
            let wishlist = Self::default_with_samples();
            wishlist.save()?;
            return Ok(wishlist);
        }

        let content = fs::read_to_string(&path)?;
        let items = parse_items(&content)?;
        Ok(Self { items })
    }

    pub fn save(&self) -> io::Result<()> {
        fs::write(wishlist_path(), self.to_markdown())
    }

    pub fn add(&mut self, item: WishlistItem) -> bool {
        if self
            .items
            .iter()
            .any(|existing| same_identity(existing, &item))
        {
            return false;
        }

        self.items.push(item);
        true
    }

    pub fn remove(&mut self, index: usize) -> Option<WishlistItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&WishlistItem> {
        self.items.get(index)
    }

    pub fn items(&self) -> &[WishlistItem] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn default_with_samples() -> Self {
        Self {
            items: vec![WishlistItem {
                name: "Gamma Hoodie".to_string(),
                brand: "Arc'teryx".to_string(),
                category: "Softshell".to_string(),
                price: "$350".to_string(),
                fit: "Regular fit".to_string(),
                weight: "545 g".to_string(),
                sizes: vec!["S".to_string(), "M".to_string(), "L".to_string()],
                in_stock: false,
            }],
        }
    }

    fn to_markdown(&self) -> String {
        if self.items.is_empty() {
            return "# Wishlist\n\n_No items yet._\n".to_string();
        }

        let mut output = String::from("# Wishlist\n");
        for item in &self.items {
            output.push('\n');
            output.push_str("## ");
            output.push_str(&item.name);
            output.push('\n');
            output.push_str(&format!("Brand: {}\n", item.brand));
            output.push_str(&format!("Category: {}\n", item.category));
            output.push_str(&format!("Price: {}\n", item.price));
            output.push_str(&format!("Fit: {}\n", item.fit));
            output.push_str(&format!("Weight: {}\n", item.weight));
            output.push_str(&format!("Sizes: {}\n", item.sizes.join(", ")));
            output.push_str(&format!(
                "In stock: {}\n",
                if item.in_stock { "yes" } else { "no" }
            ));
        }

        output
    }
}

fn wishlist_path() -> PathBuf {
    PathBuf::from(WISHLIST_FILE)
}

fn parse_items(content: &str) -> io::Result<Vec<WishlistItem>> {
    let mut items = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_fields = Vec::new();

    for line in content.lines() {
        if let Some(name) = line.strip_prefix("## ") {
            if let Some(name) = current_name.take() {
                items.push(parse_item(&name, &current_fields)?);
                current_fields.clear();
            }
            current_name = Some(name.trim().to_string());
            continue;
        }

        if current_name.is_some() && !line.trim().is_empty() {
            current_fields.push(line.to_string());
        }
    }

    if let Some(name) = current_name {
        items.push(parse_item(&name, &current_fields)?);
    }

    Ok(items)
}

fn parse_item(name: &str, fields: &[String]) -> io::Result<WishlistItem> {
    let brand = required_field(fields, "Brand")?;
    let category = required_field(fields, "Category")?;
    let price = required_field(fields, "Price")?;
    let fit = required_field(fields, "Fit")?;
    let weight = required_field(fields, "Weight")?;
    let sizes_value = required_field(fields, "Sizes")?;
    let in_stock_value = required_field(fields, "In stock")?;

    let sizes = sizes_value
        .split(',')
        .map(|size| size.trim())
        .filter(|size| !size.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    let in_stock = match in_stock_value.trim().to_ascii_lowercase().as_str() {
        "yes" | "true" => true,
        "no" | "false" => false,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid In stock value for wishlist item '{name}'"),
            ));
        }
    };

    Ok(WishlistItem {
        name: name.to_string(),
        brand,
        category,
        price,
        fit,
        weight,
        sizes,
        in_stock,
    })
}

fn required_field(fields: &[String], label: &str) -> io::Result<String> {
    let prefix = format!("{label}:");
    fields
        .iter()
        .find_map(|line| {
            line.strip_prefix(&prefix)
                .map(|value| value.trim().to_string())
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, missing_field_message(label)))
}

fn missing_field_message(label: &str) -> String {
    format!("missing field '{label}' in wishlist.md")
}

fn same_identity(left: &WishlistItem, right: &WishlistItem) -> bool {
    left.name == right.name && left.brand == right.brand && left.category == right.category
}

#[allow(dead_code)]
fn _wishlist_path_for_tests(path: &Path) -> PathBuf {
    path.join(WISHLIST_FILE)
}
