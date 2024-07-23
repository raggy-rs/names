use std::collections::{BTreeSet, HashMap};

use crate::names::{self, Info, NameEntry};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NamesApp {
    part: String,
    names: Vec<NameEntry>,
    selected: BTreeSet<String>,
    max_len: usize,
}

impl Default for NamesApp {
    fn default() -> Self {
        let mut data = std::io::Cursor::new(include_bytes!("../names.bin"));
        let mut names: Vec<NameEntry> = names::deserialize(&mut data).expect("could not parse name data").into_iter().map(|(name,info)|NameEntry::new(name, info)).collect();
        names.sort_by_key(|x|(!x.year_count.iter().skip(35).sum::<u32>(), x.name.to_owned()));
        Self {
            part: "".to_owned(),
            names, //: vec![NameEntry::new("Max".to_string(), Info{ sex: 1, year_count: [1;40]})],
            max_len: 4,
            selected: BTreeSet::new(),
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
        /*if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }*/

        Default::default()
    }
}

impl eframe::App for NamesApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
        });
        egui::SidePanel::right("right_side").show(ctx, |ui|{
            self.selected.retain(|name|!ui.button(name).clicked());
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Names");

            ui.horizontal(|ui| {
                ui.label("Contains: ");
                ui.text_edit_singleline(&mut self.part);
            });

            ui.add(egui::Slider::new(&mut self.max_len, 0..=20).text("max lenght"));

            ui.separator();
            //let re = regex::Regex::new(&self.part).unwrap();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in self.names.iter().filter(|x|x.name.len()<= self.max_len && x.sex==1 && x.name.contains(&self.part)){
                    if ui.button(format!("{} {}",entry.name, entry.total)).clicked(){
                        self.selected.insert(entry.name.clone());
                    }//, entry.year_count));
                }
            });
        });
    }
}

