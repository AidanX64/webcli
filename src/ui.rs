use ratatui::{
    Frame,
    layout::Margin,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph,
    },
};

use crate::{
    app::{App, Screen, WishlistField},
    catalog::{BRANDS, CATEGORIES},
};

const BG: Color = Color::Rgb(29, 27, 24);
const PANEL: Color = Color::Rgb(44, 40, 36);
const PANEL_ALT: Color = Color::Rgb(56, 51, 46);
const ACCENT: Color = Color::Rgb(214, 157, 89);
const ACCENT_SOFT: Color = Color::Rgb(133, 111, 86);
const TEXT: Color = Color::Rgb(232, 225, 214);
const MUTED: Color = Color::Rgb(171, 160, 145);
const ERROR: Color = Color::Rgb(198, 94, 77);

pub fn render(frame: &mut Frame, app: &App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(BG)),
        frame.area(),
    );

    let outer = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    let footer_height = if app.status_message.is_some() { 7 } else { 5 };
    let [header_area, body_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(12),
            Constraint::Length(footer_height),
        ])
        .areas(outer);

    frame.render_widget(
        header(app).block(panel_block("Main Page", PANEL_ALT)),
        header_area,
    );

    match app.screen {
        Screen::Brands => render_list(
            frame,
            body_area,
            "Brands",
            "Select a brand to view clothing categories.",
            &BRANDS.iter().map(|brand| brand.name).collect::<Vec<_>>(),
            app.selected_brand,
        ),
        Screen::Categories => render_list(
            frame,
            body_area,
            "Categories",
            &format!("{} catalog categories.", app.current_brand().name),
            &CATEGORIES
                .iter()
                .map(|category| category.name)
                .collect::<Vec<_>>(),
            app.selected_category,
        ),
        Screen::Products => render_products(frame, body_area, app),
        Screen::ProductDetail => render_product_detail(frame, body_area, app),
        Screen::Wishlist => render_wishlist(frame, body_area, app),
    }

    render_footer(frame, footer_area, app);

    if app.wishlist_form.is_some() {
        render_wishlist_input(frame, app);
    }
}

fn header(app: &App) -> Paragraph<'static> {
    let title = match app.screen {
        Screen::Brands => "Clothing Catalog".to_string(),
        Screen::Categories => app.current_brand().name.to_string(),
        Screen::Products => format!(
            "{} {}",
            app.current_brand().name,
            app.current_category().name
        ),
        Screen::ProductDetail => app
            .current_product()
            .map(|product| product.name.to_string())
            .unwrap_or_else(|| "Product Details".to_string()),
        Screen::Wishlist => "Wishlist".to_string(),
    };

    Paragraph::new(vec![
        Line::from("Mountain Outerwear").fg(ACCENT_SOFT),
        Line::from(title).bold(),
    ])
    .style(Style::default().fg(TEXT).bg(PANEL))
}

fn render_list(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    description: &str,
    items: &[&str],
    selected: usize,
) {
    let [description_area, list_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(9)])
        .areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(Line::from(description))
            .style(Style::default().fg(TEXT).bg(PANEL_ALT))
            .block(panel_block(title, PANEL)),
        description_area,
    );

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(Line::from(format!("  {item}"))))
        .collect();
    let list = List::new(list_items)
        .block(panel_block("Options", PANEL_ALT))
        .style(Style::default().fg(TEXT).bg(PANEL))
        .highlight_style(
            Style::default()
                .fg(BG)
                .bg(ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default().with_selected(Some(selected));
    frame.render_stateful_widget(list, list_area, &mut state);
}

fn render_products(frame: &mut Frame, area: Rect, app: &App) {
    let products = app.current_products();
    if products.is_empty() {
        render_empty_state(
            frame,
            area,
            "Products",
            &format!(
                "No {} items yet for {}.",
                app.current_category().name.to_lowercase(),
                app.current_brand().name
            ),
        );
        return;
    }

    let names: Vec<&str> = products.iter().map(|product| product.name).collect();
    render_list(
        frame,
        area,
        "Products",
        &format!(
            "{} {} lineup.",
            app.current_brand().name,
            app.current_category().name
        ),
        &names,
        app.selected_product,
    );
}

fn render_empty_state(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(message).bold(),
            Line::from("Pick another category or brand to continue browsing."),
        ])
        .style(Style::default().fg(TEXT).bg(PANEL_ALT))
        .block(panel_block(title, PANEL)),
        area,
    );
}

fn render_product_detail(frame: &mut Frame, area: Rect, app: &App) {
    let Some(product) = app.current_product() else {
        render_empty_state(frame, area, "Product Details", "No product selected.");
        return;
    };

    let [summary_area, spec_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(12)])
        .areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(product.name).bold(),
            Line::from(format!("Brand: {}", app.current_brand().name)),
            Line::from(format!("Category: {}", app.current_category().name)),
        ])
        .style(Style::default().fg(TEXT).bg(PANEL_ALT))
        .block(panel_block("Overview", PANEL)),
        summary_area,
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("Price: {}", product.price)),
            Line::from(format!("Fit: {}", product.fit)),
            Line::from(format!("Weight: {}", product.weight)),
            Line::from(format!("Sizes: {}", product.sizes.join(", "))),
            Line::from(format!(
                "Stock: {}",
                if product.in_stock {
                    "In stock"
                } else {
                    "Out of stock"
                }
            )),
        ])
        .style(Style::default().fg(TEXT).bg(PANEL))
        .block(panel_block("Specs", PANEL_ALT)),
        spec_area,
    );
}

fn render_wishlist(frame: &mut Frame, area: Rect, app: &App) {
    let [list_area, detail_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .areas(area);

    if app.wishlist.is_empty() {
        render_empty_state(
            frame,
            area,
            "Wishlist",
            "Wishlist is empty. Press n to create an item or add one from product details.",
        );
        return;
    }

    let items: Vec<ListItem> = app
        .wishlist
        .items()
        .iter()
        .map(|item| {
            ListItem::new(vec![
                Line::from(format!("  {}", item.name)).fg(TEXT),
                Line::from(format!("  {} / {}", item.brand, item.category)).fg(MUTED),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(panel_block("Wishlist", PANEL_ALT))
        .style(Style::default().fg(TEXT).bg(PANEL))
        .highlight_style(
            Style::default()
                .fg(BG)
                .bg(ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);

    let mut state = ListState::default().with_selected(Some(app.selected_wishlist_item));
    frame.render_stateful_widget(list, list_area, &mut state);
    render_wishlist_detail(frame, detail_area, app);
}

fn render_wishlist_detail(frame: &mut Frame, area: Rect, app: &App) {
    let Some(item) = app.current_wishlist_item() else {
        render_empty_state(
            frame,
            area,
            "Wishlist Details",
            "No wishlist item selected.",
        );
        return;
    };

    let [summary_area, spec_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(12)])
        .areas(area);

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(item.name.as_str()).bold(),
            Line::from(format!("Brand: {}", item.brand)),
            Line::from(format!("Category: {}", item.category)),
        ])
        .style(Style::default().fg(TEXT).bg(PANEL_ALT))
        .block(panel_block("Details", PANEL)),
        summary_area,
    );

    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("Price: {}", item.price)),
            Line::from(format!("Fit: {}", item.fit)),
            Line::from(format!("Weight: {}", item.weight)),
            Line::from(format!("Sizes: {}", item.sizes.join(", "))),
            Line::from(format!(
                "Stock: {}",
                if item.in_stock {
                    "In stock"
                } else {
                    "Out of stock"
                }
            )),
        ])
        .style(Style::default().fg(TEXT).bg(PANEL))
        .block(panel_block("Specs", PANEL_ALT)),
        spec_area,
    );
}

fn render_wishlist_input(frame: &mut Frame, app: &App) {
    let Some(form) = &app.wishlist_form else {
        return;
    };

    let popup_area = centered_rect(frame.area(), 72, 50);
    let field = form.field();
    let input_line = if form.input.is_empty() {
        ">".to_string()
    } else {
        format!("> {}", form.input)
    };
    let lines = vec![
        Line::from(format!(
            "Field {} of {}",
            form.field_index + 1,
            WishlistField::ALL.len()
        ))
        .fg(MUTED),
        Line::from(field.label()).fg(ACCENT).bold(),
        Line::from(field.hint()).fg(MUTED),
        Line::from(""),
        Line::from(input_line).fg(TEXT),
        Line::from(""),
        Line::from("Enter submits field  Esc cancels  Backspace edits").fg(MUTED),
    ];

    frame.render_widget(Clear, popup_area);
    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(TEXT).bg(PANEL))
            .block(panel_block("New Wishlist Item", PANEL_ALT)),
        popup_area,
    );
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = vec![Line::from(footer_text(app)).fg(TEXT)];
    if let Some(message) = &app.status_message {
        lines.push(Line::from(""));
        lines.push(Line::from(message.as_str()).fg(ERROR));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(TEXT).bg(PANEL_ALT))
            .block(panel_block("Keys", PANEL)),
        area,
    );
}

fn footer_text(app: &App) -> String {
    if app.wishlist_form.is_some() {
        return "Type to edit | Enter submit field | Esc cancel | Backspace delete".to_string();
    }

    match app.screen {
        Screen::Brands => "j/k or arrows move | Enter open brand | w wishlist | q quit".to_string(),
        Screen::Categories => {
            "j/k or arrows move | Enter open category | Esc back | w wishlist | q quit".to_string()
        }
        Screen::Products => {
            "j/k or arrows move | Enter view product | Esc back | w wishlist | q quit".to_string()
        }
        Screen::ProductDetail => {
            "a add to wishlist | Esc back to products | w wishlist | q quit".to_string()
        }
        Screen::Wishlist => {
            "j/k move | n new item | d/Delete remove | Esc back | w stay on wishlist | q quit"
                .to_string()
        }
    }
}

fn panel_block<'a>(title: &'a str, bg: Color) -> Block<'a> {
    Block::default()
        .title(Line::from(format!(" {title} ")).fg(ACCENT).bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(ACCENT_SOFT))
        .style(Style::default().bg(bg))
        .padding(Padding::uniform(1))
}

fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(height_percent)])
        .flex(Flex::Center)
        .areas::<1>(area)[0];
    Layout::horizontal([Constraint::Percentage(width_percent)])
        .flex(Flex::Center)
        .areas::<1>(vertical)[0]
}
