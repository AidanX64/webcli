use ratatui::{
    Frame,
    layout::Margin,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph,
    },
};

use crate::{
    app::{App, Screen},
    catalog::{BRANDS, CATEGORIES},
};

const BG: Color = Color::Rgb(236, 222, 198);
const PANEL: Color = Color::Rgb(206, 168, 122);
const PANEL_ALT: Color = Color::Rgb(188, 145, 96);
const ACCENT: Color = Color::Rgb(176, 88, 28);
const ACCENT_SOFT: Color = Color::Rgb(142, 83, 43);
const TEXT: Color = Color::Rgb(67, 42, 25);

pub fn render(frame: &mut Frame, app: &App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(BG)),
        frame.area(),
    );

    let outer = frame.area().inner(Margin {
        vertical: 1,
        horizontal: 2,
    });

    let [header_area, body_area, footer_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(12),
            Constraint::Length(5),
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
    }

    frame.render_widget(
        Paragraph::new(footer_text(app.screen))
            .style(Style::default().fg(TEXT).bg(PANEL_ALT))
            .block(panel_block("Keys", PANEL)),
        footer_area,
    );
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

fn footer_text(screen: Screen) -> &'static str {
    match screen {
        Screen::Brands => "j/k or arrows move | Enter open brand | q quit",
        Screen::Categories => "j/k or arrows move | Enter open category | Esc back | q quit",
        Screen::Products => "j/k or arrows move | Enter view product | Esc back | q quit",
        Screen::ProductDetail => "Esc back to products | q quit",
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
