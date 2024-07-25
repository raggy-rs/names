use crate::names::{self, NameEntry, Rating};
use egui::Grid;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Deserialize, Serialize)]
enum RatingFilter {
    Any,
    Rating,
    NoRating,
    Is(Rating),
}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NamesApp {
    part: String,
    current: usize,
    names: Vec<NameEntry>,
    max_len: usize,
    rating_filter: RatingFilter,
}

impl Default for NamesApp {
    fn default() -> Self {
        let mut data = std::io::Cursor::new(include_bytes!("../names.bin"));
        let mut names: Vec<NameEntry> = names::deserialize(&mut data)
            .expect("could not parse name data")
            .into_iter()
            .map(|(name, info)| NameEntry::new(name, info))
            .collect();
        names.sort_by_key(|x| (!x.year_count.last().unwrap(), x.name.to_owned()));
        Self {
            part: "".to_owned(),
            names,
            current: 0,
            max_len: 4,
            rating_filter: RatingFilter::Any,
        }
    }
}

impl NamesApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
        /*let mut new = NamesApp::default();
        if let Some(storage) = cc.storage {
            for name in new.names.iter_mut() {
                if let Some(data) = storage.get_string(&name.name) {
                    if let Some((rating, comments)) = data.split_once('|') {
                        name.rating = match rating {
                            "Some(Good)" => Some(Rating::Good),
                            "Some(Bad)" => Some(Rating::Bad),
                            _ => None,
                        };
                        name.comments = comments.to_string();
                    }
                }
            }
        }
        new*/
    }
}

impl eframe::App for NamesApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        /*for entry in &self.names {
            if entry.rating.is_some() || !entry.comments.is_empty() {
                storage.set_string(
                    &entry.name,
                    format!("{:?}|{}", entry.rating, entry.comments),
                )
            }
        }
        */
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });*/
        egui::SidePanel::left("left_side").show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Names");

            ui.horizontal(|ui| {
                if ui.button("export").clicked() {
                    ctx.output_mut(|output: &mut egui::PlatformOutput| {
                        output.copied_text = ron::ser::to_string(
                            &self
                                .names
                                .iter()
                                .filter(|e| e.rating.is_some() || !e.comments.is_empty())
                                .collect::<Vec<_>>(),
                        )
                        .expect("failed to serialize")
                    });
                }
            });
            if ui.text_edit_singleline(&mut self.part).clicked() {
                ctx.output_mut(|x| {
                    x.mutable_text_under_cursor = true;
                });
            }

            ui.add(egui::Slider::new(&mut self.max_len, 0..=20));
            egui::ComboBox::from_label("Rating")
                .selected_text(match self.rating_filter {
                    RatingFilter::Is(Rating::Good) => "Good",
                    RatingFilter::Is(Rating::Bad) => "Bad",
                    RatingFilter::NoRating => "Not Rated",
                    RatingFilter::Rating => "Rated",
                    RatingFilter::Any => "Any",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.rating_filter,
                        RatingFilter::Is(Rating::Good),
                        "Good",
                    );
                    ui.selectable_value(
                        &mut self.rating_filter,
                        RatingFilter::Is(Rating::Bad),
                        "Bad",
                    );
                    ui.selectable_value(&mut self.rating_filter, RatingFilter::Any, "Any");
                    ui.selectable_value(&mut self.rating_filter, RatingFilter::Rating, "Rated");
                    ui.selectable_value(
                        &mut self.rating_filter,
                        RatingFilter::NoRating,
                        "Not Rated",
                    );
                });
            ui.separator();
            //let re = regex::Regex::new(&self.part).unwrap();
            egui::ScrollArea::vertical().show(ui, |ui| {
                Grid::new("names_grid")
                    .striped(true)
                    .min_col_width(15.0)
                    .show(ui, |ui| {
                        for (idx, entry) in self.names.iter_mut().enumerate().filter(|(_, x)| {
                            let rating_match = match self.rating_filter {
                                RatingFilter::Any => true,
                                RatingFilter::Rating => x.rating.is_some(),
                                RatingFilter::NoRating => x.rating.is_none(),
                                RatingFilter::Is(r) => x.rating == Some(r),
                            };

                            x.name.len() <= self.max_len
                                && x.sex == 1
                                && x.name.contains(&self.part)
                                && rating_match
                        }) {
                            /*egui::ComboBox::from_label(&entry.name)
                            .selected_text(match entry.rating {
                                Some(Rating::Good) => "+",
                                Some(Rating::Bad) => "-",
                                None => "",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut entry.rating, Some(Rating::Good), "+");
                                ui.selectable_value(&mut entry.rating, Some(Rating::Bad), "-");
                                ui.selectable_value(&mut entry.rating, None, "");
                            });*/

                            if ui
                                .link(&entry.name) //format!("{} {}", , entry.total))
                                .clicked()
                            {
                                self.current = idx;
                            } //, entry.year_count));
                            ui.label(entry.total.to_string());
                            let mut good_button = ui.button("+");
                            let mut bad_button = ui.button("-");
                            let mut unsure_button = ui.button("?");
                            match entry.rating {
                                Some(Rating::Good) => good_button = good_button.highlight(),
                                Some(Rating::Bad) => bad_button = bad_button.highlight(),
                                None => unsure_button = unsure_button.highlight(),
                            }
                            if good_button.clicked() {
                                entry.rating = Some(Rating::Good);
                            } //, entry.year_count));
                            if bad_button.clicked() {
                                entry.rating = Some(Rating::Bad);
                            }
                            if unsure_button.clicked() {
                                entry.rating = None;
                            }
                            ui.end_row();
                        }
                    });
            });
        });
        /*egui::SidePanel::right("right_side").show(ctx, |ui| {
            self.selected.retain(|name| !ui.button(name).clicked());
            if ui.button("save").clicked() {
                self.save(_frame.storage_mut().unwrap());
                /*self.storage
                .set_item("names", &format!("{:?}", self.selected))
                .unwrap();*/
            }
            /*if ui.button("load").clicked() {
                if let Ok(Some(names)) = self.storage.get_item("names") {
                    self.selected
                        .extend(names.split(", ").map(|x| x.trim_matches('"').to_string()));
                }
            }*/
        });*/

        egui::CentralPanel::default().show(ctx, |ui| {
            let name = &self.names[self.current].name;
            ui.heading(name);
            ui.hyperlink_to(
                "wikipedia",
                format!("https://de.wikipedia.org/wiki/{}_(Vorname)", name),
            );
            if let Ok([first, second]) = name.chars().next_chunk() {
                ui.hyperlink_to(
                    "baby-vornamen.de",
                    format!(
                        "https://www.baby-vornamen.de/Jungen/{0}/{0}{1}/{2}",
                        first, second, name
                    ),
                );
            }
            ui.label("Comments:");
            ui.text_edit_multiline(&mut self.names[self.current].comments);
        });
    }
}
