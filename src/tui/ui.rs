use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::game::{Board, FallingPiece, GamePhase, Tetromino};

use super::App;

/// Info panel width.
pub const INFO_PANEL_WIDTH: u16 = 20;

/// Minimum cell dimensions.
const MIN_CELL_WIDTH: u16 = 2;
const MIN_CELL_HEIGHT: u16 = 1;

/// Returns the color for a tetromino type.
pub const fn tetromino_color(tetromino: Tetromino) -> Color {
    match tetromino {
        Tetromino::I => Color::Cyan,
        Tetromino::O => Color::Yellow,
        Tetromino::T => Color::Magenta,
        Tetromino::S => Color::Green,
        Tetromino::Z => Color::Red,
        Tetromino::J => Color::Blue,
        Tetromino::L => Color::LightRed, // Orange-ish
    }
}

/// Calculates optimal cell dimensions to fit the board in the given area.
/// Returns `(cell_width, cell_height)` that maintains roughly square cells.
#[allow(clippy::cast_possible_truncation)]
fn calculate_cell_size(area: Rect) -> (u16, u16) {
    // Available space (subtract 2 for borders)
    let available_width = area.width.saturating_sub(2);
    let available_height = area.height.saturating_sub(2);

    // Calculate max cell size that fits
    let max_cell_width = available_width / Board::WIDTH as u16;
    let max_cell_height = available_height / Board::HEIGHT as u16;

    // Terminal chars are ~2x taller than wide, so ideal ratio is width = height * 2
    // Find the best fit that maintains aspect ratio
    let cell_width;
    let cell_height;

    if max_cell_width >= max_cell_height * 2 {
        // Height is the limiting factor
        cell_height = max_cell_height;
        cell_width = cell_height * 2;
    } else {
        // Width is the limiting factor
        cell_width = max_cell_width;
        cell_height = cell_width.div_ceil(2); // Round up for better appearance
    }

    (
        cell_width.max(MIN_CELL_WIDTH),
        cell_height.max(MIN_CELL_HEIGHT),
    )
}

/// Main draw function for the TUI.
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Main layout: game area (fill) | info panel (right)
    let [game_area, info_area] =
        Layout::horizontal([Constraint::Min(24), Constraint::Length(INFO_PANEL_WIDTH)]).split(area)
            [..]
    else {
        return;
    };

    draw_board(frame, app, game_area);
    draw_info_panel(frame, app, info_area);

    // Draw overlays for game over or pause
    if app.game.phase == GamePhase::GameOver {
        draw_game_over(frame, game_area);
    } else if app.paused {
        draw_paused(frame, game_area);
    }
}

/// Draws the main game board, scaled to fit the area.
fn draw_board(frame: &mut Frame, app: &App, area: Rect) {
    let ghost_cells = app.game.ghost_piece().map(FallingPiece::cells);
    let current_cells = app.game.current.map(|p| (p.cells(), p.tetromino));

    render_board(
        frame,
        &app.game.board,
        current_cells.as_ref(),
        ghost_cells.as_ref(),
        area,
        " TETRIS ",
    );
}

/// Renders a board with optional current and ghost pieces into the given area.
#[allow(clippy::cast_possible_truncation)]
pub fn render_board(
    frame: &mut Frame,
    board: &Board,
    current: Option<&([(i8, i8); 4], Tetromino)>,
    ghost: Option<&[(i8, i8); 4]>,
    area: Rect,
    title: &str,
) {
    let (cell_width, cell_height) = calculate_cell_size(area);

    // Calculate actual board dimensions
    let board_width = Board::WIDTH as u16 * cell_width + 2;
    let board_height = Board::HEIGHT as u16 * cell_height + 2;

    // Center the board
    let centered = center_rect(area, board_width, board_height);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(title);

    let inner = block.inner(centered);
    frame.render_widget(block, centered);

    // Build the display line by line
    let mut lines: Vec<Line> = Vec::with_capacity(Board::HEIGHT * cell_height as usize);

    for display_row in 0..Board::HEIGHT {
        let board_row = Board::HEIGHT - 1 - display_row;

        // Generate cell_height lines for this row
        for _line_in_cell in 0..cell_height {
            let mut spans: Vec<Span> = Vec::with_capacity(Board::WIDTH);

            for col in 0..Board::WIDTH {
                let (cell_type, color) = get_cell_appearance(board, col, board_row, current, ghost);

                let cell_text = render_cell(cell_type, cell_width);
                spans.push(styled_span(cell_text, cell_type, color));
            }

            lines.push(Line::from(spans));
        }
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Creates a styled span for a cell.
fn styled_span(text: String, cell_type: CellType, color: Option<Color>) -> Span<'static> {
    match cell_type {
        CellType::Empty => Span::raw(text),
        CellType::Filled => {
            let c = color.unwrap_or(Color::White);
            Span::styled(text, Style::default().fg(c))
        }
        CellType::Ghost => {
            let c = color.unwrap_or(Color::DarkGray);
            Span::styled(text, Style::default().fg(c))
        }
    }
}

/// Centers a rectangle of given size within an area.
fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let [centered] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(area)[..]
    else {
        return area;
    };

    let [centered] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(centered)[..]
    else {
        return area;
    };

    centered
}

/// Determines what to display for a cell.
#[allow(clippy::cast_possible_truncation)]
fn get_cell_appearance(
    board: &Board,
    col: usize,
    board_row: usize,
    current_cells: Option<&([(i8, i8); 4], Tetromino)>,
    ghost_cells: Option<&[(i8, i8); 4]>,
) -> (CellType, Option<Color>) {
    if board[board_row][col] {
        (CellType::Filled, Some(Color::Gray))
    } else if let Some((cells, tetromino)) = current_cells {
        if cells.contains(&(col as i8, board_row as i8)) {
            (CellType::Filled, Some(tetromino_color(*tetromino)))
        } else if ghost_cells.is_some_and(|g| g.contains(&(col as i8, board_row as i8))) {
            (CellType::Ghost, Some(Color::DarkGray))
        } else {
            (CellType::Empty, None)
        }
    } else if ghost_cells.is_some_and(|g| g.contains(&(col as i8, board_row as i8))) {
        (CellType::Ghost, Some(Color::DarkGray))
    } else {
        (CellType::Empty, None)
    }
}

#[derive(Clone, Copy)]
enum CellType {
    Empty,
    Filled,
    Ghost,
}

/// Renders a cell using block characters.
fn render_cell(cell_type: CellType, width: u16) -> String {
    match cell_type {
        CellType::Empty => " ".repeat(width as usize),
        CellType::Filled => "█".repeat(width as usize),
        CellType::Ghost => "░".repeat(width as usize),
    }
}

/// Draws the info panel.
fn draw_info_panel(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::LEFT);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(6),
        Constraint::Length(4),
        Constraint::Length(3),
        Constraint::Min(10),
    ])
    .split(inner);

    draw_next_piece(frame, app, chunks[0]);
    draw_score(frame, app, chunks[1]);
    draw_lines(frame, app, chunks[2]);
    draw_controls(frame, chunks[3]);
}

/// Draws the next piece preview using block characters.
fn draw_next_piece(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Next ")
        .title_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let piece = FallingPiece::spawn(app.game.next);
    let cells = piece.cells();

    // NOTE: duplicate logic with board.rs/visualize_cells; could refactor?
    let min_col = cells.iter().map(|(c, _)| *c).min().unwrap_or(0);
    let max_col = cells.iter().map(|(c, _)| *c).max().unwrap_or(0);
    let min_row = cells.iter().map(|(_, r)| *r).min().unwrap_or(0);
    let max_row = cells.iter().map(|(_, r)| *r).max().unwrap_or(0);

    let color = tetromino_color(app.game.next);
    let mut lines: Vec<Line> = Vec::new();

    for row in (min_row..=max_row).rev() {
        let mut spans: Vec<Span> = Vec::new();
        for col in min_col..=max_col {
            if cells.contains(&(col, row)) {
                spans.push(Span::styled("██", Style::default().fg(color)));
            } else {
                spans.push(Span::raw("  "));
            }
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).centered();
    frame.render_widget(paragraph, inner);
}

/// Draws the score display.
fn draw_score(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Score ")
        .title_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let score = app.game.rows_cleared * 100;
    let paragraph = Paragraph::new(format!("{score}"))
        .centered()
        .style(Style::default().fg(Color::White).bold());
    frame.render_widget(paragraph, inner);
}

/// Draws lines cleared count.
fn draw_lines(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Lines ")
        .title_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let paragraph = Paragraph::new(format!("{}", app.game.rows_cleared))
        .centered()
        .style(Style::default().fg(Color::White));
    frame.render_widget(paragraph, inner);
}

/// Draws the controls help.
fn draw_controls(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Keys ")
        .title_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let controls = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("← → ", Style::default().fg(Color::Cyan)),
            Span::raw("Move"),
        ]),
        Line::from(vec![
            Span::styled("↓   ", Style::default().fg(Color::Cyan)),
            Span::raw("Soft"),
        ]),
        Line::from(vec![
            Span::styled("SPC ", Style::default().fg(Color::Cyan)),
            Span::raw("Drop"),
        ]),
        Line::from(vec![
            Span::styled("↑ X", Style::default().fg(Color::Cyan)),
            Span::raw("Rotate CW"),
        ]),
        Line::from(vec![
            Span::styled("↑ Z", Style::default().fg(Color::Cyan)),
            Span::raw("Rotate CCW"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("P ", Style::default().fg(Color::Yellow)),
            Span::raw("Pause"),
        ]),
        Line::from(vec![
            Span::styled("R ", Style::default().fg(Color::Green)),
            Span::raw("Restart"),
        ]),
        Line::from(vec![
            Span::styled("Q ", Style::default().fg(Color::Red)),
            Span::raw("Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(controls);
    frame.render_widget(paragraph, inner);
}

/// Draws a game over overlay.
fn draw_game_over(frame: &mut Frame, area: Rect) {
    let popup_area = center_rect(area, 24, 9);

    let bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(bg, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .title(" Game Over ");

    let text = vec![
        Line::from(""),
        Line::from("GAME OVER".bold().red()),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("R", Style::default().fg(Color::Green)),
            Span::raw(" Restart"),
        ]),
        Line::from(vec![
            Span::styled("Q", Style::default().fg(Color::Red)),
            Span::raw(" Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(text).centered().block(block);
    frame.render_widget(paragraph, popup_area);
}

/// Draws a paused overlay.
fn draw_paused(frame: &mut Frame, area: Rect) {
    let popup_area = center_rect(area, 20, 7);

    let bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(bg, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(" Paused ");

    let text = vec![
        Line::from(""),
        Line::from("PAUSED".bold().yellow()),
        Line::from(""),
        Line::from(vec![
            Span::styled("P", Style::default().fg(Color::Yellow)),
            Span::raw(" Resume"),
        ]),
    ];

    let paragraph = Paragraph::new(text).centered().block(block);
    frame.render_widget(paragraph, popup_area);
}
