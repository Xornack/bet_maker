use eframe::egui;

// Native entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Bet Maker",
        options,
        Box::new(|_cc| Ok(Box::<BetMakerApp>::default())),
    )
}

// Web entry point
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let canvas = eframe::web_sys::window()
            .expect("No window")
            .document()
            .expect("No document")
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find element with id `the_canvas_id`")
            .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
            .expect("`the_canvas_id` was not a HtmlCanvasElement");

        let runner = eframe::WebRunner::new();
        runner.start(
            canvas,
            web_options,
            Box::new(|_cc| Ok(Box::<BetMakerApp>::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

struct BetMakerApp {
    prob_input: String,
    bet_amount_input: String,
    decimal_odds_input: String,
}

impl Default for BetMakerApp {
    fn default() -> Self {
        Self {
            prob_input: "50".to_owned(),
            bet_amount_input: "100".to_owned(),
            decimal_odds_input: "2.0".to_owned(),
        }
    }
}

impl eframe::App for BetMakerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Scale UI so it fills the available width on phones while staying
        // reasonable on desktop. Target a ~360 logical-pixel "design width".
        let avail_width = ui.ctx().content_rect().width();
        let zoom = (avail_width / 360.0).clamp(1.0, 2.5);
        if (ui.ctx().zoom_factor() - zoom).abs() > 0.01 {
            ui.ctx().set_zoom_factor(zoom);
        }

        ui.heading("Bet Maker");
        ui.separator();

        ui.collapsing("Probability to Odds & Payout", |ui| {
            ui.horizontal(|ui| {
                ui.label("Probability (%):");
                ui.text_edit_singleline(&mut self.prob_input);
            });

            ui.horizontal(|ui| {
                ui.label("Bet Amount ($):");
                ui.text_edit_singleline(&mut self.bet_amount_input);
            });

            let prob = self.prob_input.parse::<f64>().unwrap_or(0.0) / 100.0;
            let bet = self.bet_amount_input.parse::<f64>().unwrap_or(0.0);

            if prob > 0.0 && prob <= 1.0 {
                let decimal_odds = 1.0 / prob;
                let total_payout = bet * decimal_odds;
                let net_profit = total_payout - bet;

                ui.label(format!("Decimal Odds: {:.2}", decimal_odds));
                ui.label(format!("Potential Payout: ${:.2}", total_payout));
                ui.label(format!("Net Profit: ${:.2}", net_profit));
                ui.label(format!("Opponent must pay: ${:.2}", net_profit));
            } else {
                ui.colored_label(egui::Color32::RED, "Enter a probability between 0 and 100");
            }
        });

        ui.add_space(20.0);

        ui.collapsing("Odds to Implied Probability", |ui| {
            ui.horizontal(|ui| {
                ui.label("Decimal Odds:");
                ui.text_edit_singleline(&mut self.decimal_odds_input);
            });

            let odds = self.decimal_odds_input.parse::<f64>().unwrap_or(0.0);

            if odds >= 1.0 {
                let implied_prob = (1.0 / odds) * 100.0;
                ui.label(format!("Implied Probability: {:.2}%", implied_prob));
            } else {
                ui.colored_label(egui::Color32::RED, "Enter odds >= 1.0");
            }
        });
    }
}
