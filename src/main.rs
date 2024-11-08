use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ComboBoxText, Label, Orientation, FileChooserAction, FileChooserDialog};
use gtk::glib;
use poppler::PopplerDocument;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use futures::executor::block_on;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct ReadingState {
    current_index: usize,
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.speedreader")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    gtk::Window::set_default_icon_name("/assets/icon.ico");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Speed Reader")
        .default_width(600)
        .default_height(400)
        .build();

    let window_rc = Rc::new(RefCell::new(window));

    let vbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(20)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let word_label = Label::builder()
        .label("Começar")
        .use_markup(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .hexpand(true)  // Expandir horizontalmente para centralizar melhor
        .vexpand(true)  // Expandir verticalmente para melhorar o alinhamento
        .build();
    word_label.set_markup(&highlight_middle_letter("Começar"));
    word_label.set_margin_top(20);

    let backward_button = Button::builder()
        .label("\u{25C0}\u{25C0}") // Unicode para seta para esquerda.
        .build();
    backward_button.set_margin_top(10);
    backward_button.set_margin_bottom(10);
    backward_button.set_size_request(50, 50);

    let play_pause_button = Button::builder()
        .label("\u{25B6}")
        .build();
    play_pause_button.set_margin_top(10);
    play_pause_button.set_margin_bottom(10);
    play_pause_button.set_size_request(50, 50);

    let forward_button = Button::builder()
        .label("\u{25B6}\u{25B6}")
        .build();
    forward_button.set_margin_top(10);
    forward_button.set_margin_bottom(10);
    forward_button.set_size_request(50, 50);

    let controls_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(gtk::Align::Center)
        .build();

    controls_box.append(&backward_button);
    controls_box.append(&play_pause_button);
    controls_box.append(&forward_button);

    let time_remaining_label = Label::builder()
        .label("Tempo de leitura restante: 0s")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    time_remaining_label.set_text("Tempo de leitura restante: 0s");

    let speed_label = Label::builder()
        .label("bpm:")
        .build();
    speed_label.set_markup("<span size='20000'>bpm:</span>");

    let speed_combo = ComboBoxText::builder()
        .halign(gtk::Align::Center)
        .build();
    speed_combo.append(Some("250"), "250");
    speed_combo.append(Some("300"), "300");
    speed_combo.append(Some("350"), "350");
    speed_combo.append(Some("400"), "400");
    speed_combo.append(Some("450"), "450");
    speed_combo.append(Some("500"), "500");
    speed_combo.set_active(Some(1)); // Seleciona 300 como padrão.

    let speed_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();
    speed_box.append(&speed_label);
    speed_box.append(&speed_combo);

    // Input para carregar arquivo PDF.
    let open_pdf_button = Button::builder()
        .label("Selecionar PDF")
        .halign(gtk::Align::Center)
        .build();
    open_pdf_button.set_margin_top(20);

    let words = Arc::new(Mutex::new(Vec::new()));
    let current_index = Arc::new(Mutex::new(0));
    let is_playing = Arc::new(Mutex::new(false));
    let file_path = Arc::new(Mutex::new(None::<PathBuf>));

    let save_state = |file_path: &PathBuf, current_index: usize| {
        if let Some(file_name) = file_path.file_name() {
            let state = ReadingState { current_index };
            let state_json = serde_json::to_string(&state).unwrap();
            let state_file = format!("{}.json", file_name.to_string_lossy());
            fs::write(state_file, state_json).expect("Unable to write file");
        }
    };

    let update_time_remaining = |words: &Vec<String>, current_index: usize, bpm: u64, time_remaining_label: &Label| {
        let words_remaining = words.len().saturating_sub(current_index);
        let seconds_remaining = (words_remaining as f64 / (bpm as f64 / 60.0)).ceil() as u64;
        let hours = seconds_remaining / 3600;
        let minutes = (seconds_remaining % 3600) / 60;
        let seconds = seconds_remaining % 60;
        let time_string = if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        };
        time_remaining_label.set_text(&format!("Tempo de leitura restante: {}", time_string));
    };

    let window_clone = Rc::clone(&window_rc);
    let word_label_clone = word_label.clone();
    let words_clone = Arc::clone(&words);
    let current_index_clone = Arc::clone(&current_index);
    let file_path_clone = Arc::clone(&file_path);
    let time_remaining_label_clone = time_remaining_label.clone();
    let speed_combo_clone = speed_combo.clone();
    open_pdf_button.connect_clicked(move |_| {
        let window = window_clone.borrow();
        let dialog = FileChooserDialog::builder()
            .title("Selecione um arquivo PDF")
            .action(FileChooserAction::Open)
            .transient_for(&*window)
            .modal(true)
            .build();
        dialog.add_buttons(&[
            ("Cancelar", gtk::ResponseType::Cancel),
            ("Abrir", gtk::ResponseType::Accept),
        ]);

        let word_label_clone = word_label_clone.clone();
        let words_clone = Arc::clone(&words_clone);
        let current_index_clone = Arc::clone(&current_index_clone);
        let file_path_clone = Arc::clone(&file_path_clone);
        let time_remaining_label_clone = time_remaining_label_clone.clone();
        let speed_combo_clone = speed_combo_clone.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(file_path) = file.path() {
                        *file_path_clone.lock().unwrap() = Some(file_path.clone());

                        if let Some(file_name) = file_path.file_name() {
                            let state_file = format!("{}.json", file_name.to_string_lossy());
                            if let Ok(state_json) = fs::read_to_string(&state_file) {
                                if let Ok(state) = serde_json::from_str::<ReadingState>(&state_json) {
                                    *current_index_clone.lock().unwrap() = state.current_index;
                                }
                            }
                        }

                        let text = block_on(async move { extract_text_from_pdf(&file_path) });
                        if let Some(text) = text {
                            let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
                            {
                                let mut words_lock = words_clone.lock().unwrap();
                                *words_lock = words;
                            }
                            let index = *current_index_clone.lock().unwrap();
                            if let Some(word) = words_clone.lock().unwrap().get(index) {
                                word_label_clone.set_markup(&highlight_middle_letter(word));
                            }
                            let bpm: u64 = speed_combo_clone.active_text().unwrap().parse().unwrap();
                            update_time_remaining(&words_clone.lock().unwrap(), index, bpm, &time_remaining_label_clone);
                        }
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    });

    let words_clone = Arc::clone(&words);
    let current_index_clone = Arc::clone(&current_index);
    let is_playing_clone = Arc::clone(&is_playing);
    let word_label_clone = word_label.clone();
    let speed_combo_clone = speed_combo.clone();
    let play_pause_button_clone = play_pause_button.clone();
    let file_path_clone = Arc::clone(&file_path);
    let time_remaining_label_clone = time_remaining_label.clone();

    play_pause_button.connect_clicked(move |_| {
        let mut is_playing = is_playing_clone.lock().unwrap();
        if *is_playing {
            *is_playing = false;
            play_pause_button_clone.set_label("\u{25B6}");

            // Salva o estado atual.
            if let Some(ref file_path) = *file_path_clone.lock().unwrap() {
                let current_index = *current_index_clone.lock().unwrap();
                save_state(file_path, current_index);
            }
        } else {
            *is_playing = true;
            play_pause_button_clone.set_label("⏸");

            let words = Arc::clone(&words_clone);
            let current_index = Arc::clone(&current_index_clone);
            let word_label = word_label_clone.clone();
            let speed_combo = speed_combo_clone.clone();
            let is_playing = Arc::clone(&is_playing_clone);
            let play_pause_button_clone = play_pause_button_clone.clone();
            let time_remaining_label_clone = time_remaining_label_clone.clone();
            glib::MainContext::default().spawn_local(async move {
                while *is_playing.lock().unwrap() {
                    {
                        let mut index = current_index.lock().unwrap();
                        let words = words.lock().unwrap();
                        if *index < words.len() {
                            word_label.set_markup(&highlight_middle_letter(&words[*index]));
                            *index += 1;
                            // Atualiza o tempo de leitura restante
                            let bpm: u64 = speed_combo.active_text().unwrap().parse().unwrap();
                            update_time_remaining(&words, *index, bpm, &time_remaining_label_clone);
                        } else {
                            *is_playing.lock().unwrap() = false;
                            play_pause_button_clone.set_label("\u{25B6}");
                        }
                    }
                    let bpm: u64 = speed_combo.active_text().unwrap().parse().unwrap();
                    let interval = 60000 / bpm;
                    glib::timeout_future(Duration::from_millis(interval)).await;
                }
            });
        }
    });

    // Lógica para o botão de avançar.
    let words_clone = Arc::clone(&words);
    let current_index_clone = Arc::clone(&current_index);
    let word_label_clone = word_label.clone();
    let file_path_clone = Arc::clone(&file_path);
    let time_remaining_label_clone = time_remaining_label.clone();
    let speed_combo_clone = speed_combo.clone();
    forward_button.connect_clicked(move |_| {
        let mut index = current_index_clone.lock().unwrap();
        let words = words_clone.lock().unwrap();
        if *index < words.len() - 1 {
            *index += 1;
            word_label_clone.set_markup(&highlight_middle_letter(&words[*index]));
            // Salva o estado atual ao avançar manualmente.
            if let Some(ref file_path) = *file_path_clone.lock().unwrap() {
                save_state(file_path, *index);
            }
            // Atualiza o tempo de leitura restante
            let bpm: u64 = speed_combo_clone.active_text().unwrap().parse().unwrap();
            update_time_remaining(&words, *index, bpm, &time_remaining_label_clone);
        }
    });

    let words_clone = Arc::clone(&words);
    let current_index_clone = Arc::clone(&current_index);
    let word_label_clone = word_label.clone();
    let file_path_clone = Arc::clone(&file_path);
    let time_remaining_label_clone = time_remaining_label.clone();
    let speed_combo_clone = speed_combo.clone();
    backward_button.connect_clicked(move |_| {
        let mut index = current_index_clone.lock().unwrap();
        let words = words_clone.lock().unwrap();
        if *index > 0 {
            *index -= 1;
            word_label_clone.set_markup(&highlight_middle_letter(&words[*index]));
            // Salva o estado atual ao retroceder manualmente.
            if let Some(ref file_path) = *file_path_clone.lock().unwrap() {
                save_state(file_path, *index);
            }
            // Atualiza o tempo de leitura restante
            let bpm: u64 = speed_combo_clone.active_text().unwrap().parse().unwrap();
            update_time_remaining(&words, *index, bpm, &time_remaining_label_clone);
        }
    });

    // Adicionando todos os widgets no container principal (vbox).
    vbox.append(&word_label);
    vbox.append(&controls_box);
    vbox.append(&speed_box);
    vbox.append(&open_pdf_button);
    vbox.append(&time_remaining_label);

    let window = window_rc.borrow();
    window.set_child(Some(&vbox));

    window.present();
}

fn extract_text_from_pdf(file_path: &Path) -> Option<String> {
    if let Ok(document) = PopplerDocument::new_from_file(file_path.to_str()?, None) {
        let mut full_text = String::new();
        for i in 0..document.get_n_pages() {
            if let Some(page) = document.get_page(i) {
                if let Some(text) = page.get_text() {
                    full_text.push_str(&text);
                    full_text.push(' ');
                }
            }
        }
        return Some(full_text);
    }
    None
}

fn highlight_middle_letter(word: &str) -> String {
    let len = word.chars().count();
    if len == 0 {
        return String::new();
    }
    let middle_index = if len == 1 { 0 } else { (len - 1) / 2 }; // Índice da letra central
    let mut highlighted_word = String::new();

    for (i, c) in word.chars().enumerate() {
        if i == middle_index {
            highlighted_word.push_str(&format!("<span foreground='red'>{}</span>", c));
        } else {
            highlighted_word.push(c);
        }
    }

    format!("<span font='monospace' size='32000'>{}</span>", highlighted_word)
}
