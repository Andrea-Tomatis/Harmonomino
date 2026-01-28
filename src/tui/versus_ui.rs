use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::game::{FallingPiece, GamePhase};

use super::ui::{INFO_PANEL_WIDTH, render_board, tetromino_color};
use super::versus_app::VersusApp;

/// Main draw function for versus mode.
pub fn draw_versus(frame: &mut Frame, app: &VersusApp) {
    let area = frame.area();

    // Layout: [user board (fill)] [info panel (fixed)] [agent board (fill)]
    let [user_area, info_area, agent_area] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(INFO_PANEL_WIDTH + 2),
        Constraint::Fill(1),
    ])
    .split(area)[..] else {
        return;
    };

    // User board with current piece + ghost
    let ghost_cells = app.user_game.ghost_piece().map(FallingPiece::cells);
    let current_cells = app.user_game.current.map(|p| (p.cells(), p.tetromino));
    render_board(
        frame,
        &app.user_game.board,
        current_cells.as_ref(),
        ghost_cells.as_ref(),
        user_area,
        " USER ",
    );

    // Agent board (no falling piece)
    let agent_title = if app.agent_game_over {
        " AGENT (OVER) "
    } else {
        " AGENT "
    };
    render_board(frame, &app.agent_board, None, None, agent_area, agent_title);

    // Center info panel
    draw_versus_info(frame, app, info_area);

    // Overlays
    if app.user_game.phase == GamePhase::GameOver {
        draw_versus_game_over(frame, user_area);
    } else if app.paused {
        draw_versus_paused(frame, user_area);
    }
}

/// Draws the center info panel for versus mode.
fn draw_versus_info(frame: &mut Frame, app: &VersusApp, area: Rect) {
    let block = Block::default().borders(Borders::LEFT | Borders::RIGHT);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(6), // Next piece
        Constraint::Length(6), // Score
        Constraint::Length(5), // Lines
        Constraint::Min(10),   // Keys
    ])
    .split(inner);

    draw_next_piece(frame, app, chunks[0]);
    draw_scores(frame, app, chunks[1]);
    draw_lines(frame, app, chunks[2]);
    draw_versus_controls(frame, chunks[3]);
}

/// Draws the next piece preview.
fn draw_next_piece(frame: &mut Frame, app: &VersusApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Next ")
        .title_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let piece = FallingPiece::spawn(app.user_game.next);
    let cells = piece.cells();

    let min_col = cells.iter().map(|(c, _)| *c).min().unwrap_or(0);
    let max_col = cells.iter().map(|(c, _)| *c).max().unwrap_or(0);
    let min_row = cells.iter().map(|(_, r)| *r).min().unwrap_or(0);
    let max_row = cells.iter().map(|(_, r)| *r).max().unwrap_or(0);

    let color = tetromino_color(app.user_game.next);
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

/// Draws scores for both user and agent.
fn draw_scores(frame: &mut Frame, app: &VersusApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Score ")
        .title_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let user_score = app.user_game.rows_cleared * 100;
    let agent_score = app.agent_rows_cleared * 100;

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" U: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{user_score}"),
                Style::default().fg(Color::White).bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled(" A: ", Style::default().fg(Color::Magenta)),
            Span::styled(
                format!("{agent_score}"),
                Style::default().fg(Color::White).bold(),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Draws lines cleared for both user and agent.
fn draw_lines(frame: &mut Frame, app: &VersusApp, area: Rect) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(" Lines ")
        .title_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(" U: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{}", app.user_game.rows_cleared)),
        ]),
        Line::from(vec![
            Span::styled(" A: ", Style::default().fg(Color::Magenta)),
            Span::raw(format!("{}", app.agent_rows_cleared)),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Draws controls help for versus mode.
fn draw_versus_controls(frame: &mut Frame, area: Rect) {
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
            Span::raw(" Rotate CW"),
        ]),
        Line::from(vec![
            Span::styled("↑ Z", Style::default().fg(Color::Cyan)),
            Span::raw(" Rotate CCW"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⌫ ", Style::default().fg(Color::Yellow)),
            Span::raw("Sync agent"),
        ]),
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

/// Draws a game over overlay on the user board.
fn draw_versus_game_over(frame: &mut Frame, area: Rect) {
    let popup_area = center_popup(area, 24, 9);

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
fn draw_versus_paused(frame: &mut Frame, area: Rect) {
    let popup_area = center_popup(area, 20, 7);

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

/// Centers a popup rectangle within an area.
fn center_popup(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
